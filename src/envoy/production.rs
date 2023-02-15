use chrono::serde::ts_seconds;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Deserializer;
use serde_with::DeserializeFromStr;
use strum::EnumString;

mod ir;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct EnergyStats {
	pub production: Production,
	pub consumption: Consumption,
	pub storage: Vec<Storage>
}

#[derive(Clone, Debug, PartialEq)]
pub struct Production {
	pub summary: Summary,
	pub detail: Detail
}

impl<'de> Deserialize<'de> for Production {
	#[inline]
	fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
		let ir = <[ir::ProductionCategory; 2]>::deserialize(de)?;
		let mut summary = None;
		let mut detail = None;

		for section in ir {
			match section {
				ir::ProductionCategory::Summary(v) => summary = Some(v),
				ir::ProductionCategory::Detail(v) => detail = Some(v.inner)
			};
		}

		let summary = summary.ok_or_else(|| serde::de::Error::custom("Missing 'inverters' production section"))?;
		let detail = detail.ok_or_else(|| serde::de::Error::custom("Missing 'eim' production section"))?;
		Ok(Self{
			summary,
			detail
		})
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct Consumption {
	pub total: Detail,
	pub net: Detail
}

impl<'de> Deserialize<'de> for Consumption {
	#[inline]
	fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
		let ir = <[ir::Detail; 2]>::deserialize(de)?;
		let mut total = None;
		let mut net = None;

		for section in ir {
			match section.measurement_type {
				ir::MeasurementType::TotalConsumption => total = Some(section.inner),
				ir::MeasurementType::NetConsumption => net = Some(section.inner),
				v => return Err(serde::de::Error::custom(format!("Found unexpected consumption section '{v}'")))
			};
		}

		let total = total.ok_or_else(|| serde::de::Error::custom("Missing 'total-consumption' consumption section"))?;
		let net = net.ok_or_else(|| serde::de::Error::custom("Missing 'net-consumption' consumption section"))?;
		Ok(Self{
			total,
			net
		})
	}
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
	pub active_count: u16,
	#[serde(with = "ts_seconds")]
	pub reading_time: DateTime<Utc>,
	#[serde(rename = "wNow")]
	pub watts_now: f32,
	#[serde(rename = "whLifetime")]
	pub watt_hours_lifetime: u64
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Detail {
	pub active_count: u16,
	#[serde(with = "ts_seconds")]
	pub reading_time: DateTime<Utc>,
	#[serde(rename = "wNow")]
	pub watts_now: f32,
	#[serde(rename = "whToday")]
	pub watt_hours_today: f32,
	#[serde(rename = "whLastSevenDays")]
	pub watt_hours_last_seven_days: f32,
	#[serde(rename = "whLifetime")]
	pub watt_hours_lifetime: f64,
	pub varh_lead_today: f32,
	pub varh_lead_lifetime: f32,
	pub varh_lag_today: f32,
	pub varh_lag_lifetime: f32,
	pub vah_lifetime: f32,
	pub rms_current: f32,
	pub rms_voltage: f32,
	#[serde(rename = "reactPwr")]
	pub react_power: f32,
	#[serde(rename = "apprntPwr")]
	pub apparent_power: f32,
	#[serde(rename = "pwrFactor")]
	pub power_factor: f32,
	pub vah_today: f32,
	pub lines: Vec<Line>
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Line {
	#[serde(rename = "wNow")]
	pub watts_now: f32,
	#[serde(rename = "whToday")]
	pub watt_hours_today: f32,
	#[serde(rename = "whLastSevenDays")]
	pub watt_hours_last_seven_days: f32,
	#[serde(rename = "whLifetime")]
	pub watt_hours_lifetime: f64,
	pub varh_lead_today: f32,
	pub varh_lead_lifetime: f32,
	pub varh_lag_today: f32,
	pub varh_lag_lifetime: f32,
	pub vah_today: f32,
	pub vah_lifetime: f32,
	pub rms_current: f32,
	pub rms_voltage: f32,
	#[serde(rename = "reactPwr")]
	pub react_power: f32,
	#[serde(rename = "apprntPwr")]
	pub apparent_power: f32,
	#[serde(rename = "pwrFactor")]
	pub power_factor: f32
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Storage {
	#[serde(rename = "type")]
	pub kind: StorageType,
	pub active_count: u16,
	#[serde(with = "ts_seconds")]
	pub reading_time: DateTime<Utc>,
	#[serde(rename = "wNow")]
	pub watts_now: f32,
	#[serde(rename = "whNow")]
	pub watt_hours_now: f32,
	pub state: StorageState
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumString, DeserializeFromStr)]
pub enum StorageType {
	#[strum(serialize = "acb")]
	Acb
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumString, DeserializeFromStr)]
pub enum StorageState {
	#[strum(serialize = "idle")]
	Idle
}

#[cfg(test)]
mod tests {
	use chrono::TimeZone;
	use super::*;

