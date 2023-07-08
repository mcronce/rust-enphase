use serde::Deserialize;
use serde_with::serde_as;
use serde_with::DeserializeFromStr;
use strum::Display;
use strum::EnumString;

use super::Summary;

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(tag = "type")]
pub(super) enum ProductionCategory {
	#[serde(rename = "inverters")]
	Summary(Summary),
	#[serde(rename = "eim")]
	Detail(Detail)
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Detail {
	pub(super) measurement_type: MeasurementType,
	#[serde(flatten)]
	pub(super) inner: super::Detail
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Display, EnumString, DeserializeFromStr)]
pub(super) enum MeasurementType {
	#[strum(serialize = "production")]
	Production,
	#[strum(serialize = "total-consumption")]
	TotalConsumption,
	#[strum(serialize = "net-consumption")]
	NetConsumption
}
