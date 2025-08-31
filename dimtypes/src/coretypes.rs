use std::fmt;
use std::ops::{Add,Sub,Mul,Div,Neg};
use crate::dimens::Unitless;

/**
A [Quantity] represents a physical quantity with the power of each physical dimension encoded in the five [`isize`] const generics. Since this is generally clumsy to work with, the [dimens][crate::dimens] module provides type definitions for most quantities
of interest. For example [`Length`][crate::dimens::Length] aliases `Quantity<0,1,0,0,0>`, [`Force`][crate::dimens::Force] aliases `Quantity<-2,1,1,0,0>`, etc.

Internally, Quantity wraps a single [f64] value representing the physical quantity in SI base units. This ensures math between instances of Quantity always follows a consistent unit system.
*/
#[derive(Clone, Copy)]
pub struct Quantity<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize> {
	value_si: f64
}

/// Helper function to 
pub const fn div_evenly(num: isize, den: isize) -> isize {
	if num % den != 0 {
		panic!("Result would have non-integer power of dimension");
	}
	num/den
}

impl<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
Quantity<T,L,M,I,TEMP> {
	/// Get the numerical value of this quantity in the given `unit`.  `unit` must implement [Unit] with [Unit::Dimen] matching this quantity.
	pub fn as_unit(self, unit: impl Unit<Dimen=Self>) -> f64 {
		unit.qty_to_val(self)
	}

	/// Get the numerical value of this quantity in SI base units (s<sup>T</sup>m<sup>L</sup>kg<sup>M</sup>A<sup>I</sup>K<sup>TEMP</sup>)
	pub const fn as_si(self) -> f64 {
		self.value_si
	}

	/// Create a [Quantity] from a numerical value in the appropriate combination of SI base units (s<sup>T</sup>m<sup>L</sup>kg<sup>M</sup>A<sup>I</sup>K<sup>TEMP</sup>)  
	/// For [Unitless] quantities also consider using the [`From<f64>`] implementation (e.g. `Unitless::from(1.5)`)
	pub const fn from_si(val: f64) -> Self {
		Quantity { value_si:val }
	}


	/// Raise `self` to an integer power `P`.  Implemented as generic function since the dimenson (and thus type) of the result is dependent on the power
	pub fn pow<const P:isize>(self) ->
		Quantity<{P*T},{P*L},{P*M},{P*I},{P*TEMP}>
	{ 
			Quantity{value_si:self.value_si.powi(P as i32)}
	}

	/// Take the `R`th root of `self`.  Implemented as generic function since the dimenson (and thus type) of the result is dependent on the power.  
	/// `root::<R>` can only be called on types where all dimension powers are integer multiples of `R`.
	pub fn root<const R:isize>(self) ->
		Quantity<{div_evenly(T,R)},{div_evenly(L,R)},{div_evenly(M,R)},{div_evenly(I,R)},{div_evenly(TEMP,R)}>
	{
		Quantity{value_si:self.value_si.powf(1.0/(R as f64)) }
	}
}


/// [Unitless] quantities can coerce directly to/from [f64]
impl const From<f64> for Unitless {
	fn from(value: f64) -> Self { Quantity {value_si:value} }
}
/// [Unitless] quantities can coerce directly to/from [f64]
impl const From<Unitless> for f64 {
	fn from(value: Unitless) -> Self { value.value_si }
}



macro_rules! write_unit_power {
	($fmt:expr, $power:expr, $symbol:literal) => {
		if $power != 0 {
			write!($fmt, concat!(" ",$symbol))?;
			if $power != 1 {
				write!($fmt, "^{}", $power)?;
			}
		}
	}
}
macro_rules! fmt_impl_with_suffix {
	($suffix:literal) => {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			if let Some(digits) = f.precision() {
				write!(f, concat!("{1:.0$",$suffix,"}"), digits, self.value_si)?;
			} else {
				write!(f, concat!("{:",$suffix,"}"),  self.value_si)?;
			}
			write_unit_power!(f,M,"kg");
			write_unit_power!(f,L,"m");
			write_unit_power!(f,T,"s");
			write_unit_power!(f,I,"A");
			write_unit_power!(f,TEMP,"K");
			Ok(())
		}
	}
}

impl<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
fmt::Display for Quantity<T,L,M,I,TEMP> {
	fmt_impl_with_suffix!("");
}
impl<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
fmt::LowerExp for Quantity<T,L,M,I,TEMP> {
	fmt_impl_with_suffix!("e");
}
impl<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
fmt::UpperExp for Quantity<T,L,M,I,TEMP> {
	fmt_impl_with_suffix!("E");
}
impl<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
fmt::Debug for Quantity<T,L,M,I,TEMP> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(self, f) }
}




