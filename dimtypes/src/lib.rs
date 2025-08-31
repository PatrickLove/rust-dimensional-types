
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(const_ops)]
#![feature(const_trait_impl)]
#![feature(const_from)]


use std::fmt;
use std::borrow::Borrow;
use std::ops::{Add,Sub,Mul,Div,Neg};
mod defs;
pub use defs::{units,dimens,consts};
use dimens::Unitless;

#[derive(Clone, Copy)]
pub struct Quantity<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize> {
	value_si: f64
}

pub trait Unit : Copy {
	type Dimen;
	fn qty_to_val(&self, value: Self::Dimen) -> f64;
	fn val_to_qty(&self, value: f64) -> Self::Dimen;
}

// Any quantity can be a unit, just divide the quantity in question by self as the "unit" quantity to get number of units
// All linear units can be implemented this way, but nonlinear units (e.g. deg C or deg F) can implment arbitrary conversions
impl<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
Unit for Quantity<T,L,M,I,TEMP> {
	type Dimen = Self;
	fn qty_to_val(&self, value: Self) -> f64 { value.value_si/self.value_si }
	fn val_to_qty(&self, value: f64) -> Self { value*(*self) }
}

// Cannot do this generically since it could match Unit implementations outside this crate
#[macro_export]
macro_rules! f64_mul_construct_impl
{
	($type:ty) => {
		type Output = <$type as Unit>::Dimen;
		fn mul(self, rhs: $type) -> Self::Output { rhs.val_to_qty(self) }
	}
}

#[derive(Clone, Copy)]
pub struct OffsetSystem<Dimen: Copy>{
	unit: Dimen,
	zero: Dimen
}
impl<Dimen: Copy> OffsetSystem<Dimen> {
	pub const fn new(baseunit: Dimen, zero: Dimen) -> OffsetSystem<Dimen> {
		OffsetSystem{ unit:baseunit, zero:zero }
	}
	pub const fn zero_qty(&self) -> Dimen { self.zero }
}

impl<Dimen: Copy> OffsetSystem<Dimen> where
	OffsetSystem<Dimen>: Unit<Dimen=Dimen>
{
	pub const fn as_abs_unit(&self) -> impl Unit<Dimen=Dimen> { *self }	
	pub fn abs_qty_of(&self, value: f64) -> Dimen { self.val_to_qty(value) }
}
impl<Dimen: Copy> OffsetSystem<Dimen> where
	Dimen: Unit<Dimen=Dimen>
{
	pub const fn as_rel_unit(&self) -> impl Unit<Dimen=Dimen> { self.unit }
	pub fn rel_qty_of(&self, value: f64) -> Dimen { self.unit.val_to_qty(value) }
}

impl<Dimen,O> Unit for OffsetSystem<Dimen> where
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
impl<Dimen: Copy> Mul<OffsetSystem<Dimen>> for f64 where
	OffsetSystem<Dimen>: Unit
{
	f64_mul_construct_impl!(OffsetSystem<Dimen>);
}


#[derive(Clone, Copy)]
pub struct LogUnit<Dimen: Copy> {
	base: f64,
	scale: f64,
	reference: Dimen
}
impl<Dimen: Copy> LogUnit<Dimen> {
	pub const fn new(base:f64,scale:f64,reference:Dimen) -> LogUnit<Dimen> {
		LogUnit { base:base, scale:scale, reference:reference }
	}
	pub const fn base10(scale:f64,reference:Dimen) -> LogUnit<Dimen> { LogUnit::new(10.0,scale,reference) }
	pub const fn base2(scale:f64,reference:Dimen) -> LogUnit<Dimen> { LogUnit::new(2.0,scale,reference) }
	pub const fn basee(scale:f64,reference:Dimen) -> LogUnit<Dimen> { LogUnit::new(std::f64::consts::E,scale,reference) }
}
impl<Dimen: Copy> LogUnit<Dimen> where
	LogUnit<Dimen>: Unit<Dimen=Dimen>
{
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
		self.scale*f64::log(ratio,self.base)
	}
	fn val_to_qty(&self, value: f64) -> Dimen {
		let ratio = self.base.powf(value/self.scale);
		ratio * self.reference
	}
}
impl<Dimen: Copy> Mul<LogUnit<Dimen>> for f64 where
	LogUnit<Dimen>: Unit
{
	f64_mul_construct_impl!(LogUnit<Dimen>);
}

pub const fn div_evenly(num: isize, den: isize) -> isize {
	if num % den != 0 {
		panic!("Result would have non-integer power of dimension");
	}
	num/den
}

impl<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
Quantity<T,L,M,I,TEMP> {
	pub fn as_unit(self, unit: impl Unit<Dimen=Self>) -> f64 { unit.borrow().qty_to_val(self) }
	pub const fn as_si(self) -> f64 { self.value_si }
	pub const fn from_si(val: f64) -> Self { Quantity { value_si:val } }
	pub fn pow<const P:isize>(self) -> Quantity<{P*T},{P*L},{P*M},{P*I},{P*TEMP}> { Quantity{value_si:self.value_si.powi(P as i32)} }
	pub fn root<const R:isize>(self) -> Quantity<{div_evenly(T,R)},{div_evenly(L,R)},{div_evenly(M,R)},{div_evenly(I,R)},{div_evenly(TEMP,R)}> { Quantity{value_si:self.value_si.powf(1.0/(R as f64)) } }
}

