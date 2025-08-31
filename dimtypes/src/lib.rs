//! 

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(const_ops)]
#![feature(const_trait_impl)]
#![feature(const_from)]


mod defs;
mod coretypes;

pub mod math;
pub use defs::{units,dimens,consts};
pub use coretypes::{Quantity,Unit,OffsetUnit,LogUnit};