use core::time::Duration;
use std::borrow::Cow;
use std::sync::Arc;

use arcstr::ArcStr;
use compact_str::CompactString;
use serde::Deserialize;
use serde_with::serde_as;
use serde_with::TimestampSeconds;
use time::Date;
use time::OffsetDateTime;
use time::Time;
use tokio::sync::Mutex;

use crate::DATE_FORMAT;
use super::ConnectionType;
use super::Granularity;

time::serde::format_description!(parse_date, Date, "[year]-[month]-[day]");

#[derive(Clone, Debug)]
pub struct System {
	client: reqwest::Client,
	api_key_qstr: ArcStr,
	auth_header: Arc<Mutex<String>>,
	pub system_id: u32,
	pub name: CompactString,
	pub public_name: CompactString,
	pub timezone: CompactString,
	pub address: Address,
	pub connection_type: ConnectionType,
	pub status: CompactString,
	pub last_report_at: OffsetDateTime,
	pub last_energy_at: OffsetDateTime,
	pub operational_at: OffsetDateTime,
	pub attachment_type: Option<CompactString>,
	pub interconnect_date: Option<Date>,
	pub other_references: Vec<CompactString>,
	pub energy_lifetime: u64,
	pub energy_today: u32,
	pub system_size: Option<f32>
}

impl System {
	pub async fn get_summary(&self) -> Result<SystemSummary, reqwest::Error> {
		// TODO:  Handle pagination
		self.client
			.get(format!("https://api.enphaseenergy.com/api/v4/systems/{}/summary?{}&size=100", self.system_id, self.api_key_qstr))
			.header("Authorization", &*self.auth_header.lock().await)
			.send()
			.await?
			.error_for_status()?
			.json()
			.await
	}

	pub async fn get_lifetime_production(&self, start_date: Option<&Date>, end_date: Option<&Date>, include_split_meter_and_microinverters: bool) -> Result<Vec<(OffsetDateTime, u32)>, reqwest::Error> {
		let mut args = Vec::with_capacity(4);
		args.push(Cow::Borrowed(self.api_key_qstr.as_ref()));
		if let Some(date) = start_date {
			args.push(Cow::Owned(format!("start_date={}", date.format(&DATE_FORMAT).unwrap())));
		}
		if let Some(date) = end_date {
			args.push(Cow::Owned(format!("end_date={}", date.format(&DATE_FORMAT).unwrap())));
		}
		if(include_split_meter_and_microinverters) {
			args.push(Cow::Borrowed("production=all"));
		}
		let response: LifetimeProductionResponse = self.client
			.get(format!("https://api.enphaseenergy.com/api/v4/systems/{}/energy_lifetime?{}", self.system_id, args.join("&")))
			.header("Authorization", &*self.auth_header.lock().await)
			.send()
			.await?
			.error_for_status()?
			.json()
			.await?;
		let start_date = response.start_date;
		let result = response
			.production
			.into_iter()
			.enumerate()
			.map(|(i, prod)| {
				let date = start_date + Duration::from_secs(86400 * i as u64);
				(date.with_time(Time::MIDNIGHT).assume_utc(), prod)
			})
			.collect();
		Ok(result)
	}

	pub async fn get_microinverter_production(&self, start_date: &Date, granularity: Option<Granularity>) -> Result<Vec<MicroinverterProduction>, reqwest::Error> {
		let mut args = Vec::with_capacity(3);
		args.push(Cow::Borrowed(self.api_key_qstr.as_ref()));
		args.push(Cow::Owned(format!("start_date={}", start_date.format(&DATE_FORMAT).unwrap())));
		if let Some(granularity) = granularity {
			args.push(Cow::Owned(format!("granularity={granularity}")));
		}
		let response: MicroinverterProductionResponse = self.client
			.get(format!("https://api.enphaseenergy.com/api/v4/systems/{}/telemetry/production_micro?{}", self.system_id, args.join("&")))
			.header("Authorization", &*self.auth_header.lock().await)
			.send()
			.await?
			.error_for_status()?
			.json()
			.await?;
		Ok(response.intervals)
	}
}

