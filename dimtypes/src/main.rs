use dimtypes::consts;
use dimtypes::units::*;
use dimtypes::dimens::*;

fn total_energy(speed: Velocity, mass: Mass, height: Length) -> Energy {
    0.5*mass*speed.pow::<2>() + mass*dimtypes::consts::STANDARD_GRAVITY*height
}

fn main() {

    let total_length = 25.0*METER + 100.0*FOOT;
    println!("{:.3}",total_length.as_unit(YARD));

    println!("{:.3}",(190.0*POUND_MASS).as_unit(KILO*GRAM));

    let result = total_energy(10.0*MILE/HOUR,2500.0*KILO*GRAM,1.0*FURLONG);
    println!("{:.6e}",result);
    println!("{:.4} kWh",result.as_unit(KILO*WATT*HOUR));

    println!("eps_0 = {:.6e}",consts::VACUUM_PERMITTIVITY);
    println!("{:.3} deg C",(212.0*FAHRENHEIT).as_unit(CELSIUS));
    println!("{:.3} deg C",(212.0*FAHRENHEIT-32.0*FAHRENHEIT).as_unit(CELSIUS.as_rel_unit()));
    println!("{:.3} deg F",(0.0*CELSIUS + FAHRENHEIT.rel_qty_of(27.0)).as_unit(FAHRENHEIT));
    println!("{:.3} dBV",(30.0*MILLI*AMPERE * 100.0*OHM).as_unit(amplitude_decibels_vs(1.0*VOLT)));
    println!("{:.3} psia",(15.7*gauge_pressure_in(KILO*PASCAL)).as_unit(PSI))
}

