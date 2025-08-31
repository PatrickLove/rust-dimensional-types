//! Unit-aware variants of commmon mathematical function

use crate::Quantity;
use crate::dimens::Unitless;

/// [f64::atan2] implemented on dimensioned types.  The dimension of `x` and `y` must be the same.  
/// The result is a [Unitless] value representing the counterclockwise angle of the vector `[x,y]` with the x-axis.
pub fn atan2<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
	(x: Quantity<T,L,M,I,TEMP>, y: Quantity<T,L,M,I,TEMP>) -> Unitless {
	Unitless::from(f64::atan2(x.as_si(),y.as_si()))
}

macro_rules! reimpl_f64_to_unitless
{
	($func:ident) => {
		#[doc = concat!("Reimplementation of [f64::",stringify!($func),"] for [Unitless] types")]
		pub fn $func(x: Unitless) -> Unitless { Unitless::from(f64::$func(x.into())) }
	}
}

reimpl_f64_to_unitless!(sin);
reimpl_f64_to_unitless!(cos);
reimpl_f64_to_unitless!(tan);
reimpl_f64_to_unitless!(sinh);
reimpl_f64_to_unitless!(cosh);
reimpl_f64_to_unitless!(tanh);
reimpl_f64_to_unitless!(asin);
reimpl_f64_to_unitless!(acos);
reimpl_f64_to_unitless!(atan);
reimpl_f64_to_unitless!(asinh);
reimpl_f64_to_unitless!(acosh);
reimpl_f64_to_unitless!(atanh);
reimpl_f64_to_unitless!(ln);
reimpl_f64_to_unitless!(log10);
reimpl_f64_to_unitless!(exp);