impl From<(reqwest::Client, ArcStr, Arc<Mutex<String>>, SystemResponse)> for System {
	#[inline]
	fn from(input: (reqwest::Client, ArcStr, Arc<Mutex<String>>, SystemResponse)) -> Self {
		Self{
			client: input.0,
			api_key_qstr: input.1,
			auth_header: input.2,
			system_id: input.3.system_id,
			name: input.3.name,
			public_name: input.3.public_name,
			timezone: input.3.timezone,
			address: input.3.address,
			connection_type: input.3.connection_type,
			status: input.3.status,
			last_report_at: input.3.last_report_at,
			last_energy_at: input.3.last_energy_at,
			operational_at: input.3.operational_at,
			attachment_type: input.3.attachment_type,
			interconnect_date: input.3.interconnect_date,
			other_references: input.3.other_references,
			energy_lifetime: input.3.energy_lifetime,
			energy_today: input.3.energy_today,
			system_size: input.3.system_size
		}
	}
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct SystemSummary {
	pub system_id: u32,
	pub current_power: u32,
	pub energy_lifetime: u32,
	pub energy_today: u32,
	#[serde_as(as = "Option<TimestampSeconds<i64>>")]
	pub last_interval_end_at: Option<OffsetDateTime>,
	#[serde_as(as = "TimestampSeconds<i64>")]
	pub last_report_at: OffsetDateTime,
	pub modules: u16,
	#[serde_as(as = "TimestampSeconds<i64>")]
	pub operational_at: OffsetDateTime,
	pub size_w: u32,
	pub source: CompactString,
	pub status: CompactString,
	#[serde(with = "parse_date")]
	pub summary_date: Date
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ListSystemsResponse {
	//total: u16,
	//current_page: u8,
	//size: u8,
	//count: u8,
	//items: CompactString,
	pub systems: Vec<SystemResponse>
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct LifetimeProductionResponse {
	//system_id: u32,
	#[serde(with = "parse_date")]
	start_date: Date,
	production: Vec<u32>,
	//meta: Metadata
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct MicroinverterProductionResponse {
	//system_id: u32,
	//granularity: Granularity,
	//total_devices: u16,
	//#[serde(with = "time::serde::iso8601")]
	//start_date: OffsetDateTime,
	//#[serde(with = "time::serde::iso8601")]
	//end_date: OffsetDateTime,
	//items: CompactString,
	intervals: Vec<MicroinverterProduction>,
	//meta: Metadata
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct MicroinverterProduction {
	#[serde_as(as = "TimestampSeconds<i64>")]
	pub end_at: OffsetDateTime,
	pub devices_reporting: u16,
	#[serde(rename = "powr")]
	pub instantaneous_power_watts: u32,
	#[serde(rename = "enwh")]
	pub energy_this_interval_wh: u32
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct SystemResponse {
	system_id: u32,
	name: CompactString,
	public_name: CompactString,
	timezone: CompactString,
	address: Address,
	connection_type: ConnectionType,
	status: CompactString,
	#[serde_as(as = "TimestampSeconds<i64>")]
	last_report_at: OffsetDateTime,
	#[serde_as(as = "TimestampSeconds<i64>")]
	last_energy_at: OffsetDateTime,
	#[serde_as(as = "TimestampSeconds<i64>")]
	operational_at: OffsetDateTime,
	attachment_type: Option<CompactString>,
	interconnect_date: Option<Date>,
	other_references: Vec<CompactString>,
	energy_lifetime: u64,
	energy_today: u32,
	system_size: Option<f32>
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct Metadata {
	pub status: CompactString,
	#[serde_as(as = "TimestampSeconds<i64>")]
	pub last_report_at: OffsetDateTime,
	#[serde_as(as = "TimestampSeconds<i64>")]
	pub last_energy_at: OffsetDateTime,
	#[serde_as(as = "Option<TimestampSeconds<i64>>")]
	pub opertional_at: Option<OffsetDateTime>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Address {
	pub country: CompactString,
	pub state: CompactString,
	pub postal_code: CompactString
}