// Arithmetic

/// Define addition of any two [Quantities][Quantity] with the same dimension
impl<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
const Add for Quantity<T,L,M,I,TEMP> {
	/// Dimensioned addition does not change the dimension
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output { Quantity {value_si:self.value_si+rhs.value_si} }
}
/// Define subtraction of any two [Quantities][Quantity] with the same dimension
impl<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
const Sub for Quantity<T,L,M,I,TEMP> {
	/// Dimensioned subtraction does not change the dimension
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output { Quantity {value_si:self.value_si-rhs.value_si} }
}

impl<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
Neg for Quantity<T,L,M,I,TEMP> {
	type Output = Self;
	fn neg(self) -> Self { Quantity {value_si:-self.value_si} }
}


// The true magic - dimension tracking multiplication and division

/// Define unit-aware multiplication of any two [Quantities][Quantity], computing the correct dimensioned type for the result
impl<	const T1: isize, const L1: isize, const M1: isize, const I1: isize, const TEMP1: isize,
		const T2: isize, const L2: isize, const M2: isize, const I2: isize, const TEMP2: isize>
const Mul<Quantity<T2,L2,M2,I2,TEMP2>> for Quantity<T1,L1,M1,I1,TEMP1> where
	Quantity<{T1+T2},{L1+L2},{M1+M2},{I1+I2},{TEMP1+TEMP2}>: Sized
{
	/// Dimensioned multiplication produces a result with the sum of the exponents of each dimension
	type Output = Quantity<{T1+T2},{L1+L2},{M1+M2},{I1+I2},{TEMP1+TEMP2}>;
	fn mul(self, rhs: Quantity<T2,L2,M2,I2,TEMP2>) -> Quantity<{T1+T2},{L1+L2},{M1+M2},{I1+I2},{TEMP1+TEMP2}>
	{
		Quantity {value_si:self.value_si*rhs.value_si}
	}
}

/// Define unit-aware division of any two [Quantities][Quantity], computing the correct dimensioned type for the result
impl<	const T1: isize, const L1: isize, const M1: isize, const I1: isize, const TEMP1: isize,
		const T2: isize, const L2: isize, const M2: isize, const I2: isize, const TEMP2: isize>
const Div<Quantity<T2,L2,M2,I2,TEMP2>> for Quantity<T1,L1,M1,I1,TEMP1> where
	Quantity<{T1-T2},{L1-L2},{M1-M2},{I1-I2},{TEMP1-TEMP2}>: Sized
{
	/// Dimensioned division produces a result with the sum of the exponents of each dimension
	type Output = Quantity<{T1-T2},{L1-L2},{M1-M2},{I1-I2},{TEMP1-TEMP2}>;
	fn div(self, rhs: Quantity<T2,L2,M2,I2,TEMP2>) -> Quantity<{T1-T2},{L1-L2},{M1-M2},{I1-I2},{TEMP1-TEMP2}>
	{
		Quantity {value_si:self.value_si/rhs.value_si}
	}
}



/// Define direct operations with floats as unitless values to avoid needing from and into everywhere
impl<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
const Mul<f64> for Quantity<T,L,M,I,TEMP> {
	type Output = Self;
	fn mul(self, rhs: f64) -> Self::Output { Quantity{value_si:self.value_si*rhs} }
}
/// Define direct operations with floats as unitless values to avoid needing from and into everywhere
impl<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
const Div<f64> for Quantity<T,L,M,I,TEMP> {
	type Output = Self;
	fn div(self, rhs: f64) -> Self::Output { Quantity{value_si:self.value_si/rhs}  }
}
/// Define direct operations with floats as unitless values to avoid needing from and into everywhere
impl<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
const Mul<Quantity<T,L,M,I,TEMP>> for f64 {
	type Output = Quantity<T,L,M,I,TEMP>;
	fn mul(self, rhs: Quantity<T,L,M,I,TEMP>) -> Quantity<T,L,M,I,TEMP> { Quantity{value_si:self*rhs.value_si} }
}
/// Define direct operations with floats as unitless values to avoid needing from and into everywhere
impl<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
const Div<Quantity<T,L,M,I,TEMP>> for f64 where
	Quantity<{-T},{-L},{-M},{-I},{-TEMP}>: Sized
{
	type Output = Quantity<{-T},{-L},{-M},{-I},{-TEMP}>;
	fn div(self, rhs: Quantity<T,L,M,I,TEMP>) -> Quantity<{-T},{-L},{-M},{-I},{-TEMP}> { Quantity{value_si:self/rhs.value_si} }
}
/// Define direct operations with floats as unitless values to avoid needing from and into everywhere
impl const Add<f64> for Unitless {
	type Output = Unitless;
	fn add(self,rhs: f64) -> Unitless { self+Unitless::from(rhs) }
}
/// Define direct operations with floats as unitless values to avoid needing from and into everywhere
impl const Sub<f64> for Unitless {
	type Output = Unitless;
	fn sub(self,rhs: f64) -> Unitless { self-Unitless::from(rhs) }
}
/// Define direct operations with floats as unitless values to avoid needing from and into everywhere
impl const Add<Unitless> for f64 {
	type Output = Unitless;
	fn add(self,rhs: Unitless) -> Unitless { Unitless::from(self)+rhs }
}
/// Define direct operations with floats as unitless values to avoid needing from and into everywhere
impl const Sub<Unitless> for f64 {
	type Output = Unitless;
	fn sub(self,rhs: Unitless) -> Unitless { Unitless::from(self)-rhs }
}






