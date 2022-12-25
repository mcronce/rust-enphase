use std::str::FromStr;

use serde_with::DeserializeFromStr;

#[derive(Clone, Debug, DeserializeFromStr)]
pub enum ConnectionType {
	Ethernet,
	WiFi
}

impl FromStr for ConnectionType {
	type Err = InvalidConnectionType;
	#[inline]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"ethernet" => Ok(Self::Ethernet),
			"wifi" => Ok(Self::WiFi),
			s => Err(InvalidConnectionType(s.into()))
		}
	}
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid connection type \"{0}\"")]
pub struct InvalidConnectionType(String);

