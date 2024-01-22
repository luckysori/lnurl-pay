// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

use core::fmt;

#[derive(Debug)]
pub enum Error {
    Bech32(bech32::Error),
    #[cfg(feature = "api")]
    Reqwest(reqwest::Error),
    InvalidLnUrl,
    InvalidLightningAddress,
    UnknownTag,
    AmountTooLow {
        msats: u64,
        min: u64,
    },
    AmountTooHigh {
        msats: u64,
        max: u64,
    },
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bech32(e) => write!(f, "Bech32: {e}"),
            #[cfg(feature = "api")]
            Self::Reqwest(e) => write!(f, "Reqwest: {e}"),
            Self::InvalidLnUrl => write!(f, "Invalid LNURL"),
            Self::InvalidLightningAddress => write!(f, "Invalid Lightning Address"),
            Self::UnknownTag => write!(f, "Unknown tag"),
            Self::AmountTooLow { msats, min } => {
                write!(f, "Amount too low: {msats} msats (min. {min} msats)")
            }
            Self::AmountTooHigh { msats, max } => {
                write!(f, "Amount too high: {msats} msats (max. {max} msats)")
            }
        }
    }
}

impl From<bech32::Error> for Error {
    fn from(e: bech32::Error) -> Self {
        Self::Bech32(e)
    }
}

#[cfg(feature = "api")]
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}
