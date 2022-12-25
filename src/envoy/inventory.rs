use compact_str::CompactString;
use serde::Deserialize;
use serde::Deserializer;
use serde_with::serde_as;
use serde_with::DeserializeFromStr;
use serde_with::TimestampSeconds;
use smallvec::SmallVec;
use strum::EnumString;
use time::OffsetDateTime;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Inventory {
	pub pcu: Vec<Device>,
	pub acb: Vec<Device>,
	pub nsrb: Vec<Device>
}

impl<'de> Deserialize<'de> for Inventory {
	#[inline]
	fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
		let ir = InventoryIr::deserialize(de)?;
		let mut pcu = None;
		let mut acb = None;
		let mut nsrb = None;

		for section in ir.0 {
			match section.kind.as_ref() {
				"PCU" => pcu = Some(section.devices),
				"ACB" => acb = Some(section.devices),
				"NSRB" => nsrb = Some(section.devices),
				s => return Err(serde::de::Error::custom(format!("Unknown inventory section '{s}'")))
			};
		}

		let pcu = pcu.ok_or_else(|| serde::de::Error::custom("Missing 'PCU' inventory section"))?;
		let acb = acb.ok_or_else(|| serde::de::Error::custom("Missing 'ACB' inventory section"))?;
		let nsrb = nsrb.ok_or_else(|| serde::de::Error::custom("Missing 'NSRB' inventory section"))?;
		Ok(Self{
			pcu,
			acb,
			nsrb
		})
	}
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
struct InventoryIr(SmallVec<[InventoryIrSection; 3]>);

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
struct InventoryIrSection {
	#[serde(rename = "type")]
	kind: CompactString,
	devices: Vec<Device>
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumString, DeserializeFromStr)]
pub enum DeviceStatus {
	#[strum(serialize = "envoy.global.ok")]
	Ok,
	#[strum(serialize = "envoy.cond_flags.pcu_chan.dcvoltagetoolow")]
	DcVoltageTooLow,
	#[strum(serialize = "envoy.cond_flags.pcu_ctrl.dc-pwr-low")]
	DcPowerLow,
	#[strum(serialize = "envoy.cond_flags.obs_strs.failure")]
	Failure
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct DeviceControl {
	pub gficlearset: bool
}

#[serde_as]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Device {
	pub part_num: CompactString,
	#[serde_as(as = "TimestampSeconds<String>")]
	pub installed: OffsetDateTime,
	pub serial_num: CompactString,
	pub device_status: SmallVec<[DeviceStatus; 2]>,
	#[serde_as(as = "TimestampSeconds<String>")]
	pub last_rpt_date: OffsetDateTime,
	pub admin_state: u8,
	pub dev_type: u8,
	#[serde_as(as = "TimestampSeconds<String>")]
	pub created_date: OffsetDateTime,
	#[serde_as(as = "TimestampSeconds<String>")]
	pub img_load_date: OffsetDateTime,
	pub img_pnum_running: CompactString,
	pub ptpn: CompactString,
	#[serde_as(as = "TimestampSeconds<i64>")]
	pub chaneid: OffsetDateTime,
	pub device_control: SmallVec<[DeviceControl; 2]>,
	pub producing: bool,
	pub communicating: bool,
	pub provisioned: bool,
	pub operating: bool
}

#[cfg(test)]
mod tests {
	use smallvec::smallvec;
	use super::*;

	#[test]
	fn test_deserialize_single_device() {
		let s = include_str!("inventory/testdata/single-device.json");
		let device: Device = serde_json::from_str(s).unwrap();
		assert_eq!(device, Device{
			part_num: "800-00661-r08".into(),
			installed: OffsetDateTime::from_unix_timestamp(1571245440).unwrap(),
			serial_num: "121816047176".into(),
			device_status: smallvec![DeviceStatus::Ok],
			last_rpt_date: OffsetDateTime::from_unix_timestamp(1670868959).unwrap(),
			admin_state: 1,
			dev_type: 1,
			created_date: OffsetDateTime::from_unix_timestamp(1571245440).unwrap(),
			img_load_date: OffsetDateTime::from_unix_timestamp(1575566582).unwrap(),
			img_pnum_running: "520-00071-r01-v02.14.02".into(),
			ptpn: "540-00131-r01-v02.14.04".into(),
			chaneid: OffsetDateTime::from_unix_timestamp(1627390225).unwrap(),
			device_control: smallvec![DeviceControl{gficlearset: false}],
			producing: true,
			communicating: true,
			provisioned: true,
			operating: false
		});
	}

	#[test]
	fn test_deserialize_whole_inventory() {
		let s = include_str!("inventory/testdata/whole-inventory.json");
		let inventory: Inventory = serde_json::from_str(s).unwrap();
		assert_eq!(inventory.acb, vec![]);
		assert_eq!(inventory.nsrb, vec![]);
		assert_eq!(inventory.pcu[0], Device{
			part_num: "800-00661-r08".into(),
			installed: OffsetDateTime::from_unix_timestamp(1571245440).unwrap(),
			serial_num: "121816047176".into(),
			device_status: smallvec![DeviceStatus::Ok],
			last_rpt_date: OffsetDateTime::from_unix_timestamp(1670868959).unwrap(),
			admin_state: 1,
			dev_type: 1,
			created_date: OffsetDateTime::from_unix_timestamp(1571245440).unwrap(),
			img_load_date: OffsetDateTime::from_unix_timestamp(1575566582).unwrap(),
			img_pnum_running: "520-00071-r01-v02.14.02".into(),
			ptpn: "540-00131-r01-v02.14.04".into(),
			chaneid: OffsetDateTime::from_unix_timestamp(1627390225).unwrap(),
			device_control: smallvec![DeviceControl{gficlearset: false}],
			producing: true,
			communicating: true,
			provisioned: true,
			operating: false
		});
	}

	#[test]
	fn test_deserialize_whole_inventory_2() {
		let s = include_str!("inventory/testdata/whole-inventory-2.json");
		let inventory: Inventory = serde_json::from_str(s).unwrap();
		assert_eq!(inventory.acb, vec![]);
		assert_eq!(inventory.nsrb, vec![]);
		assert_eq!(inventory.pcu[0], Device{
			part_num: "800-00661-r08".into(),
			installed: OffsetDateTime::from_unix_timestamp(1571245440).unwrap(),
			serial_num: "121816047176".into(),
			device_status: smallvec![DeviceStatus::DcPowerLow, DeviceStatus::Failure],
			last_rpt_date: OffsetDateTime::from_unix_timestamp(1671053563).unwrap(),
			admin_state: 1,
			dev_type: 1,
			created_date: OffsetDateTime::from_unix_timestamp(1571245440).unwrap(),
			img_load_date: OffsetDateTime::from_unix_timestamp(1575566582).unwrap(),
			img_pnum_running: "520-00071-r01-v02.14.02".into(),
			ptpn: "540-00131-r01-v02.14.04".into(),
			chaneid: OffsetDateTime::from_unix_timestamp(1627390225).unwrap(),
			device_control: smallvec![DeviceControl{gficlearset: false}],
			producing: false,
			communicating: false,
			provisioned: false,
			operating: false
		});
	}
}