macro_rules! reimpl_f64_to_unitless
{
	($func:ident) => {
		pub fn $func(x: Unitless) -> Unitless { Unitless::from(f64::$func(x.into())) }
	}
}

pub mod math {
	use crate::Quantity;
	use crate::dimens::Unitless;
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

	pub fn atan2<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
		(x: Quantity<T,L,M,I,TEMP>, y: Quantity<T,L,M,I,TEMP>) -> Unitless {
		Unitless::from(f64::atan2(x.value_si,y.value_si))
	}
}


// Unitless quantities can coerce directly to/from floats
impl const From<f64> for Unitless {
	fn from(value: f64) -> Self { Quantity {value_si:value} }
}
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
impl<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
const Add for Quantity<T,L,M,I,TEMP> {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output { Quantity {value_si:self.value_si+rhs.value_si} }
}
impl<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
const Sub for Quantity<T,L,M,I,TEMP> {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output { Quantity {value_si:self.value_si-rhs.value_si} }
}
impl<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
Neg for Quantity<T,L,M,I,TEMP> {
	type Output = Self;
	fn neg(self) -> Self { Quantity {value_si:-self.value_si} }
}


// The true magic - dimension tracking multiplication and division
impl<	const T1: isize, const L1: isize, const M1: isize, const I1: isize, const TEMP1: isize,
		const T2: isize, const L2: isize, const M2: isize, const I2: isize, const TEMP2: isize>
const Mul<Quantity<T2,L2,M2,I2,TEMP2>> for Quantity<T1,L1,M1,I1,TEMP1> where
	Quantity<{T1+T2},{L1+L2},{M1+M2},{I1+I2},{TEMP1+TEMP2}>: Sized
{
	type Output = Quantity<{T1+T2},{L1+L2},{M1+M2},{I1+I2},{TEMP1+TEMP2}>;
	fn mul(self, rhs: Quantity<T2,L2,M2,I2,TEMP2>) -> Quantity<{T1+T2},{L1+L2},{M1+M2},{I1+I2},{TEMP1+TEMP2}>
	{
		Quantity {value_si:self.value_si*rhs.value_si}
	}
}
impl<	const T1: isize, const L1: isize, const M1: isize, const I1: isize, const TEMP1: isize,
		const T2: isize, const L2: isize, const M2: isize, const I2: isize, const TEMP2: isize>
const Div<Quantity<T2,L2,M2,I2,TEMP2>> for Quantity<T1,L1,M1,I1,TEMP1> where
	Quantity<{T1-T2},{L1-L2},{M1-M2},{I1-I2},{TEMP1-TEMP2}>: Sized
{
	type Output = Quantity<{T1-T2},{L1-L2},{M1-M2},{I1-I2},{TEMP1-TEMP2}>;
	fn div(self, rhs: Quantity<T2,L2,M2,I2,TEMP2>) -> Quantity<{T1-T2},{L1-L2},{M1-M2},{I1-I2},{TEMP1-TEMP2}>
	{
		Quantity {value_si:self.value_si/rhs.value_si}
	}
}



// Direct operations with floats to avoid needing from and into everywhere
impl<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
const Mul<f64> for Quantity<T,L,M,I,TEMP> {
	type Output = Self;
	fn mul(self, rhs: f64) -> Self::Output { Quantity{value_si:self.value_si*rhs} }
}
impl<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
const Div<f64> for Quantity<T,L,M,I,TEMP> {
	type Output = Self;
	fn div(self, rhs: f64) -> Self::Output { Quantity{value_si:self.value_si/rhs}  }
}
impl<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
const Mul<Quantity<T,L,M,I,TEMP>> for f64 {
	type Output = Quantity<T,L,M,I,TEMP>;
	fn mul(self, rhs: Quantity<T,L,M,I,TEMP>) -> Quantity<T,L,M,I,TEMP> { Quantity{value_si:self*rhs.value_si} }
}
impl<const T: isize, const L: isize, const M: isize, const I: isize, const TEMP: isize>
const Div<Quantity<T,L,M,I,TEMP>> for f64 where
	Quantity<{-T},{-L},{-M},{-I},{-TEMP}>: Sized
{
	type Output = Quantity<{-T},{-L},{-M},{-I},{-TEMP}>;
	fn div(self, rhs: Quantity<T,L,M,I,TEMP>) -> Quantity<{-T},{-L},{-M},{-I},{-TEMP}> { Quantity{value_si:self/rhs.value_si} }
}
impl const Add<f64> for Unitless {
	type Output = Unitless;
	fn add(self,rhs: f64) -> Unitless { self+Unitless::from(rhs) }
}
impl const Sub<f64> for Unitless {
	type Output = Unitless;
	fn sub(self,rhs: f64) -> Unitless { self-Unitless::from(rhs) }
}
impl const Add<Unitless> for f64 {
	type Output = Unitless;
	fn add(self,rhs: Unitless) -> Unitless { Unitless::from(self)+rhs }
}
impl const Sub<Unitless> for f64 {
	type Output = Unitless;
	fn sub(self,rhs: Unitless) -> Unitless { Unitless::from(self)-rhs }
}