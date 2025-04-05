use std::{error::Error, fmt::Display};
use std::{net::Ipv6Addr, str::FromStr};

use crate::eui::{EUI48, EUI64, EUIConvertError};

#[derive(Debug)]
pub enum IPv6ParseError {
    InvalidFormat(String),
}

impl Error for IPv6ParseError {}

impl Display for IPv6ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFormat(value) => {
                write!(f, "IPv6 address format is incorrect. {}", value)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IPv6(Ipv6Addr);

impl IPv6 {
    pub fn parse<T>(source: T) -> Result<IPv6, IPv6ParseError>
    where
        T: Into<String>,
    {
        let source: String = source.into();

        match Ipv6Addr::from_str(&source) {
            Ok(addr) => Ok(Self(addr)),
            Err(e) => Err(IPv6ParseError::InvalidFormat(e.to_string())),
        }
    }

    pub fn is_unicast_link_local(&self) -> bool {
        self.0.is_unicast_link_local()
    }

    pub fn segments(&self) -> [u16; 8] {
        self.0.segments()
    }

    pub fn to_mac_address(self) -> Result<EUI48, EUIConvertError> {
        let eui48: EUI48 = self.try_into()?;

        Ok(eui48)
    }
}

impl Display for IPv6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl FromStr for IPv6 {
    type Err = IPv6ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl From<EUI48> for IPv6 {
    fn from(value: EUI48) -> Self {
        let eui64: EUI64 = value.into();
        eui64.into()
    }
}

impl From<EUI64> for IPv6 {
    fn from(value: EUI64) -> Self {
        let segments = value.segments();
        let e = (segments[0] as u16 ^ 0x02) << 8 | segments[1] as u16;
        let f = (segments[2] as u16) << 8 | (segments[3] as u16);
        let g = (segments[4] as u16) << 8 | (segments[5] as u16);
        let h = (segments[6] as u16) << 8 | (segments[7] as u16);

        let addr = Ipv6Addr::new(0xfe80, 0, 0, 0, e, f, g, h);

        Self(addr)
    }
}

#[cfg(test)]
mod tests {
    use crate::eui::EUI48;

    use super::IPv6;

    #[test]
    fn test_eui48_to_ipv6() {
        let eui48: EUI48 = "01-00-5E-90-10-FF".parse().unwrap();
        let ipv6: IPv6 = eui48.try_into().unwrap();
        assert_eq!("fe80::300:5eff:fe90:10ff", ipv6.to_string())
    }
}