	#[test]
	fn test_deserialize_production() {
		let s = include_str!("production/testdata/production-detail.json");
		let stats: EnergyStats = serde_json::from_str(s).unwrap();
		let expected = EnergyStats{ // {{{
			production: Production{
				summary: Summary{
					active_count: 58,
					reading_time: Utc.timestamp_opt(1670878991, 0).unwrap(),
					watts_now: 164,
					watt_hours_lifetime: 57341389
				},
				detail: Detail{ // {{{
					active_count: 0,
					reading_time: Utc.timestamp_opt(1670879008, 0).unwrap(),
					watts_now: 313.472,
					watt_hours_today: 0.0,
					watt_hours_last_seven_days: 0.0,
					watt_hours_lifetime: 0.0,
					varh_lead_today: 0.0,
					varh_lead_lifetime: 0.0,
					varh_lag_today: 0.0,
					varh_lag_lifetime: 0.0,
					vah_today: 0.0,
					vah_lifetime: 0.0,
					rms_current: 9.819,
					rms_voltage: 240.991,
					react_power: 1108.847,
					apparent_power: 1182.584,
					power_factor: 0.27,
					lines: vec![
						Line{
							watts_now: 157.389,
							watt_hours_today: 0.0,
							watt_hours_last_seven_days: 0.0,
							watt_hours_lifetime: 0.0,
							varh_lead_today: 0.0,
							varh_lead_lifetime: 0.0,
							varh_lag_today: 0.0,
							varh_lag_lifetime: 0.0,
							vah_today: 0.0,
							vah_lifetime: 0.0,
							rms_current: 4.906,
							rms_voltage: 120.149,
							react_power: 552.172,
							apparent_power: 589.108,
							power_factor: 0.27
						},
						Line{
							watts_now: 156.083,
							watt_hours_today: 0.0,
							watt_hours_last_seven_days: 0.0,
							watt_hours_lifetime: 0.0,
							varh_lead_today: 0.0,
							varh_lead_lifetime: 0.0,
							varh_lag_today: 0.0,
							varh_lag_lifetime: 0.0,
							vah_today: 0.0,
							vah_lifetime: 0.0,
							rms_current: 4.913,
							rms_voltage: 120.842,
							react_power: 556.675,
							apparent_power: 593.476,
							power_factor: 0.27
						}
					]
				/* }}} */ }
			},
			consumption: Consumption{
				total: Detail{ // {{{
					active_count: 0,
					reading_time: Utc.timestamp_opt(1670879008, 0).unwrap(),
					watts_now: 313.472,
					watt_hours_today: 0.0,
					watt_hours_last_seven_days: 0.0,
					watt_hours_lifetime: 0.0,
					varh_lead_today: 0.0,
					varh_lead_lifetime: 0.0,
					varh_lag_today: 0.0,
					varh_lag_lifetime: 0.0,
					vah_today: 0.0,
					vah_lifetime: 0.0,
					rms_current: 10.085,
					rms_voltage: 240.954,
					react_power: -1108.847,
					apparent_power: 2430.089,
					power_factor: 0.13,
					lines: vec![
						Line{
							watts_now: 157.389,
							watt_hours_today: 0.0,
							watt_hours_last_seven_days: 0.0,
							watt_hours_lifetime: 0.0,
							varh_lead_today: 0.0,
							varh_lead_lifetime: 0.0,
							varh_lag_today: 0.0,
							varh_lag_lifetime: 0.0,
							vah_today: 0.0,
							vah_lifetime: 0.0,
							rms_current: 5.034,
							rms_voltage: 120.133,
							react_power: -552.172,
							apparent_power: 604.808,
							power_factor: 0.26
						},
						Line{
							watts_now: 156.083,
							watt_hours_today: 0.0,
							watt_hours_last_seven_days: 0.0,
							watt_hours_lifetime: 0.0,
							varh_lead_today: 0.0,
							varh_lead_lifetime: 0.0,
							varh_lag_today: 0.0,
							varh_lag_lifetime: 0.0,
							vah_today: 0.0,
							vah_lifetime: 0.0,
							rms_current: 5.051,
							rms_voltage: 120.821,
							react_power: -556.675,
							apparent_power: 610.242,
							power_factor: 0.26
						}
					]
				/* }}} */ },
				net: Detail{ // {{{
					active_count: 0,
					reading_time: Utc.timestamp_opt(1670879008, 0).unwrap(),
					watts_now: 0.0,
					watt_hours_today: 0.0,
					watt_hours_last_seven_days: 0.0,
					watt_hours_lifetime: 0.0,
					varh_lead_today: 0.0,
					varh_lead_lifetime: 0.0,
					varh_lag_today: 0.0,
					varh_lag_lifetime: 0.0,
					vah_today: 0.0,
					vah_lifetime: 0.0,
					rms_current: 0.266,
					rms_voltage: 240.918,
					react_power: 0.0,
					apparent_power: 32.207,
					power_factor: 0.0,
					lines: vec![
						Line{
							watts_now: 0.0,
							watt_hours_today: 0.0,
							watt_hours_last_seven_days: 0.0,
							watt_hours_lifetime: 0.0,
							varh_lead_today: 0.0,
							varh_lead_lifetime: 0.0,
							varh_lag_today: 0.0,
							varh_lag_lifetime: 0.0,
							vah_today: 0.0,
							vah_lifetime: 0.0,
							rms_current: 0.129,
							rms_voltage: 120.118,
							react_power: -0.0,
							apparent_power: 15.473,
							power_factor: 0.0
						},
						Line{
							watts_now: 0.0,
							watt_hours_today: 0.0,
							watt_hours_last_seven_days: 0.0,
							watt_hours_lifetime: 0.0,
							varh_lead_today: 0.0,
							varh_lead_lifetime: 0.0,
							varh_lag_today: 0.0,
							varh_lag_lifetime: 0.0,
							vah_today: 0.0,
							vah_lifetime: 0.0,
							rms_current: 0.138,
							rms_voltage: 120.8,
							react_power: 0.0,
							apparent_power: 16.734,
							power_factor: 0.0
						}
					]
				/* }}} */ }
			},
			storage: vec![
				Storage{
					kind: StorageType::Acb,
					active_count: 0,
					reading_time: Utc.timestamp_opt(0, 0).unwrap(),
					watts_now: 0.0,
					watt_hours_now: 0.0,
					state: StorageState::Idle
				}
			]
		/* }}} */ };
		assert_eq!(stats, expected);
	}

