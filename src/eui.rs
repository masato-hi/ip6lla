use std::{error::Error, fmt::Display, str::FromStr};

use crate::ipv6::IPv6;

#[derive(Debug)]
pub enum EUIParseError {
    InvalidLength(String),
    InvalidChar(String),
    InvalidFormat(String),
    Overflow(String),
}

impl Error for EUIParseError {}

impl Display for EUIParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLength(value) => write!(f, "Invalid MAC Address length. {}", value),
            Self::InvalidChar(value) => {
                write!(f, "MAC address contains invalid characters. {}", value)
            }
            Self::InvalidFormat(value) => {
                write!(f, "MAC address format is incorrect. {}", value)
            }
            Self::Overflow(value) => {
                write!(f, "MAC address field value is too large. {}", value)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum EUIConvertError {
    InvalidExtensionIdentifer(EUI64),
    InvalidInterfaceIdentifer(IPv6),
    NotUnicastLinkLocal(IPv6),
}

impl Error for EUIConvertError {}

impl Display for EUIConvertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EUIConvertError::InvalidExtensionIdentifer(eui64) => {
                write!(f, "Invalid extension identifer. {}", eui64)
            }
            EUIConvertError::InvalidInterfaceIdentifer(ipv6) => {
                write!(f, "Invalid interface identifer. {}", ipv6)
            }
            EUIConvertError::NotUnicastLinkLocal(ipv6) => {
                write!(f, "Not unicast link local address. {}", ipv6)
            }
        }
    }
}

fn parse_eui(source: &str) -> Result<Vec<u8>, EUIParseError> {
    const DIGIT_RADIX: u32 = 16;
    const SEGMENT_MAX: u32 = 255;

    let source: String = source.into();

    if source.len() % 3 != 2 {
        return Err(EUIParseError::InvalidLength(source));
    }

    let mut segments = Vec::with_capacity(8);

    let mut segment: u32 = 0;
    for (i, char) in source.chars().enumerate() {
        match char.to_digit(DIGIT_RADIX) {
            Some(digit) => {
                segment = (segment << 4) + digit;
            }
            None if char == ':' || char == '-' => {
                if (i + 1) % 3 != 0 {
                    return Err(EUIParseError::InvalidFormat(source));
                }
                if segment > SEGMENT_MAX {
                    return Err(EUIParseError::Overflow(source));
                }

                segments.push(segment as u8);
                segment = 0;
            }
            _ => return Err(EUIParseError::InvalidChar(source)),
        }
    }

    if segment > SEGMENT_MAX {
        return Err(EUIParseError::Overflow(source));
    }

    segments.push(segment as u8);

    Ok(segments)
}

const EXTENSION_IDENTIFIER_PREFIX_HIGH: u8 = 0xff;
const EXTENSION_IDENTIFIER_PREFIX_LOW: u8 = 0xfe;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EUI48 {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
}

impl EUI48 {
    pub fn parse<T>(source: T) -> Result<Self, EUIParseError>
    where
        T: Into<String>,
    {
        const SEGMENTS_NUM: usize = 6;

        let source: String = source.into();
        let segments = parse_eui(source.as_str())?;

        if segments.len() != SEGMENTS_NUM {
            return Err(EUIParseError::InvalidLength(source));
        }

        let value = Self {
            a: segments[0],
            b: segments[1],
            c: segments[2],
            d: segments[3],
            e: segments[4],
            f: segments[5],
        };

        Ok(value)
    }

    pub fn hyphenated(&self) -> String {
        format!(
            "{:02x}-{:02x}-{:02x}-{:02x}-{:02x}-{:02x}",
            self.a, self.b, self.c, self.d, self.e, self.f
        )
    }

    pub fn colon_separated(&self) -> String {
        format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.a, self.b, self.c, self.d, self.e, self.f
        )
    }

    pub fn to_ipv6(self) -> IPv6 {
        self.into()
    }
}

impl Display for EUI48 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hyphenated())
    }
}

