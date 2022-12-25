#![allow(unused_parens)]
use time::format_description::FormatItem;
use time::macros::format_description;

pub mod cloud;
pub mod envoy;
mod model;

pub(crate) const DATE_FORMAT: &[FormatItem] = format_description!("[year]-[month]-[day]");