// Unit definitions


/// A implementation of [Unit] represents a means to turn some physical quantity of type [Self::Dimen] into a numerical value ([f64]).
pub trait Unit : Copy {
	/// The dimensioned value to convert to/from its float value in whatever unit this represents.  This is normally some specialization of [Quantity] from [crate::dimens]
	type Dimen;
	/// Produce a physical quantity from a numerical value in this unit.  Should generally be inverse to [Self::val_to_qty()]
	fn qty_to_val(&self, value: Self::Dimen) -> f64;
	/// Produce the numerical value of this unit corresponding to the provided physical value.  Should generally be inverse to [Self::qty_to_val()]
	fn val_to_qty(&self, value: f64) -> Self::Dimen;
}

/// Any [Quantity] can also act as a unit of that type of quantity by division.
/// Most units are implmented in this fashion, except where nonlinear behavior is required (ref [OffsetUnit], [LogUnit])
impl<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
Unit for Quantity<T,L,M,I,TEMP> {
	type Dimen = Self;
	fn qty_to_val(&self, value: Self) -> f64 { value.value_si/self.value_si }
	fn val_to_qty(&self, value: f64) -> Self { value*(*self) }
}


/// Represents a [Unit] of `Dimen` with an offset zero, such as [CELSIUS][crate::units::CELSIUS] or [FAHRENHEIT][crate::units::FAHRENHEIT].  When using these units, care needs to be taken as to whether values represent absolute quantities or relative quantities (differences).  
#[derive(Clone, Copy, Debug)]
pub struct OffsetUnit<Dimen: Copy>{
	unit: Dimen,
	zero: Dimen
}
impl<Dimen: Copy> OffsetUnit<Dimen> {
	/// Create an offset unit with the same scale as `baseunit` but offset with 0 corresponding with the physical quantity `zero`
	pub const fn new(baseunit: Dimen, zero: Dimen) -> OffsetUnit<Dimen> {
		OffsetUnit{ unit:baseunit, zero:zero }
	}
	/// Get the zero quantity for this unit
	pub const fn zero_qty(&self) -> Dimen { self.zero }
}
impl<Dimen: Copy> OffsetUnit<Dimen> where
	OffsetUnit<Dimen>: Unit<Dimen=Dimen>
{
	/// Get the [Unit] implementation for this unit which considers quantities to be absolute.  This is the default implementation for [OffsetUnit] and just returns `self`
	pub const fn as_abs_unit(&self) -> impl Unit<Dimen=Dimen> { *self }	

	/// Get an absolute quantity from a numeric value of this unit.  Equivalent to `value*self`
	pub fn abs_qty_of(&self, value: f64) -> Dimen { self.val_to_qty(value) }
}
impl<Dimen: Copy> OffsetUnit<Dimen> where
	Dimen: Unit<Dimen=Dimen>
{
	/// Get the [Unit] implementation for this unit which considers quantities as relative.  This ignores the offset zero since it cancels out when performing relative differences.  For example, if you have
	/// two [Temperature][crate::dimens::Temperature] values `t1` and `t2` and want to know how many deg F apart they are, you would write <code>(t1-t2).as_unit([FAHRENHEIT][crate::units::FAHRENHEIT].as_rel_unit())</code> and not
	/// `(t1-t2).as_unit(FAHRENHEIT)` as the latter would interpret the difference as a (likely very cold) absolute temperature.
	pub const fn as_rel_unit(&self) -> impl Unit<Dimen=Dimen> { self.unit }
	
	/// Get a relative quantity in this unit.  This should be used when adding an offset to an existing value.  For example given a [Temperature][crate::dimens::Temperature] `temp`, to add 5 deg F you would
	/// write <code>temp+[FAHRENHEIT][crate::units::FAHRENHEIT].rel_qty_of(5.0)</code> and not `temp+5.0*FAHRENHEIT` since the latter would interpret `5.0*FAHRENHEIT` as an absolute temperature (258.15 K).
	pub fn rel_qty_of(&self, value: f64) -> Dimen { self.unit.val_to_qty(value) }
}
impl<Dimen,O> Unit for OffsetUnit<Dimen> where
	Dimen: Copy + Add<Dimen,Output=Dimen> + Sub<Dimen,Output=Dimen> + Div<Dimen,Output=O>,
	O: Into<f64>,
	f64: Mul<Dimen,Output=Dimen>
{
	type Dimen = Dimen;
	fn qty_to_val(&self, value: Dimen) -> f64 {
		((value - self.zero)/self.unit).into()
	}
	fn val_to_qty(&self, value: f64) -> Dimen {
		value*self.unit + self.zero
	}
}