impl TryFrom<String> for EUI48 {
    type Error = EUIParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<&str> for EUI48 {
    type Error = EUIParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl FromStr for EUI48 {
    type Err = EUIParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl TryFrom<EUI64> for EUI48 {
    type Error = EUIConvertError;
    fn try_from(value: EUI64) -> Result<Self, Self::Error> {
        if value.d != EXTENSION_IDENTIFIER_PREFIX_HIGH {
            return Err(EUIConvertError::InvalidExtensionIdentifer(value));
        }

        if value.e != EXTENSION_IDENTIFIER_PREFIX_LOW {
            return Err(EUIConvertError::InvalidExtensionIdentifer(value));
        }

        Ok(Self {
            a: value.a,
            b: value.b,
            c: value.c,
            d: value.f,
            e: value.g,
            f: value.h,
        })
    }
}

impl TryFrom<IPv6> for EUI48 {
    type Error = EUIConvertError;
    fn try_from(value: IPv6) -> Result<Self, Self::Error> {
        let eui64: EUI64 = value.try_into()?;
        let eui48: EUI48 = eui64.try_into()?;
        Ok(eui48)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EUI64 {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    g: u8,
    h: u8,
}

impl EUI64 {
    pub fn parse<T>(source: T) -> Result<Self, EUIParseError>
    where
        T: Into<String>,
    {
        const SEGMENTS_NUM: usize = 8;

        let source: String = source.into();
        let segments = parse_eui(source.as_str())?;

        if segments.len() != SEGMENTS_NUM {
            return Err(EUIParseError::InvalidLength(source));
        }

        let value = Self {
            a: segments[0],
            b: segments[1],
            c: segments[2],
            d: segments[3],
            e: segments[4],
            f: segments[5],
            g: segments[6],
            h: segments[7],
        };

        Ok(value)
    }

    pub fn segments(&self) -> [u8; 8] {
        [
            self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h,
        ]
    }

    pub fn hyphenated(&self) -> String {
        format!(
            "{:02x}-{:02x}-{:02x}-{:02x}-{:02x}-{:02x}-{:02x}-{:02x}",
            self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h
        )
    }
}

impl Display for EUI64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hyphenated())
    }
}

impl FromStr for EUI64 {
    type Err = EUIParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl From<EUI48> for EUI64 {
    fn from(value: EUI48) -> Self {
        Self {
            a: value.a,
            b: value.b,
            c: value.c,
            d: EXTENSION_IDENTIFIER_PREFIX_HIGH,
            e: EXTENSION_IDENTIFIER_PREFIX_LOW,
            f: value.d,
            g: value.e,
            h: value.f,
        }
    }
}

impl TryFrom<IPv6> for EUI64 {
    type Error = EUIConvertError;
    fn try_from(value: IPv6) -> Result<Self, Self::Error> {
        if !value.is_unicast_link_local() {
            return Err(EUIConvertError::NotUnicastLinkLocal(value));
        }

        let segments = value.segments();

        let a = ((segments[4] >> 8) as u8) ^ 2;
        let b = segments[4] as u8;
        let c = (segments[5] >> 8) as u8;
        let d = segments[5] as u8;
        let e = (segments[6] >> 8) as u8;
        let f = segments[6] as u8;
        let g = (segments[7] >> 8) as u8;
        let h = segments[7] as u8;

        if d != EXTENSION_IDENTIFIER_PREFIX_HIGH {
            return Err(EUIConvertError::InvalidInterfaceIdentifer(value));
        }

        if e != EXTENSION_IDENTIFIER_PREFIX_LOW {
            return Err(EUIConvertError::InvalidInterfaceIdentifer(value));
        }

        Ok(Self {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            h,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{eui::EUI64, ipv6::IPv6};

    use super::EUI48;

    #[test]
    fn test_eui48_parse() {
        // colon-separated upper
        let value = EUI48::parse("01:00:5E:90:10:FF").unwrap();
        assert_eq!("01-00-5E-90-10-FF", value.to_string());

        // colon-separated lower
        let value = EUI48::parse("01:00:5e:90:10:ff").unwrap();
        assert_eq!("01-00-5E-90-10-FF", value.to_string());

        // hyphenated upper
        let value = EUI48::parse("01-00-5E-90-10-FF").unwrap();
        assert_eq!("01-00-5E-90-10-FF", value.to_string());

        // hyphenated lower
        let value = EUI48::parse("01-00-5e-90-10-ff").unwrap();
        assert_eq!("01-00-5E-90-10-FF", value.to_string());

        // invalid length
        assert_eq!(
            "Invalid MAC Address length. 01-00-5E-90-10-F",
            EUI48::parse("01-00-5E-90-10-F").unwrap_err().to_string()
        );

        // invalid length
        assert_eq!(
            "Invalid MAC Address length. 01-00-5E-90-10-FFF",
            EUI48::parse("01-00-5E-90-10-FFF").unwrap_err().to_string()
        );

        // invalid character
        assert_eq!(
            "MAC address contains invalid characters. 01-00-5E-90-10-FG",
            EUI48::parse("01-00-5E-90-10-FG").unwrap_err().to_string()
        );

        // invalid separator
        assert_eq!(
            "MAC address contains invalid characters. 01-00-5E-90-10/FF",
            EUI48::parse("01-00-5E-90-10/FF").unwrap_err().to_string()
        );
    }

    #[test]
    fn test_eui48_hyphenated() {
        let value = EUI48::parse("01-00-5E-90-10-FF").unwrap();
        assert_eq!("01-00-5E-90-10-FF", value.hyphenated());
    }

    #[test]
    fn test_eui48_colon_separated() {
        let value = EUI48::parse("01-00-5E-90-10-FF").unwrap();
        assert_eq!("01:00:5E:90:10:FF", value.colon_separated());
    }

    #[test]
    fn test_eui64_from_eui48() {
        let value: EUI64 = EUI48::parse("01-00-5E-90-10-FF").unwrap().into();
        assert_eq!("01-00-5E-FF-FE-90-10-FF", value.to_string());
    }

    #[test]
    fn test_eui64_to_eui48() {
        let eui64: EUI64 = "01-00-5E-FF-FE-90-10-FF".parse().unwrap();
        let eui48: EUI48 = eui64.try_into().unwrap();
        assert_eq!("01-00-5E-90-10-FF", eui48.to_string());
    }

    #[test]
    fn test_eui64_segments() {
        let value: EUI64 = EUI48::parse("01-00-5E-90-10-FF").unwrap().into();
        assert_eq!([1, 0, 94, 255, 254, 144, 16, 255], value.segments());
    }

    #[test]
    fn test_ipv6_to_eui64() {
        let ipv6: IPv6 = "fe80::300:5eff:fe90:10ff".parse().unwrap();
        let value: EUI64 = ipv6.try_into().unwrap();
        assert_eq!("01-00-5E-FF-FE-90-10-FF", value.to_string());
    }

    #[test]
    fn test_ipv6_to_eui48() {
        let ipv6: IPv6 = "fe80::300:5eff:fe90:10ff".parse().unwrap();
        let value: EUI48 = ipv6.try_into().unwrap();
        assert_eq!("01-00-5E-90-10-FF", value.to_string());
    }
}
