# Rust Dimensional Types

This project brings compile time dimensional analysis to rust using const generics.  This crate currently requires nightly for `!#[generic_const_exprs]` to do math on the generics, and a few other const-related features to allow const expressions involving dimensional types.

The core of the system is the `dimtypes::Quantity<Time, Length, Mass, Current, Temperature>` generic struct which represents a physical quantity with the power of each physical dimension encoded in the 5 `isize` const generics.  Since this is generally clumsy to work with, the `dimtypes::dimens` module provides type definitions for most quantities of interest.  For example `dimtypes::dimens::Length` aliases `Quantity<0,1,0,0,0>`, `dimtypes::dimens::Force` aliases `Quantity<-2,1,1,0,0>`, etc.

Internally, `Quantity` wraps a single `f64` value representing the physical quantity in SI base units.  This ensures math between instances of Quantity always follows a consistent unit system.  The magic happens with the implementation of mathematical operations on `Quantity` types:

* `Quantity` implements `Add` and `Sub` only for instances of the same variant, allowing rust to verify at compile time that additions are only performed between compatible quantites
* The `Mul` and `Div` implementations for pairs of `Quantity` variants produce an `Output` type with const parameters equal to the sum or difference of the input const parameters
* `pow::<N>()` and `root::<N>()` are implemented as generic functions such that their output types are variable based on the power or root performed.  `root::<N>()` will only work for types where all the dimension powers divide evenly by N.

The value of a `Quantity` can be extracted in any compatible unit using the `as_unit()` function.  A "compatible unit" is an implementor of the `dimtypes::Unit` trait with the internal `Dimen` type the same as the `Quantity` being converted.  Generally this is simply another `Quantity` of the same dimension where the conversion is found by division, however other implementations can be used for unit systems which are nonlinear (notably Celsius, Fahrenheit, Decibels, etc.).  This package provides `OffsetUnit` and `LogUnit` types for some of these cases.

The `dimtypes::units` module provides constant definitions for many common units.  SI prefixes are implmented as unitless scaling factors and so can be applied to any linear unit through multiplication (e.g. `KILO*GRAM`, `MICRO*FARAD`).  `dimtypes::consts` also provides unit-aware versions of selected physical constants.

Some examples (assuming `use dimtypes::units::*`, `use dimtypes::dimens::*`, and `use dimtypes::consts`)

```rust
// Basic example of unit-aware addition and result conversion
let total_length = 25.0*METER + 100.0*FOOT;
println!("{:.3}",total_length.as_unit(YARD));
// 60.674
```

```rust
// How much does that 190lb man weigh in metric...
println!("{:.3}",(190.0*POUND_FORCE).as_unit(KILO*GRAM));
// Fails to compile!  Kilograms measure mass (Quantity<0,0,1,0,0>), but we provided
// a weight (Force; Quantity<-2, 1, 1, 0, 0>)
/*
error[E0271]: type mismatch resolving `<Quantity<0, 0, 1, 0, 0> as Unit>::Dimen == Quantity<-2, 1, 1, 0, 0>`
  --> src\main.rs:21:50
   |
21 |     println!("{:.3}",(100.0*POUND_FORCE).as_unit(KILO*GRAM));
   |                                          ------- ^^^^^^^^^ expected `-2`, found `0`
   |                                          |
   |                                          required by a bound introduced by this call
   |
   = note: expected struct `Quantity<-2, 1, _, _, _>`
              found struct `Quantity<0, 0, _, _, _>`
*/

//Fixed using correct unit of pound-mass
//(could also convert to KILO*GRAM*consts::STANDARD_GRAVITY, which would be kilogram-force)
println!("{:.3}",(190.0*POUND_MASS).as_unit(KILO*GRAM));
// 86.183

```

```rust
// Let's find the total kinetic and potential energy of an object
fn total_energy(speed: Velocity, mass: Mass, height: Length) -> Energy {
	0.5*mass*speed + mass*consts::STANDARD_GRAVITY*height
}
// Oops, we forgot to square the speed.  Luckily the compiler is watching out for us! 
// It complains both about adding incompatible units and not matching the return type.
/*
error[E0308]: mismatched types
  --> src\main.rs:12:22
   |
12 |     0.5*mass*speed + mass*dimtypes::consts::STANDARD_GRAVITY*height
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `-1`, found `-2`
   |
   = note: expected struct `Quantity<-1, 1, _, _, _>`
              found struct `Quantity<-2, 2, _, _, _>`

error[E0308]: mismatched types
  --> src\main.rs:12:5
   |
11 | fn total_energy(speed: Velocity, mass: Mass, height: Length) -> Energy {
   |                                                                 ------ expected `Quantity<-2, 2, 1, 0, 0>` because of return type
12 |     0.5*mass*speed + mass*dimtypes::consts::STANDARD_GRAVITY*height
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `-2`, found `-1`
   |
   = note: expected struct `Quantity<-2, 2, _, _, _>`
              found struct `Quantity<-1, 1, _, _, _>`

*/

// Lets fix it with pow just to demonstrate it
fn total_energy(speed: Velocity, mass: Mass, height: Length) -> Energy {
    0.5*mass*speed.pow::<2>() + mass*consts::STANDARD_GRAVITY*height
}
(...)
let result = total_energy(10.0*MILE/HOUR,2500.0*KILO*GRAM,1.0*FURLONG);
println!("{:.6e}",result); // Quantities format themselves with their SI base units
// 4.956941e6 kg m^2 s^-2
println!("{:.4} kWh",result.as_unit(KILO*WATT*HOUR)); // Or we can convert to other units
// 1.3769 kWh

```
