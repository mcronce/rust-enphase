use std::fmt;
use std::str::FromStr;

use serde_with::DeserializeFromStr;
use serde_with::SerializeDisplay;

#[derive(Clone, Copy, Debug, DeserializeFromStr, SerializeDisplay)]
pub enum Granularity {
	Week,
	Day,
	FifteenMinutes
}

impl fmt::Display for Granularity {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Week => write!(f, "week"),
			Self::Day => write!(f, "day"),
			Self::FifteenMinutes => write!(f, "15mins")
		}
	}
}

impl FromStr for Granularity {
	type Err = InvalidGranularity;
	#[inline]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"week" => Ok(Self::Week),
			"day" => Ok(Self::Day),
			"15mins" => Ok(Self::FifteenMinutes),
			s => Err(InvalidGranularity(s.into()))
		}
	}
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid granularity \"{0}\"")]
pub struct InvalidGranularity(String);

