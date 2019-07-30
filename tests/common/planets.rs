extern crate posidonius;

pub fn basic_configuration(star: &posidonius::Particle) -> Vec<posidonius::Particle> {
    let planet1_mass_factor: f64 = 1.0;
    let planet1_mass: f64 = planet1_mass_factor * posidonius::constants::M_EARTH; // Solar masses (3.0e-6 solar masses = 1 earth mass)
    let planet1_evolution_type = posidonius::EvolutionType::NonEvolving;
    let planet1_semimajor_axis = 0.018;
    let planet1 = earth_like(&star, planet1_mass, planet1_evolution_type, planet1_semimajor_axis);

    let planet2_mass_factor: f64 = 0.00095; // Earth masses
    let planet2_mass: f64 = planet2_mass_factor * posidonius::constants::M_EARTH; // Solar masses (3.0e-6 solar masses = 1 earth mass)
    let planet2_evolution_type = posidonius::EvolutionType::LeconteChabrier2013;
    let planet2_semimajor_axis = 0.18;
    let planet2 = jupiter_like(&star, planet2_mass, planet2_evolution_type, planet2_semimajor_axis);

    let planet3_mass_factor: f64 = 0.00095; // Earth masses
    let planet3_mass: f64 = planet3_mass_factor * posidonius::constants::M_EARTH; // Solar masses (3.0e-6 solar masses = 1 earth mass)
    let planet3_evolution_type = posidonius::EvolutionType::NonEvolving;
    let planet3_semimajor_axis = 1.8;
    let planet3 = jupiter_like(&star, planet3_mass, planet3_evolution_type, planet3_semimajor_axis);
    return vec![planet1, planet2, planet3];
}

pub fn earth_like(star: &posidonius::Particle, planet_mass: f64, planet_evolution_type: posidonius::EvolutionType, semimajor_axis: f64) -> posidonius::Particle {
    match planet_evolution_type {
        posidonius::EvolutionType::NonEvolving => { },
        _ => { panic!("Evolution type should be non evolving to create a terrestrial/earth-like planet!"); }
    }
    ////// Planetary radius in AU (rearth in AU) Rocky planet
    // Earth-like => mass-radius relationship from Fortney 2007
    let planet_percent_rock = 0.70;
    let planet_mass_factor: f64 = planet_mass / posidonius::constants::M_EARTH;
    let planet_radius_factor : f64 = (0.0592*planet_percent_rock+0.0975) * planet_mass_factor.log10().powi(2) 
                             + (0.2337*planet_percent_rock+0.4938) * planet_mass_factor.log10() 
                             + 0.3102*planet_percent_rock+0.7932;
    //let planet_radius_factor : f64 = 0.00004307581066392812;
    let planet_radius: f64 = planet_radius_factor * posidonius::constants::R_EARTH;
    let planet_love_number: f64 = 0.299; // Earth
    let planet_fluid_love_number: f64 = 0.9532; // Earth
    ////// Disipation factor (sigma)
    let planet_dissipation_factor_scale: f64 = 1.;
    //// Hot Gas Giant:
    //let planet_dissipation_factor: f64 = 2.006*3.845764d4;
    //// Terrestrial:
    let k2pdelta: f64 = 2.465278e-3; // Terrestrial planets (no gas)
    let planet_dissipation_factor: f64 = 2. * posidonius::constants::K2 * k2pdelta/(3. * planet_radius.powi(5));
    ////// Radius of gyration
    let planet_radius_of_gyration_2: f64 = 3.308e-1; // Earth type planet

    ////////// Specify initial position and velocity for a stable orbit
    ////// Keplerian orbital elements, in the `asteroidal' format of Mercury code
    //let a: f64 = 0.018;                             // semi-major axis (in AU)
    let a: f64 = semimajor_axis;                             // semi-major axis (in AU)
    let e: f64 = 0.1;                               // eccentricity
    let i: f64 = 5. * posidonius::constants::DEG2RAD;                      // inclination (degrees)
    let mut p: f64 = 0. * posidonius::constants::DEG2RAD;                            // argument of pericentre (degrees)
    let n: f64 = 0. * posidonius::constants::DEG2RAD;                      // longitude of the ascending node (degrees)
    let l: f64 = 0. * posidonius::constants::DEG2RAD;                      // mean anomaly (degrees)
    p = p + n;                          // Convert to longitude of perihelion !!
    let q = a * (1.0 - e);                          // perihelion distance
    let gm: f64 = posidonius::constants::G*(planet_mass+star.mass);
    let (x, y, z, vx, vy, vz) = posidonius::tools::calculate_cartesian_coordinates(gm, q, e, i, p, n, l);

    //////// Cartesian coordinates
    //let gm: f64 = posidonius::constants::G*(planet_mass+star.mass);
    //let x: f64 = 0.1;
    //let y: f64 = 0.;
    //let z: f64 = 0.;
    //let vx: f64 = 0.;
    //let vy: f64 = core::num::Float::sqrt(gm/x); // Stable orbit
    //let vz: f64 = 0.;

    let planet_position = posidonius::Axes{x:x, y:y, z:z};
    let planet_velocity = posidonius::Axes{x:vx, y:vy, z:vz};
    let planet_acceleration = posidonius::Axes{x:0., y:0., z:0.};

    ////// Initialization of planetary spin
    let planet_obliquity: f64 = 11.459156 * posidonius::constants::DEG2RAD; // 0.2 rad
    let planet_rotation_period: f64 = 24.; // hours
    let planet_angular_frequency = posidonius::constants::TWO_PI/(planet_rotation_period/24.); // days^-1
    //
    let planet_keplerian_orbital_elements = posidonius::tools::calculate_keplerian_orbital_elements(posidonius::constants::G*star.mass*planet_mass, planet_position, planet_velocity);
    let planet_inclination = planet_keplerian_orbital_elements.3;
    let planet_spin = posidonius::tools::calculate_spin(planet_angular_frequency, planet_inclination, planet_obliquity);

    let planet_wind_k_factor = 0.;
    let planet_wind_rotation_saturation = 0.;
    let planet = posidonius::Particle::new(planet_mass, planet_radius, planet_dissipation_factor, planet_dissipation_factor_scale, 
                                            planet_radius_of_gyration_2, planet_love_number, planet_fluid_love_number,
                                            planet_position, planet_velocity, planet_acceleration, planet_spin, 
                                            planet_evolution_type, planet_wind_k_factor, planet_wind_rotation_saturation);
    planet
}