/// Represents a logarthmically scaled [Unit] of `Dimen`
#[derive(Clone, Copy, Debug)]
pub struct LogUnit<Dimen: Copy> {
	scale: f64,
	reference: Dimen
}
impl<Dimen: Copy> LogUnit<Dimen> {
	/// Construct a logarithmic unit with base `base` and `scale` units per factor of `base` relative to the `reference` quantity
	pub fn new(base:f64,scale:f64,reference:Dimen) -> LogUnit<Dimen> {
		LogUnit { scale:scale/f64::log2(base), reference:reference }
	}
	/// Construct a logarithmic unit with `scale` units/octave relative to the `reference` quantity
	pub const fn base2(scale:f64,reference:Dimen) -> LogUnit<Dimen> {
		LogUnit { scale:scale, reference:reference }
	}
	/// Construct a logarithmic unit with `scale` units/decade relative to the `reference` quantity
	pub const fn base10(scale:f64,reference:Dimen) -> LogUnit<Dimen> { LogUnit::base2(scale/std::f64::consts::LOG2_10,reference) }
	/// Construct a logarithmic unit with `scale` units per factor of e relative to the `reference` quantity
	pub const fn basee(scale:f64,reference:Dimen) -> LogUnit<Dimen> { LogUnit::base2(scale/std::f64::consts::LOG2_E,reference) }
}
impl<Dimen: Copy> LogUnit<Dimen> where
	LogUnit<Dimen>: Unit<Dimen=Dimen>
{
	/// Get the physical quantity corresponding to the value `val` in this unit
	pub fn qty_of(&self, val: f64) -> Dimen { self.val_to_qty(val) }
}
impl<Dimen,O> Unit for LogUnit<Dimen> where
	Dimen: Copy + Div<Dimen,Output=O>,
	O: Into<f64>,
	f64: Mul<Dimen,Output=Dimen> + Mul<f64>
{
	type Dimen = Dimen;
	fn qty_to_val(&self, value: Dimen) -> f64 {
		let ratio: f64 = (value/self.reference).into();
		self.scale * f64::log2(ratio)
	}
	fn val_to_qty(&self, value: f64) -> Dimen {
		let ratio =f64::exp2(value/self.scale);
		ratio * self.reference
	}
}


// Multiplication Constructors
/**
Generates an implementation body to go in an `impl Mul<type> for f64`  on a type `type` implementing [Unit].
This implementation of [Mul] creates a quantity from the multiplying [f64] via [Unit::val_to_qty()]

Cannot do this generically since it is implemented on [f64] and it would implement [Mul] specializations for unowned types.
Generates only the body of the implmentation so the caller can add customized generics/generic bounds around the implmentation as needed.
*/
#[macro_export]
macro_rules! unit_mul_constructor_impl
{
	($type:ty) => {
		type Output = <$type as Unit>::Dimen;
		fn mul(self, rhs: $type) -> Self::Output { rhs.val_to_qty(self) }
	}
}
impl<Dimen: Copy> Mul<OffsetUnit<Dimen>> for f64 where
	OffsetUnit<Dimen>: Unit
{
	unit_mul_constructor_impl!(OffsetUnit<Dimen>);
}
impl<Dimen: Copy> Mul<LogUnit<Dimen>> for f64 where
	LogUnit<Dimen>: Unit
{
	unit_mul_constructor_impl!(LogUnit<Dimen>);
}