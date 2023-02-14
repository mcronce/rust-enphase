#![allow(unused_parens)]
pub mod cloud;
pub mod envoy;
mod model;
pub use model::AggregateProduction;

pub(crate) const DATE_FORMAT: &str = "%Y-%m-%d";

