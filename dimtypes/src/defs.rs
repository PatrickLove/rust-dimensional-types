pub mod dimens {
	use crate::Quantity;
	pub type Unitless =		Quantity<0,0,0,0,0>;
	pub type Time =			Quantity<1,0,0,0,0>;
	pub type Length =		Quantity<0,1,0,0,0>;
	pub type Area =			Quantity<0,2,0,0,0>;
	pub type Volume =		Quantity<0,3,0,0,0>;
	pub type Mass =			Quantity<0,0,1,0,0>;
	pub type Density =		Quantity<0,-3,1,0,0>;
	pub type Current =		Quantity<0,0,0,1,0>;
	pub type Temperature =	Quantity<0,0,0,0,1>;
	pub type Force =		Quantity<-2,1,1,0,0>;
	pub type Pressure =		Quantity<-2,-1,1,0,0>;
	pub type Momentum =		Quantity<-1,1,1,0,0>;
	pub type Velocity =		Quantity<-1,1,0,0,0>;
	pub type Acceleration =	Quantity<-2,1,0,0,0>;
	pub type Energy =		Quantity<-2,2,1,0,0>;
	pub type Power =		Quantity<-3,2,1,0,0>;
	pub type Voltage =		Quantity<-3,2,1,-1,0>;
	pub type Charge =		Quantity<1,0,0,1,0>;
	pub type Resistance =	Quantity<-3,2,1,-2,0>;
	pub type Capacitance =	Quantity<4,-2,-1,2,0>;
	pub type Inductance =	Quantity<-2,2,1,-2,0>;
	pub type MagneticFlux =	Quantity<-2,2,1,-1,0>;
	pub type Frequency =	Quantity<-1,0,0,0,0>;
}

pub mod consts {
	use crate::Quantity;
	use crate::units::*;
	use crate::dimens::*;

	pub const PLANK_CONSTANT: Quantity<-1,2,1,0,0> = Quantity::from_si(6.62607015e-34);
	pub const SPEED_OF_LIGHT: Velocity = 299792458.0 * METER/SECOND;
	pub const ELEMENTARY_CHARGE: Charge = 1.602176634e-19 * COULOMB;
	pub const BOLTZMANN_CONSTANT: Quantity<-2,2,1,0,-1> = Quantity::from_si(1.380649e-23);
	pub const CAESIUM_HYPERFINE: Frequency = 9192631770.0 * HERTZ;

	pub const STANDARD_GRAVITY: Acceleration =  9.80665 * METER/SECOND/SECOND;
	pub const STANDARD_ATMOSPHERE: Pressure = 101325.0 * PASCAL;
	pub const GRAVITIONAL_CONSTANT: Quantity<-2,3,-1,0,0> = Quantity::from_si(6.67430e-11);
	pub const FINE_STRUCTURE_CONSTANT: Unitless = Unitless::from(0.0072973525643);

	pub const VACUUM_PERMITTIVITY: Quantity<4,-3,-1,2,0> = 0.5*ELEMENTARY_CHARGE*ELEMENTARY_CHARGE/FINE_STRUCTURE_CONSTANT/PLANK_CONSTANT/SPEED_OF_LIGHT;
	pub const VACUUM_PERMEABILITY: Quantity<-2,1,1,-2,0> = 2.0*FINE_STRUCTURE_CONSTANT*PLANK_CONSTANT/ELEMENTARY_CHARGE/ELEMENTARY_CHARGE/SPEED_OF_LIGHT;
}

pub mod units {
	use crate::{LogUnit,OffsetSystem};
	use crate::consts;
	use crate::dimens::*;

	// Prefixes
	pub const QUECTO: Unitless = Unitless::from(1.0e-30);
	pub const RONTO: Unitless = Unitless::from(1.0e-27);
	pub const YOCTO: Unitless = Unitless::from(1.0e-24);
	pub const ZEPTO: Unitless = Unitless::from(1.0e-21);
	pub const ATTO: Unitless = Unitless::from(1.0e-18);
	pub const FEMPTO: Unitless = Unitless::from(1.0e-15);
	pub const PICO: Unitless = Unitless::from(1.0e-12);
	pub const NANO: Unitless = Unitless::from(1.0e-9);
	pub const MICRO: Unitless = Unitless::from(1.0e-6);
	pub const MILLI: Unitless = Unitless::from(1.0e-3);
	pub const CENTI: Unitless = Unitless::from(1.0e-2);
	pub const DECI: Unitless = Unitless::from(1.0e-1);

	pub const DECA: Unitless = Unitless::from(1.0e1);
	pub const HECTO: Unitless = Unitless::from(1.0e2);
	pub const KILO: Unitless = Unitless::from(1.0e3);
	pub const MEGA: Unitless = Unitless::from(1.0e6);
	pub const GIGA: Unitless = Unitless::from(1.0e9);
	pub const TERA: Unitless = Unitless::from(1.0e12);
	pub const PETA: Unitless = Unitless::from(1.0e15);
	pub const EXA: Unitless = Unitless::from(1.0e18);
	pub const ZETTA: Unitless = Unitless::from(1.0e21);
	pub const YOTTA: Unitless = Unitless::from(1.0e24);
	pub const RONNA: Unitless = Unitless::from(1.0e27);
	pub const QUETTA: Unitless = Unitless::from(1.0e30);

