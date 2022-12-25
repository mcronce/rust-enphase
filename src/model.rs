use core::cmp;

use time::OffsetDateTime;

#[derive(Clone, Debug)]
pub struct AggregateProduction {
	pub timestamp: OffsetDateTime,
	pub inverters_reporting: u16,
	pub instantaneous_power_watts: u32,
}

impl Default for AggregateProduction {
	#[inline]
	fn default() -> Self {
		Self{
			timestamp: OffsetDateTime::from_unix_timestamp(0).unwrap(),
			inverters_reporting: 0,
			instantaneous_power_watts: 0,
		}
	}
}

impl From<&[crate::envoy::Inverter]> for AggregateProduction {
	#[inline]
	fn from(raw: &[crate::envoy::Inverter]) -> Self {
		let mut aggregate = Self{
			inverters_reporting: raw.len() as u16,
			..Default::default()
		};
		for inverter in raw {
			aggregate.timestamp = cmp::max(aggregate.timestamp, inverter.last_report_date);
			aggregate.instantaneous_power_watts += inverter.last_report_watts as u32;
		}
		aggregate
	}
}

impl From<crate::cloud::MicroinverterProduction> for AggregateProduction {
	#[inline]
	fn from(raw: crate::cloud::MicroinverterProduction) -> Self {
		Self{
			timestamp: raw.end_at,
			inverters_reporting: raw.devices_reporting,
			instantaneous_power_watts: raw.instantaneous_power_watts
		}
	}
}

