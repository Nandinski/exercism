// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s)
    }
}

pub trait Planet {
    const EARTH_YEAR_SECONDS: f64 = 31_557_600.0;
    const ORBITAL_PERIOD_IN_EARTH_YEARS: f64;

    fn years_during(d: &Duration) -> f64 {
        ((d.0 as f64) / Self::EARTH_YEAR_SECONDS) / Self::ORBITAL_PERIOD_IN_EARTH_YEARS
    }
}

macro_rules! create_planet {
    ($planet_name:ident, $orbital_period_in_earth_years:expr) => {
        pub struct $planet_name;

        impl Planet for $planet_name {
            const ORBITAL_PERIOD_IN_EARTH_YEARS: f64 = $orbital_period_in_earth_years;
        }
    };
}

create_planet!(Mercury, 0.2408467);
create_planet!(Venus, 0.61519726);
create_planet!(Earth, 1.0);
create_planet!(Mars, 1.8808158);
create_planet!(Jupiter, 11.862615);
create_planet!(Saturn, 29.447498);
create_planet!(Uranus, 84.016846);
create_planet!(Neptune, 164.79132);

// What if we wanted to specify a list of planets?
macro_rules! create_planet_vec {
    ($(($planet_name:ident, $opiey:expr)),+) => {
        $(
            create_planet![$planet_name, $opiey];
        )*
    };
}

create_planet_vec![(Pluto, 247.94), (Eris, 559.07)];
