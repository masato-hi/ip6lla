use std::{error::Error, fmt::Display, process::exit};

use clap::Parser;

use crate::{
    eui::{EUI48, EUIConvertError, EUIParseError},
    ipv6::{IPv6, IPv6ParseError},
};

#[derive(Debug)]
pub struct ExecutionError(String);

impl Error for ExecutionError {}

impl Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<IPv6ParseError> for ExecutionError {
    fn from(value: IPv6ParseError) -> Self {
        Self(value.to_string())
    }
}

impl From<EUIParseError> for ExecutionError {
    fn from(value: EUIParseError) -> Self {
        Self(value.to_string())
    }
}

impl From<EUIConvertError> for ExecutionError {
    fn from(value: EUIConvertError) -> Self {
        Self(value.to_string())
    }
}

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
pub struct CLI {
    /// Display colon-separated MAC address.
    #[clap(short = 'c')]
    colon_separeted: bool,

    /// Display the address in uppercase.
    #[clap(short = 'u')]
    upcase: bool,

    /// IPv6 or MAC Address
    address: String,
}

impl CLI {
    pub fn run(&self) {
        if let Err(err) = self.run_inner() {
            println!("{}", err);
            exit(1);
        }
    }

    fn run_inner(&self) -> Result<(), ExecutionError> {
        if self.is_ipv6() {
            self.ipv6_to_mac_address()?;
        } else if self.is_mac_address() {
            self.mac_address_to_ipv6()?;
        } else {
            return Err(ExecutionError(String::from("Not IPv6 or MAC Address.")));
        }

        Ok(())
    }

    fn ipv6_to_mac_address(&self) -> Result<(), ExecutionError> {
        let ipv6 = IPv6::parse(self.address.as_str())?;
        let mac_address = ipv6.to_mac_address()?;

        let mac_address = if self.colon_separeted {
            mac_address.colon_separated()
        } else {
            mac_address.to_string()
        };

        let mac_address = if self.upcase {
            mac_address.to_uppercase()
        } else {
            mac_address
        };

        println!("{}", mac_address);

        Ok(())
    }

    fn mac_address_to_ipv6(&self) -> Result<(), ExecutionError> {
        let mac_address = EUI48::parse(self.address.as_str())?;
        let ipv6 = mac_address.to_ipv6().to_string();

        let ipv6 = if self.upcase {
            ipv6.to_uppercase()
        } else {
            ipv6
        };

        println!("{}", ipv6);

        Ok(())
    }

    fn is_ipv6(&self) -> bool {
        IPv6::parse(self.address.as_str()).is_ok()
    }

    fn is_mac_address(&self) -> bool {
        EUI48::parse(self.address.as_str()).is_ok()
    }
}