pub fn jupiter_like(star: &posidonius::Particle, planet_mass: f64, planet_evolution_type: posidonius::EvolutionType, semimajor_axis: f64) -> posidonius::Particle {
    match planet_evolution_type {
        posidonius::EvolutionType::LeconteChabrier2013 => { },
        posidonius::EvolutionType::NonEvolving => { },
        _ => { panic!("Evolution type should be LeconteChabrier2013 or non evolving to create a gaseous/jupiter-like planet!"); }
    }

    // Planetary radius in AU (rearth in AU)
    let planet_radius_factor: f64 = 10.9;
    let planet_radius: f64 = planet_radius_factor * posidonius::constants::R_EARTH;
    let planet_love_number: f64 = 0.380; // Gas
    let planet_fluid_love_number: f64 = planet_love_number;
    // Disipation factor (sigma)
    let planet_dissipation_factor_scale: f64 = 1.;
    // Hot Gas Giant:
    //let k2pdelta: f64 = 8.101852e-9; // Gas giant
    let k2pdelta: f64 = 2.893519e-7; // Gas giant for Jupiter: 2-3d-2 s, here in day (Leconte)
    let planet_dissipation_factor: f64 = 2. * posidonius::constants::K2 * k2pdelta/(3. * planet_radius.powi(5));
    //let planet_dissipation_factor: f64 = 2.006*3.845764d4; // Gas giant
    // Radius of gyration
    let planet_radius_of_gyration_2: f64 = 2.54e-1; // Gas planet

    ////////// Specify initial position and velocity for a stable orbit
    ////// Keplerian orbital elements, in the `asteroidal' format of Mercury code
    //let a: f64 = 0.018;                             // semi-major axis (in AU)
    let a: f64 = semimajor_axis;                             // semi-major axis (in AU)
    let e: f64 = 0.1;                               // eccentricity
    let i: f64 = 5. * posidonius::constants::DEG2RAD;                      // inclination (degrees)
    let mut p: f64 = 0. * posidonius::constants::DEG2RAD;                            // argument of pericentre (degrees)
    let n: f64 = 0. * posidonius::constants::DEG2RAD;                      // longitude of the ascending node (degrees)
    let l: f64 = 0. * posidonius::constants::DEG2RAD;                      // mean anomaly (degrees)
    p = p + n;                          // Convert to longitude of perihelion !!
    let q = a * (1.0 - e);                          // perihelion distance
    let gm: f64 = posidonius::constants::G*(planet_mass+star.mass);
    let (x, y, z, vx, vy, vz) = posidonius::tools::calculate_cartesian_coordinates(gm, q, e, i, p, n, l);

    //////// Cartesian coordinates
    //let gm: f64 = posidonius::constants::G*(planet_mass+star.mass);
    //let x: f64 = 0.1;
    //let y: f64 = 0.;
    //let z: f64 = 0.;
    //let vx: f64 = 0.;
    //let vy: f64 = core::num::Float::sqrt(gm/x); // Stable orbit
    //let vz: f64 = 0.;

    let planet_position = posidonius::Axes{x:x, y:y, z:z};
    let planet_velocity = posidonius::Axes{x:vx, y:vy, z:vz};
    let planet_acceleration = posidonius::Axes{x:0., y:0., z:0.};
    
    ////// Initialization of planetary spin
    let planet_obliquity: f64 = 11.459156 * posidonius::constants::DEG2RAD; // 0.2 rad
    let planet_rotation_period: f64 = 24.; // hours
    let planet_angular_frequency = posidonius::constants::TWO_PI/(planet_rotation_period/24.); // days^-1
    //
    let planet_keplerian_orbital_elements = posidonius::tools::calculate_keplerian_orbital_elements(posidonius::constants::G*star.mass*planet_mass, planet_position, planet_velocity);
    let planet_inclination = planet_keplerian_orbital_elements.3;
    let planet_spin = posidonius::tools::calculate_spin(planet_angular_frequency, planet_inclination, planet_obliquity);

    let planet_wind_k_factor = 0.;
    let planet_wind_rotation_saturation = 0.;
    let planet = posidonius::Particle::new(planet_mass, planet_radius, planet_dissipation_factor, planet_dissipation_factor_scale, 
                                            planet_radius_of_gyration_2, planet_love_number, planet_fluid_love_number,
                                            planet_position, planet_velocity, planet_acceleration, planet_spin, 
                                            planet_evolution_type, planet_wind_k_factor, planet_wind_rotation_saturation);
    planet
}
