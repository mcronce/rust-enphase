use compact_str::CompactString;
use serde::Deserialize;
use serde_with::serde_as;
use serde_with::TimestampSeconds;
use time::OffsetDateTime;

#[serde_as]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inverter {
	pub serial_number: CompactString,
	#[serde_as(as = "TimestampSeconds<i64>")]
	pub last_report_date: OffsetDateTime,
	pub dev_type: u8,
	pub last_report_watts: i16,
	pub max_report_watts: u16
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_deserialize_single() {
		let s = include_str!("inverters/testdata/single.json");
		let inverter: Inverter = serde_json::from_str(s).unwrap();
		assert_eq!(inverter, Inverter{
			serial_number: "121817002899".into(),
			last_report_date: OffsetDateTime::from_unix_timestamp(1670955839).unwrap(),
			dev_type: 1,
			last_report_watts: 55,
			max_report_watts: 245
		});
	}

	#[test]
	fn test_deserialize_many() {
		let s = include_str!("inverters/testdata/many.json");
		let inverters: Vec<Inverter> = serde_json::from_str(s).unwrap();
		assert_eq!(inverters.len(), 58);
		assert_eq!(inverters[1], Inverter{
			serial_number: "121817001633".into(),
			last_report_date: OffsetDateTime::from_unix_timestamp(1670955788).unwrap(),
			dev_type: 1,
			last_report_watts: 85,
			max_report_watts: 248
		});
		assert_eq!(inverters[57], Inverter{
			serial_number: "121920031546".into(),
			last_report_date: OffsetDateTime::from_unix_timestamp(1670955773).unwrap(),
			dev_type: 1,
			last_report_watts: 81,
			max_report_watts: 288
		});
	}

	#[test]
	fn test_deserialize_many_2() {
		let s = include_str!("inverters/testdata/many-2.json");
		let inverters: Vec<Inverter> = serde_json::from_str(s).unwrap();
		assert_eq!(inverters.len(), 58);
		assert_eq!(inverters[2], Inverter{
			serial_number: "121816046692".into(),
			last_report_date: OffsetDateTime::from_unix_timestamp(1671053554).unwrap(),
			dev_type: 1,
			last_report_watts: -4,
			max_report_watts: 248
		});
	}
}

