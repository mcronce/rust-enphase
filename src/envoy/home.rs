use std::net::IpAddr;

use compact_str::CompactString;
use macaddr::MacAddr6;
use serde::Deserialize;
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use serde_with::TimestampSeconds;
use time::OffsetDateTime;

#[serde_as]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Home {
	#[serde_as(as = "TimestampSeconds<i64>")]
	pub software_build_epoch: OffsetDateTime,
	pub is_nonvoy: bool,
	pub db_size: CompactString,
	#[serde_as(as = "DisplayFromStr")]
	pub db_percent_full: u8,
	pub timezone: CompactString,
	pub current_date: CompactString,
	pub current_time: CompactString,
	pub network: Network,
	pub tariff: CompactString,
	// TODO:  comm
	// TODO:  alerts
	pub update_status: CompactString
}

#[serde_as]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Network {
	pub web_comm: bool,
	pub ever_reported_to_enlighten: bool,
	#[serde_as(as = "TimestampSeconds<i64>")]
	pub last_enlighten_report_time: OffsetDateTime,
	pub primary_interface: CompactString,
	pub interfaces: Vec<Interface>
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(tag = "type")]
pub enum Interface {
	#[serde(rename = "ethernet")]
	Wired(WiredInterface),
	#[serde(rename = "wifi")]
	WiFi(WiFiInterface)
}

#[serde_as]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct WiredInterface {
	pub interface: CompactString,
	#[serde_as(as = "DisplayFromStr")]
	pub mac: MacAddr6,
	pub dhcp: bool,
	pub ip: IpAddr,
	//pub signal_strength: u8,
	//pub signal_strength_max: u8,
	pub carrier: bool
}

#[serde_as]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct WiFiInterface {
	pub interface: CompactString,
	pub signal_strength: u8,
	pub signal_strength_max: u8,
	#[serde_as(as = "DisplayFromStr")]
	pub mac: MacAddr6,
	pub dhcp: bool,
	pub ip: IpAddr,
	pub carrier: bool,
	pub supported: bool,
	pub present: bool,
	pub configured: bool,
	pub status: CompactString
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_deserialize_home() {
		let s = include_str!("home/testdata/home.json");
		let home: Home = serde_json::from_str(s).unwrap();
		assert_eq!(home, Home{
			software_build_epoch: OffsetDateTime::from_unix_timestamp(1584607493).unwrap(),
			is_nonvoy: false,
			db_size: "43 MB".into(),
			db_percent_full: 11,
			timezone: "US/Eastern".into(),
			current_date: "12/12/2022".into(),
			current_time: "14:36".into(),
			network: Network{
				web_comm: true,
				ever_reported_to_enlighten: true,
				last_enlighten_report_time: OffsetDateTime::from_unix_timestamp(1670873269).unwrap(),
				primary_interface: "none".into(),
				interfaces: vec![
					Interface::Wired(WiredInterface{
						interface: "eth0".into(),
						mac: MacAddr6::new(0x00, 0x1d, 0xc0, 0x6d, 0x32, 0xc6),
						dhcp: true,
						ip: IpAddr::from([169, 254, 120, 1]),
						carrier: false
					}),
					Interface::WiFi(WiFiInterface{
						interface: "wlan0".into(),
						signal_strength: 3,
						signal_strength_max: 5,
						mac: MacAddr6::new(0x38, 0x81, 0xd7, 0x35, 0x9b, 0xd2),
						dhcp: true,
						ip: IpAddr::from([192, 168, 1, 80]),
						carrier: true,
						supported: true,
						present: true,
						configured: true,
						status: "connected".into()
					})
				],
			},
			tariff: "none".into(),
			update_status: "satisfied".into()
		});
	}
}