	#[test]
	fn test_deserialize_production_2() {
		let s = include_str!("production/testdata/production-detail-2.json");
		let stats: EnergyStats = serde_json::from_str(s).unwrap();
		let expected = EnergyStats{ // {{{
			production: Production{
				summary: Summary{
					active_count: 58,
					reading_time: Utc.timestamp_opt(1671051033, 0).unwrap(),
					watts_now: 418,
					watt_hours_lifetime: 57397093
				},
				detail: Detail{ // {{{
					active_count: 0,
					reading_time: Utc.timestamp_opt(1671051078, 0).unwrap(),
					watts_now: 542.243,
					watt_hours_today: 0.0,
					watt_hours_last_seven_days: 0.0,
					watt_hours_lifetime: 0.0,
					varh_lead_today: 0.0,
					varh_lead_lifetime: 0.0,
					varh_lag_today: 0.0,
					varh_lag_lifetime: 0.0,
					vah_today: 0.0,
					vah_lifetime: 0.0,
					rms_current: 10.599,
					rms_voltage: 239.986,
					react_power: 1099.473,
					apparent_power: 1272.086,
					power_factor: 0.39,
					lines: vec![
						Line{
							watts_now: 270.825,
							watt_hours_today: 0.0,
							watt_hours_last_seven_days: 0.0,
							watt_hours_lifetime: 0.0,
							varh_lead_today: 0.0,
							varh_lead_lifetime: 0.0,
							varh_lag_today: 0.0,
							varh_lag_lifetime: 0.0,
							vah_today: 0.0,
							vah_lifetime: 0.0,
							rms_current: 5.297,
							rms_voltage: 119.678,
							react_power: 547.902,
							apparent_power: 634.117,
							power_factor: 0.39
						},
						Line{
							watts_now: 271.418,
							watt_hours_today: 0.0,
							watt_hours_last_seven_days: 0.0,
							watt_hours_lifetime: 0.0,
							varh_lead_today: 0.0,
							varh_lead_lifetime: 0.0,
							varh_lag_today: 0.0,
							varh_lag_lifetime: 0.0,
							vah_today: 0.0,
							vah_lifetime: 0.0,
							rms_current: 5.302,
							rms_voltage: 120.308,
							react_power: 551.571,
							apparent_power: 637.969,
							power_factor: 0.4
						}
					]
				/* }}} */ }
			},
			consumption: Consumption{
				total: Detail{ // {{{
					active_count: 0,
					reading_time: Utc.timestamp_opt(1671051078, 0).unwrap(),
					watts_now: 542.243,
					watt_hours_today: 0.0,
					watt_hours_last_seven_days: 0.0,
					watt_hours_lifetime: 0.0,
					varh_lead_today: 0.0,
					varh_lead_lifetime: 0.0,
					varh_lag_today: 0.0,
					varh_lag_lifetime: 0.0,
					vah_today: 0.0,
					vah_lifetime: 0.0,
					rms_current: 10.867,
					rms_voltage: 240.014,
					react_power: -1099.473,
					apparent_power: 2608.199,
					power_factor: 0.21,
					lines: vec![
						Line{
							watts_now: 270.825,
							watt_hours_today: 0.0,
							watt_hours_last_seven_days: 0.0,
							watt_hours_lifetime: 0.0,
							varh_lead_today: 0.0,
							varh_lead_lifetime: 0.0,
							varh_lag_today: 0.0,
							varh_lag_lifetime: 0.0,
							vah_today: 0.0,
							vah_lifetime: 0.0,
							rms_current: 5.428,
							rms_voltage: 119.684,
							react_power: -547.902,
							apparent_power: 649.662,
							power_factor: 0.42
						},
						Line{
							watts_now: 271.418,
							watt_hours_today: 0.0,
							watt_hours_last_seven_days: 0.0,
							watt_hours_lifetime: 0.0,
							varh_lead_today: 0.0,
							varh_lead_lifetime: 0.0,
							varh_lag_today: 0.0,
							varh_lag_lifetime: 0.0,
							vah_today: 0.0,
							vah_lifetime: 0.0,
							rms_current: 5.439,
							rms_voltage: 120.33,
							react_power: -551.571,
							apparent_power: 654.441,
							power_factor: 0.41
						}
					]
				/* }}} */ },
				net: Detail{ // {{{
					active_count: 0,
					reading_time: Utc.timestamp_opt(1671051078, 0).unwrap(),
					watts_now: 0.0,
					watt_hours_today: 0.0,
					watt_hours_last_seven_days: 0.0,
					watt_hours_lifetime: 0.0,
					varh_lead_today: 0.0,
					varh_lead_lifetime: 0.0,
					varh_lag_today: 0.0,
					varh_lag_lifetime: 0.0,
					vah_today: 0.0,
					vah_lifetime: 0.0,
					rms_current: 0.268,
					rms_voltage: 240.042,
					react_power: 0.0,
					apparent_power: 32.074,
					power_factor: 0.0,
					lines: vec![
						Line{
							watts_now: 0.0,
							watt_hours_today: 0.0,
							watt_hours_last_seven_days: 0.0,
							watt_hours_lifetime: 0.0,
							varh_lead_today: 0.0,
							varh_lead_lifetime: 0.0,
							varh_lag_today: 0.0,
							varh_lag_lifetime: 0.0,
							vah_today: 0.0,
							vah_lifetime: 0.0,
							rms_current: 0.131,
							rms_voltage: 119.691,
							react_power: -0.0,
							apparent_power: 15.654,
							power_factor: 0.0
						},
						Line{
							watts_now: 0.0,
							watt_hours_today: 0.0,
							watt_hours_last_seven_days: 0.0,
							watt_hours_lifetime: 0.0,
							varh_lead_today: 0.0,
							varh_lead_lifetime: 0.0,
							varh_lag_today: 0.0,
							varh_lag_lifetime: 0.0,
							vah_today: 0.0,
							vah_lifetime: 0.0,
							rms_current: 0.137,
							rms_voltage: 120.351,
							react_power: 0.0,
							apparent_power: 16.42,
							power_factor: 0.0
						}
					]
				/* }}} */ }
			},
			storage: vec![
				Storage{
					kind: StorageType::Acb,
					active_count: 0,
					reading_time: Utc.timestamp_opt(0, 0).unwrap(),
					watts_now: 0.0,
					watt_hours_now: 0.0,
					state: StorageState::Idle
				}
			]
		/* }}} */ };
		assert_eq!(stats, expected);
	}
}