	pub const NONE: Unitless = Unitless::from(1.0);
	pub const DOZEN: Unitless = Unitless::from(12.0);
	pub const RADIAN: Unitless = Unitless::from(1.0);
	pub const DEGREE: Unitless = Unitless::from(std::f64::consts::PI/180.0);
	pub const MOLE: Unitless = Unitless::from(6.02214076e23);

	// Time Units
	pub const SECOND: Time = Time::from_si(1.0);
	pub const MINUTE: Time = 60.0*SECOND;
	pub const HOUR: Time = 60.0*MINUTE;
	pub const DAY: Time = 24.0*HOUR;
	pub const YEAR: Time = 365.25*DAY;

	pub const HERTZ: Frequency = 1.0/SECOND;

	// Length Units
	pub const METER: Length = Length::from_si(1.0);
	pub const INCH: Length = 2.54*CENTI*METER;
	pub const FOOT: Length = 12.0*INCH;
	pub const YARD: Length = 3.0*FOOT;
	pub const MILE: Length = 5280.0*FOOT;
	pub const FURLONG: Length = 660.0*FOOT;

	// Area Units
	pub const ACRE: Area = 66.0*FOOT*FURLONG;
	pub const HECTARE: Area = 10000.0*METER*METER;
	pub const BARN: Area = 1e-28*METER*METER;

	// Volume Units
	pub const LITER: Volume = 0.001*METER*METER*METER;
	pub const US_BUSHEL: Volume = 2150.42*INCH*INCH*INCH;
	pub const US_GAL: Volume = 231.0*INCH*INCH*INCH;
	pub const US_QUART: Volume = US_GAL/4.0;
	pub const US_PINT: Volume = US_QUART/2.0;
	pub const CUP: Volume = US_PINT/2.0;
	pub const US_FL_OZ: Volume = CUP/8.0;
	pub const US_TBSP: Volume = US_FL_OZ/2.0;
	pub const US_TSP: Volume = US_TBSP/3.0;

	// Mass units
	pub const GRAM: Mass = Mass::from_si(0.001);
	pub const POUND_MASS: Mass = 0.45359237*KILO*GRAM;
	pub const OUNCE_MASS: Mass = POUND_MASS/16.0;
	pub const SLUG: Mass = POUND_FORCE*SECOND*SECOND/FOOT;

	// Force units
	pub const NEWTON: Force = KILO*GRAM*METER/SECOND/SECOND;
	pub const POUNDAL: Force = POUND_MASS*FOOT/SECOND/SECOND;
	pub const POUND_FORCE: Force = consts::STANDARD_GRAVITY*POUND_MASS;

	// Pressure units
	pub const PASCAL: Pressure = NEWTON/METER/METER;
	pub const PSI: Pressure = POUND_FORCE/INCH/INCH;
	pub const BAR: Pressure = 1e5*PASCAL;
	pub const TORR: Pressure = consts::STANDARD_ATMOSPHERE/760.0;
	const DENSITY_HG: Density = 13595.1 * KILO*GRAM/METER/METER/METER;
	pub const IN_HG: Pressure = consts::STANDARD_GRAVITY*DENSITY_HG*INCH;
	pub const MM_HG: Pressure = consts::STANDARD_GRAVITY*DENSITY_HG*MILLI*METER;

	//Energy/power units
	pub const JOULE: Energy = NEWTON*METER;
	pub const WATT: Power = JOULE/SECOND;

	//Electrical Units
	pub const AMPERE: Current = Current::from_si(1.0);
	pub const COULOMB: Charge = AMPERE*SECOND;
	pub const WEBER: MagneticFlux = VOLT*SECOND;
	pub const VOLT: Voltage = JOULE/COULOMB;
	pub const OHM: Resistance = VOLT/AMPERE;
	pub const FARAD: Capacitance = COULOMB/VOLT;
	pub const HENRY: Inductance = WEBER/AMPERE;

	pub const KELVIN: Temperature = Temperature::from_si(1.0);
	pub const RANKINE: Temperature = KELVIN/1.8;

	// Offset and Log systems

	pub const fn gauge_pressure_in(unit: Pressure) -> OffsetSystem<Pressure> {
		OffsetSystem::new(unit,consts::STANDARD_ATMOSPHERE)
	}

	pub const CELSIUS: OffsetSystem<Temperature> = OffsetSystem::new(KELVIN,273.15*KELVIN);
	pub const FAHRENHEIT: OffsetSystem<Temperature> = OffsetSystem::new(RANKINE,CELSIUS.zero_qty()-32.0*RANKINE);
	
	pub const fn power_decibels_vs<Dimen: Copy>(reference: Dimen) -> LogUnit<Dimen> {
		LogUnit::base10(10.0, reference)
	}
	pub const fn amplitude_decibels_vs<Dimen: Copy>(reference: Dimen) -> LogUnit<Dimen> {
		LogUnit::base10(20.0, reference)
	}
	pub const DBM: LogUnit<Power> = power_decibels_vs(MILLI*WATT);
	pub const DECIBEL: LogUnit<Unitless> = power_decibels_vs((1.0).into());
	pub const SPL: LogUnit<Pressure> = amplitude_decibels_vs(20.0*MICRO*PASCAL);
} 