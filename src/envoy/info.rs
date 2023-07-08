use chrono::serde::ts_seconds;
use chrono::DateTime;
use chrono::Utc;
use compact_str::CompactString;
use serde::Deserialize;
use serde_with::serde_as;

#[serde_as]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Info {
	#[serde(with = "ts_seconds")]
	pub time: DateTime<Utc>,
	pub device: DeviceMetadata,
	#[serde(rename = "package")]
	pub packages: Vec<Package>,
	pub build_info: BuildInfo
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct DeviceMetadata {
	#[serde(rename = "sn")]
	pub serial_number: CompactString,
	#[serde(rename = "pn")]
	pub package_number: CompactString,
	pub software: CompactString,
	pub euaid: CompactString,
	pub seqnum: u8,
	pub apiver: u8,
	pub imeter: bool
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Package {
	pub name: CompactString,
	#[serde(rename = "pn")]
	pub package_number: CompactString,
	pub version: CompactString,
	pub build: CompactString
}

#[serde_as]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct BuildInfo {
	pub build_id: String,
	#[serde(with = "ts_seconds")]
	pub build_time_gmt: DateTime<Utc>
}

#[cfg(test)]
mod tests {
	use chrono::TimeZone;

	use super::*;

	#[test]
	fn test_deserialize_info() {
		let s = include_str!("info/testdata/info.xml");
		let info: Info = serde_xml_rs::from_str(s).unwrap();
		assert_eq!(
			info,
			Info {
				time: Utc.timestamp_opt(1670878465, 0).unwrap(),
				device: DeviceMetadata {
					serial_number: "121915008901".into(),
					package_number: "800-00555-r03".into(),
					software: "D5.0.49".into(),
					euaid: "4c8675".into(),
					seqnum: 0,
					apiver: 1,
					imeter: true
				},
				packages: vec![
					Package {
						name: "rootfs".into(),
						package_number: "500-00001-r01".into(),
						version: "02.00.00".into(),
						build: "950".into()
					},
					Package {
						name: "kernel".into(),
						package_number: "500-00011-r01".into(),
						version: "04.01.15".into(),
						build: "8f3564".into()
					},
					Package {
						name: "boot".into(),
						package_number: "590-00018-r01".into(),
						version: "02.00.01".into(),
						build: "426697".into()
					},
					Package {
						name: "app".into(),
						package_number: "500-00002-r01".into(),
						version: "05.00.49".into(),
						build: "77afa8".into()
					},
					Package {
						name: "devimg".into(),
						package_number: "500-00004-r01".into(),
						version: "01.02.245".into(),
						build: "1d5ba3".into()
					},
					Package {
						name: "geo".into(),
						package_number: "500-00008-r01".into(),
						version: "02.01.22".into(),
						build: "2faa48".into()
					},
					Package {
						name: "backbone".into(),
						package_number: "500-00010-r01".into(),
						version: "05.00.02".into(),
						build: "4fe435".into()
					},
					Package {
						name: "meter".into(),
						package_number: "500-00013-r01".into(),
						version: "03.02.07".into(),
						build: "276642".into()
					},
					Package {
						name: "agf".into(),
						package_number: "500-00012-r01".into(),
						version: "02.02.00".into(),
						build: "771104".into()
					},
					Package {
						name: "security".into(),
						package_number: "500-00016-r01".into(),
						version: "02.00.00".into(),
						build: "54a6dc".into()
					},
					Package {
						name: "full".into(),
						package_number: "500-00001-r01".into(),
						version: "02.00.00".into(),
						build: "950".into()
					}
				],
				build_info: BuildInfo {
					build_id: "release-5.0.x-88-Mar-19-20-02:43:57".into(),
					build_time_gmt: Utc.timestamp_opt(1584611077, 0).unwrap()
				}
			}
		);
	}
}
