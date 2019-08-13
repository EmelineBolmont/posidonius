use std::iter;
use super::super::constants::{G, SPEED_OF_LIGHT_2, MAX_PARTICLES, DBL_EPSILON_2};
use super::{Particle};
use super::{Axes};
use super::universe::{IgnoreGravityTerms};

pub fn calculate_kidder1995_general_relativity_acceleration(host_particle: &mut Particle, particles: &mut [Particle], more_particles: &mut [Particle]) {
    let mut host_particle = host_particle;
    let mut particles = particles;
    let mut more_particles = more_particles;
    calculate_kidder1995_first_order_general_relativity_acceleration(&mut host_particle, &mut particles, &mut more_particles);
    calculate_kidder1995_second_order_general_relativity_acceleration(&mut host_particle, &mut particles, &mut more_particles);
    calculate_kidder1995_spin_orbit_general_relativity_acceleration_and_dangular_momentum_dt(&mut host_particle, &mut particles, &mut more_particles);
}


fn calculate_kidder1995_first_order_general_relativity_acceleration(host_particle: &mut Particle, particles: &mut [Particle], more_particles: &mut [Particle]) {
    let mut sum_total_general_relativity_acceleration = Axes{x:0., y:0., z:0.};

    for particle in particles.iter_mut().chain(more_particles.iter_mut()) {
        // Radial part of the GR force (Kidder 1995, Mardling & Lin 2002)
        // - Equation 11 from Bolmont et al. 2015
        let star_planet_mass_g = host_particle.mass_g + particle.mass_g;
        let distance_2 = particle.distance.powi(2);
        let radial_velocity_2 = particle.radial_velocity.powi(2);
        let radial_component_of_the_general_relativity_force = -star_planet_mass_g / (distance_2 * SPEED_OF_LIGHT_2)
            * ( (1.0 + 3.0 * particle.general_relativity_factor) * particle.norm_velocity_vector_2
            -2.0 * (2.0 + particle.general_relativity_factor) * star_planet_mass_g/particle.distance
            -1.5 * particle.general_relativity_factor * radial_velocity_2);
        //println!("Radial component GR force {:e}", radial_component_of_the_general_relativity_force);
        // Orthoradial part of the GR force
        // - Equation 11 from Bolmont et al. 2015
        let orthogonal_component_of_the_general_relativity_force = star_planet_mass_g / (distance_2 * SPEED_OF_LIGHT_2)
            * 2.0 * (2.0 - particle.general_relativity_factor) * particle.radial_velocity * particle.norm_velocity_vector;
        //println!("Ortho component GR force {:e}", orthogonal_component_of_the_general_relativity_force);
        // Total General Relativity force
        // - Equation 10 from Bolmont et al. 2015
        let total_general_relativity_acceleration_x = radial_component_of_the_general_relativity_force * particle.position.x / particle.distance
                + orthogonal_component_of_the_general_relativity_force * particle.velocity.x / particle.norm_velocity_vector;
        let total_general_relativity_acceleration_y = radial_component_of_the_general_relativity_force * particle.position.y / particle.distance
                + orthogonal_component_of_the_general_relativity_force * particle.velocity.y / particle.norm_velocity_vector;
        let total_general_relativity_acceleration_z = radial_component_of_the_general_relativity_force * particle.position.z / particle.distance
                + orthogonal_component_of_the_general_relativity_force * particle.velocity.z / particle.norm_velocity_vector;
        
        sum_total_general_relativity_acceleration.x += particle.mass/host_particle.mass * total_general_relativity_acceleration_x;
        sum_total_general_relativity_acceleration.y += particle.mass/host_particle.mass * total_general_relativity_acceleration_y;
        sum_total_general_relativity_acceleration.z += particle.mass/host_particle.mass * total_general_relativity_acceleration_z;

        // - Equation 19 from Bolmont et al. 2015 (first term)
        particle.general_relativity_acceleration.x = total_general_relativity_acceleration_x;
        particle.general_relativity_acceleration.y = total_general_relativity_acceleration_y;
        particle.general_relativity_acceleration.z = total_general_relativity_acceleration_z;
        //println!("GR force {:e} {:e} {:e}", total_general_relativity_acceleration_x,
        //total_general_relativity_acceleration_y, total_general_relativity_acceleration_z);
    }
    
    // - Equation 19 from Bolmont et al. 2015 (second term)
    //for particle in particles.iter_mut() {
        //particle.general_relativity_acceleration.x += sum_total_general_relativity_acceleration.x;
        //particle.general_relativity_acceleration.y += sum_total_general_relativity_acceleration.y;
        //particle.general_relativity_acceleration.z += sum_total_general_relativity_acceleration.z;
    //}
    // Instead of the previous code, keep star tidal acceleration separated:
    host_particle.general_relativity_acceleration.x = -1.0 * sum_total_general_relativity_acceleration.x;
    host_particle.general_relativity_acceleration.y = -1.0 * sum_total_general_relativity_acceleration.y;
    host_particle.general_relativity_acceleration.z = -1.0 * sum_total_general_relativity_acceleration.z;
}

fn calculate_kidder1995_second_order_general_relativity_acceleration(host_particle: &mut Particle, particles: &mut [Particle], more_particles: &mut [Particle]) {
    // 2nd order Post-Newtonian
    let mut sum_total_second_order_general_relativity_acceleration = Axes{x:0., y:0., z:0.};

    for particle in particles.iter_mut().chain(more_particles.iter_mut()) {
        let star_planet_mass_g = host_particle.mass_g + particle.mass_g;
        let distance_2 = particle.distance.powi(2);
        let norm_velocity_vector_2 = particle.norm_velocity_vector.powi(2);
        let norm_velocity_vector_4 = norm_velocity_vector_2.powi(2);
        let radial_velocity_2 = particle.radial_velocity.powi(2);
        let radial_velocity_4 = radial_velocity_2.powi(2);
        let general_relativity_factor_2 = particle.general_relativity_factor.powi(2);

        // Radial part of the GR force (Kidder 1995, equation 2.2d)
        let radial_component_of_the_second_order_general_relativity_acceleration = -star_planet_mass_g / (distance_2 * SPEED_OF_LIGHT_2)
                * (3.0/4.0*(12.0+29.0*particle.general_relativity_factor)*(star_planet_mass_g.powi(2)/distance_2)
                + particle.general_relativity_factor*(3.0-4.0*particle.general_relativity_factor)*norm_velocity_vector_4
                + 15.0/8.0*particle.general_relativity_factor*(1.0-3.0*particle.general_relativity_factor)*radial_velocity_4
                - 3.0/2.0*particle.general_relativity_factor*(3.0-4.0*particle.general_relativity_factor)*radial_velocity_2*norm_velocity_vector_2
                - 0.5*particle.general_relativity_factor*(13.0-4.0*particle.general_relativity_factor)*(star_planet_mass_g/particle.distance)*norm_velocity_vector_2
                - (2.0 + 25.0*particle.general_relativity_factor+2.0*general_relativity_factor_2)*(star_planet_mass_g/particle.distance)*radial_velocity_2);

        let orthogonal_component_of_the_second_order_general_relativity_acceleration = -star_planet_mass_g / (distance_2 * SPEED_OF_LIGHT_2)
                * (-0.5)*particle.radial_velocity
                * (particle.general_relativity_factor*(15.0+4.0*particle.general_relativity_factor)*norm_velocity_vector_2 
                - (4.0+41.0*particle.general_relativity_factor+8.0*general_relativity_factor_2)*(star_planet_mass_g/particle.distance)
                - 3.0*particle.general_relativity_factor*(3.0+2.0*particle.general_relativity_factor)*radial_velocity_2);

        let total_second_order_general_relativity_acceleration_x = radial_component_of_the_second_order_general_relativity_acceleration * particle.position.x / particle.distance
                + orthogonal_component_of_the_second_order_general_relativity_acceleration * particle.velocity.x;
        let total_second_order_general_relativity_acceleration_y = radial_component_of_the_second_order_general_relativity_acceleration * particle.position.y / particle.distance
                + orthogonal_component_of_the_second_order_general_relativity_acceleration * particle.velocity.y;
        let total_second_order_general_relativity_acceleration_z = radial_component_of_the_second_order_general_relativity_acceleration * particle.position.z / particle.distance
                + orthogonal_component_of_the_second_order_general_relativity_acceleration * particle.velocity.z;
        
        sum_total_second_order_general_relativity_acceleration.x += particle.mass/host_particle.mass * total_second_order_general_relativity_acceleration_x;
        sum_total_second_order_general_relativity_acceleration.y += particle.mass/host_particle.mass * total_second_order_general_relativity_acceleration_y;
        sum_total_second_order_general_relativity_acceleration.z += particle.mass/host_particle.mass * total_second_order_general_relativity_acceleration_z;

        particle.general_relativity_acceleration.x += total_second_order_general_relativity_acceleration_x;
        particle.general_relativity_acceleration.y += total_second_order_general_relativity_acceleration_y;
        particle.general_relativity_acceleration.z += total_second_order_general_relativity_acceleration_z;
        //println!("a {} {} {}", total_second_order_general_relativity_acceleration_x, total_second_order_general_relativity_acceleration_y, total_second_order_general_relativity_acceleration_z);
    }
    
    // - Equation 19 from Bolmont et al. 2015 (second term)
    //for particle in particles.iter_mut() {
        //particle.general_relativity_acceleration.x += sum_total_second_order_general_relativity_acceleration.x;
        //particle.general_relativity_acceleration.y += sum_total_second_order_general_relativity_acceleration.y;
        //particle.general_relativity_acceleration.z += sum_total_second_order_general_relativity_acceleration.z;
    //}
    // Instead of the previous code, keep star tidal acceleration separated:
    host_particle.general_relativity_acceleration.x += -1.0 * sum_total_second_order_general_relativity_acceleration.x;
    host_particle.general_relativity_acceleration.y += -1.0 * sum_total_second_order_general_relativity_acceleration.y;
    host_particle.general_relativity_acceleration.z += -1.0 * sum_total_second_order_general_relativity_acceleration.z;
}

fn calculate_kidder1995_spin_orbit_general_relativity_acceleration_and_dangular_momentum_dt(host_particle: &mut Particle, particles: &mut [Particle], more_particles: &mut [Particle]) {
    // - Equation 5 from https://arxiv.org/pdf/1102.5192.pdf
    // Spin effects are known for the dominant relativistic spin-orbit coupling term at 1.5PN
    // https://arxiv.org/pdf/gr-qc/0202016.pdf
    // Spin in Kidder is defined as angular_momentum
    let star_angular_momentum = Axes{
                                        x:host_particle.moment_of_inertia*host_particle.spin.x,
                                        y:host_particle.moment_of_inertia*host_particle.spin.y,
                                        z:host_particle.moment_of_inertia*host_particle.spin.z
    };
    let mut sum_total_general_relativity_spin_orbit_acceleration = Axes{x:0., y:0., z:0.};

    host_particle.dangular_momentum_dt_due_to_general_relativity.x = 0.;
    host_particle.dangular_momentum_dt_due_to_general_relativity.y = 0.;
    host_particle.dangular_momentum_dt_due_to_general_relativity.z = 0.;

    for particle in particles.iter_mut().chain(more_particles.iter_mut()) {
        // - Equation 2.2c from Kidder 1995
        let star_planet_mass = host_particle.mass + particle.mass;
        let star_planet_diff_mass = host_particle.mass - particle.mass;
        let mass_factor = star_planet_diff_mass / star_planet_mass;

        // Spin in Kidder is defined as angular_momentum
        let particle_angular_momentum = Axes{
                                            x:particle.moment_of_inertia*particle.spin.x,
                                            y:particle.moment_of_inertia*particle.spin.y,
                                            z:particle.moment_of_inertia*particle.spin.z
        };

        let particle_normalized_position = Axes{
                                            x:particle.position.x/particle.distance,
                                            y:particle.position.y/particle.distance,
                                            z:particle.position.z/particle.distance
        };

        let mass_spin_factor_x = mass_factor*star_planet_mass*(particle_angular_momentum.x/particle.mass - star_angular_momentum.x/host_particle.mass);
        let mass_spin_factor_y = mass_factor*star_planet_mass*(particle_angular_momentum.y/particle.mass - star_angular_momentum.y/host_particle.mass);
        let mass_spin_factor_z = mass_factor*star_planet_mass*(particle_angular_momentum.z/particle.mass - star_angular_momentum.z/host_particle.mass);

        let element1_x: f64 = 6.*particle_normalized_position.x
                           * ((particle_normalized_position.y * particle.velocity.z - particle_normalized_position.z * particle.velocity.y)
                           * (2.*(star_angular_momentum.x+particle_angular_momentum.x) + mass_spin_factor_x));
        let element1_y :f64 = 6.*particle_normalized_position.y
                           * ((particle_normalized_position.z * particle.velocity.x - particle_normalized_position.x * particle.velocity.z)
                           * (2.*(star_angular_momentum.y+particle_angular_momentum.y) + mass_spin_factor_y));
        let element1_z: f64 = 6.*particle_normalized_position.z
                           * ((particle_normalized_position.x * particle.velocity.y - particle_normalized_position.y * particle.velocity.x)
                           * (2.*(star_angular_momentum.z+particle_angular_momentum.z) + mass_spin_factor_z));

        let element7s = Axes{
                                x:7.*(star_angular_momentum.x+particle_angular_momentum.x) + 3.*mass_spin_factor_x,
                                y:7.*(star_angular_momentum.y+particle_angular_momentum.y) + 3.*mass_spin_factor_y,
                                z:7.*(star_angular_momentum.z+particle_angular_momentum.z) + 3.*mass_spin_factor_z
        };
        let element2_x: f64 = particle.velocity.y * element7s.z - particle.velocity.z * element7s.y;
        let element2_y: f64 = particle.velocity.z * element7s.x - particle.velocity.x * element7s.z;
        let element2_z: f64 = particle.velocity.x * element7s.y - particle.velocity.y * element7s.x;

        let element3s = Axes{
                                x:3.*(star_angular_momentum.x+particle_angular_momentum.x) + mass_spin_factor_x,
                                y:3.*(star_angular_momentum.y+particle_angular_momentum.y) + mass_spin_factor_y,
                                z:3.*(star_angular_momentum.z+particle_angular_momentum.z) + mass_spin_factor_z
        };
        let element3_x: f64 = 3.*particle.radial_velocity * (particle_normalized_position.y * element3s.z - particle_normalized_position.z * element3s.y);
        let element3_y: f64 = 3.*particle.radial_velocity * (particle_normalized_position.z * element3s.x - particle_normalized_position.x * element3s.z);
        let element3_z: f64 = 3.*particle.radial_velocity * (particle_normalized_position.x * element3s.y - particle_normalized_position.y * element3s.x);

        let factor_a = G / SPEED_OF_LIGHT_2;
        let total_general_relativity_spin_orbit_acceleration_x = factor_a * (element1_x - element2_x + element3_x);
        let total_general_relativity_spin_orbit_acceleration_y = factor_a * (element1_y - element2_y + element3_y);
        let total_general_relativity_spin_orbit_acceleration_z = factor_a * (element1_z - element2_z + element3_z);

        sum_total_general_relativity_spin_orbit_acceleration.x += particle.mass/host_particle.mass * total_general_relativity_spin_orbit_acceleration_x;
        sum_total_general_relativity_spin_orbit_acceleration.y += particle.mass/host_particle.mass * total_general_relativity_spin_orbit_acceleration_y;
        sum_total_general_relativity_spin_orbit_acceleration.z += particle.mass/host_particle.mass * total_general_relativity_spin_orbit_acceleration_z;

        particle.general_relativity_acceleration.x += total_general_relativity_spin_orbit_acceleration_x;
        particle.general_relativity_acceleration.y += total_general_relativity_spin_orbit_acceleration_y;
        particle.general_relativity_acceleration.z += total_general_relativity_spin_orbit_acceleration_z;
        //println!("{} {} {}", total_general_relativity_spin_orbit_acceleration_x, total_general_relativity_spin_orbit_acceleration_y, total_general_relativity_spin_orbit_acceleration_z);

        // Kidder 1995, equation 2.4a
        let mu = (host_particle.mass * particle.mass) / star_planet_mass;
        let newtonian_orbital_angular_momentum = Axes{
                                            x:mu * (particle.position.y * particle.velocity.z - particle.position.z * particle.velocity.y),
                                            y:mu * (particle.position.z * particle.velocity.x - particle.position.x * particle.velocity.z),
                                            z:mu * (particle.position.x * particle.velocity.y - particle.position.y * particle.velocity.x)
        };

        let factor_mass = 2. + 3./2. * particle.mass/host_particle.mass;
        let element1_x: f64 = factor_mass 
            * (newtonian_orbital_angular_momentum.y * star_angular_momentum.z - newtonian_orbital_angular_momentum.z * star_angular_momentum.y);
        let element1_y: f64 = factor_mass
            * (newtonian_orbital_angular_momentum.z * star_angular_momentum.x - newtonian_orbital_angular_momentum.x * star_angular_momentum.z);
        let element1_z: f64 = factor_mass
            * (newtonian_orbital_angular_momentum.x * star_angular_momentum.y - newtonian_orbital_angular_momentum.y * star_angular_momentum.x);

        let element2_x: f64 = particle_angular_momentum.y * star_angular_momentum.z - particle_angular_momentum.z * star_angular_momentum.y;
        let element2_y :f64 = particle_angular_momentum.z * star_angular_momentum.x - particle_angular_momentum.x * star_angular_momentum.z;
        let element2_z: f64 = particle_angular_momentum.x * star_angular_momentum.y - particle_angular_momentum.y * star_angular_momentum.x;

        let scalar_product_particle_normalized_position_with_particle_angular_momentum = 
            particle_normalized_position.x * particle_angular_momentum.x 
            + particle_normalized_position.y * particle_angular_momentum.y 
            + particle_normalized_position.z * particle_angular_momentum.z;
        let element3_x: f64 = 3. * scalar_product_particle_normalized_position_with_particle_angular_momentum 
            * (particle_normalized_position.y * star_angular_momentum.z - particle_normalized_position.z * star_angular_momentum.y);
        let element3_y :f64 = 3. * scalar_product_particle_normalized_position_with_particle_angular_momentum 
            * (particle_normalized_position.z * star_angular_momentum.x - particle_normalized_position.x * star_angular_momentum.z);
        let element3_z: f64 = 3. * scalar_product_particle_normalized_position_with_particle_angular_momentum 
            * (particle_normalized_position.x * star_angular_momentum.y - particle_normalized_position.y * star_angular_momentum.x);
        
        host_particle.dangular_momentum_dt_due_to_general_relativity.x += factor_a * (element1_x - element2_x + element3_x);
        host_particle.dangular_momentum_dt_due_to_general_relativity.y += factor_a * (element1_y - element2_y + element3_y);
        host_particle.dangular_momentum_dt_due_to_general_relativity.z += factor_a * (element1_z - element2_z + element3_z);
        //println!("{} {} {}", factor_a * (element1_x - element2_x + element3_x), factor_a * (element1_y - element2_y + element3_y), factor_a * (element1_z - element2_z + element3_z));
        
        // Kidder 1995, equation 2.4b
        let factor_mass = 2. + 3./2. * host_particle.mass/particle.mass;
        let element1_x: f64 = factor_mass 
            * (newtonian_orbital_angular_momentum.y * particle_angular_momentum.z - newtonian_orbital_angular_momentum.z * particle_angular_momentum.y);
        let element1_y: f64 = factor_mass
            * (newtonian_orbital_angular_momentum.z * particle_angular_momentum.x - newtonian_orbital_angular_momentum.x * particle_angular_momentum.z);
        let element1_z: f64 = factor_mass
            * (newtonian_orbital_angular_momentum.x * particle_angular_momentum.y - newtonian_orbital_angular_momentum.y * particle_angular_momentum.x);

        let element2_x: f64 = star_angular_momentum.y * particle_angular_momentum.z - star_angular_momentum.z * particle_angular_momentum.y;
        let element2_y :f64 = star_angular_momentum.z * particle_angular_momentum.x - star_angular_momentum.x * particle_angular_momentum.z;
        let element2_z: f64 = star_angular_momentum.x * particle_angular_momentum.y - star_angular_momentum.y * particle_angular_momentum.x;

        let scalar_product_particle_normalized_position_with_star_angular_momentum = 
            particle_normalized_position.x * star_angular_momentum.x 
            + particle_normalized_position.y * star_angular_momentum.y 
            + particle_normalized_position.z * star_angular_momentum.z;
        let element3_x: f64 = 3. * scalar_product_particle_normalized_position_with_star_angular_momentum 
            * (particle_normalized_position.y*particle_angular_momentum.z - particle_normalized_position.z*particle_angular_momentum.y);
        let element3_y :f64 = 3. * scalar_product_particle_normalized_position_with_star_angular_momentum 
            * (particle_normalized_position.z*particle_angular_momentum.x - particle_normalized_position.x*particle_angular_momentum.z);
        let element3_z: f64 = 3. * scalar_product_particle_normalized_position_with_star_angular_momentum
            * (particle_normalized_position.x*particle_angular_momentum.y - particle_normalized_position.y*particle_angular_momentum.x);
        
        particle.dangular_momentum_dt_due_to_general_relativity.x = factor_a * (element1_x - element2_x + element3_x);
        particle.dangular_momentum_dt_due_to_general_relativity.y = factor_a * (element1_y - element2_y + element3_y);
        particle.dangular_momentum_dt_due_to_general_relativity.z = factor_a * (element1_z - element2_z + element3_z);
    }
    //for particle in particles.iter_mut() {
        //particle.general_relativity_acceleration.x += sum_total_general_relativity_spin_orbit_acceleration.x;
        //particle.general_relativity_acceleration.y += sum_total_general_relativity_spin_orbit_acceleration.y;
        //particle.general_relativity_acceleration.z += sum_total_general_relativity_spin_orbit_acceleration.z;
    //}
    // Instead of the previous code, keep star tidal acceleration separated:
    host_particle.general_relativity_acceleration.x += -1.0 * sum_total_general_relativity_spin_orbit_acceleration.x;
    host_particle.general_relativity_acceleration.y += -1.0 * sum_total_general_relativity_spin_orbit_acceleration.y;
    host_particle.general_relativity_acceleration.z += -1.0 * sum_total_general_relativity_spin_orbit_acceleration.z;
}

////////////////////////////////////////////////////////////////////////////
//--------------------------------------------------------------------------
// [start] General Relativity based on REBOUNDx gr.c
pub fn calculate_anderson1975_general_relativity_acceleration(host_particle: &mut Particle, particles: &mut [Particle], more_particles: &mut [Particle], ignored_gravity_terms: IgnoreGravityTerms) {
    // Calculate Newtonian accelerations in the current setup and considering all particles
    let mut host_particle = host_particle;
    let mut particles = particles;
    let mut more_particles = more_particles;
    let (host_newtonian_inertial_accelerations, newtonian_inertial_accelerations) = get_anderson1975_newhall1983_newtonian_inertial_accelerations(&mut host_particle, &mut particles, &mut more_particles, ignored_gravity_terms);

    // Transform to Jacobi coordinates
    let (jacobi_star_mass, _jacobi_star_position, _jacobi_star_velocity, _jacobi_star_acceleration, jacobi_particles_positions, jacobi_particles_velocities, mut jacobi_particles_accelerations) = anderson1975_general_relativity_inertial_to_jacobi_posvelacc(&host_particle, &particles, &more_particles, host_newtonian_inertial_accelerations, newtonian_inertial_accelerations);

    let n_particles = particles.len() + more_particles.len();
    let mu = host_particle.mass_g;
    for ((jacobi_particle_acceleration, jacobi_particle_velocity), jacobi_particle_position) in jacobi_particles_accelerations[..n_particles].iter_mut()
                                                                                                    .zip(jacobi_particles_velocities[..n_particles].iter())
                                                                                                    .zip(jacobi_particles_positions[..n_particles].iter()) {
        let mut vi = Axes{x: jacobi_particle_velocity.x, y: jacobi_particle_velocity.y, z: jacobi_particle_velocity.z};
        let mut vi2 = jacobi_particle_velocity.x.powi(2) + jacobi_particle_velocity.y.powi(2) + jacobi_particle_velocity.z.powi(2);
        let ri = (jacobi_particle_position.x.powi(2) + jacobi_particle_position.y.powi(2) + jacobi_particle_position.z.powi(2)).sqrt();
        let mut factor_a = (0.5*vi2 + 3.*mu/ri)/SPEED_OF_LIGHT_2;
        let mut old_v = Axes{x:0., y:0., z:0.};

        let max_iterations = 10;
        for q in 0..max_iterations {
            old_v.x = vi.x;
            old_v.y = vi.y;
            old_v.z = vi.z;
            vi.x = jacobi_particle_velocity.x/(1.-factor_a);
            vi.y = jacobi_particle_velocity.y/(1.-factor_a);
            vi.z = jacobi_particle_velocity.z/(1.-factor_a);
            vi2 =vi.x*vi.x + vi.y*vi.y + vi.z*vi.z;
            factor_a = (0.5*vi2 + 3.*mu/ri)/SPEED_OF_LIGHT_2;
            let dvx = vi.x - old_v.x;
            let dvy = vi.y - old_v.y;
            let dvz = vi.z - old_v.z;
            if (dvx*dvx + dvy*dvy + dvz*dvz)/vi2 < DBL_EPSILON_2 {
                break;
            } else if q == max_iterations {
                println!("[WARNING {} UTC] {} iterations in general relativity failed to converge. This is typically because the perturbation is too strong for the current implementation.", time::now_utc().strftime("%Y.%m.%d %H:%M:%S").unwrap(), max_iterations);
            }
        }

        let factor_b = (mu/ri - 1.5*vi2)*mu/(ri*ri*ri)/SPEED_OF_LIGHT_2;
        let rdotrdot = jacobi_particle_position.x*jacobi_particle_velocity.x
                        + jacobi_particle_position.y*jacobi_particle_velocity.y
                        + jacobi_particle_position.z*jacobi_particle_velocity.z;
        let vidot = Axes{x: jacobi_particle_acceleration.x + factor_b*jacobi_particle_position.x,
                            y: jacobi_particle_acceleration.y + factor_b*jacobi_particle_position.y,
                            z: jacobi_particle_acceleration.z + factor_b*jacobi_particle_position.z};
        let vdotvdot = vi.x*vidot.x + vi.y*vidot.y + vi.z*vidot.z;
        let factor_d = (vdotvdot - 3.*mu/(ri*ri*ri)*rdotrdot)/SPEED_OF_LIGHT_2;
        jacobi_particle_acceleration.x = factor_b*(1.-factor_a)*jacobi_particle_position.x - factor_a*jacobi_particle_acceleration.x - factor_d*vi.x;
        jacobi_particle_acceleration.y = factor_b*(1.-factor_a)*jacobi_particle_position.y - factor_a*jacobi_particle_acceleration.y - factor_d*vi.y;
        jacobi_particle_acceleration.z = factor_b*(1.-factor_a)*jacobi_particle_position.z - factor_a*jacobi_particle_acceleration.z - factor_d*vi.z;
    }


    let jacobi_star_acceleration = Axes{x:0., y:0., z:0.};
    let (star_acceleration, particles_accelerations) = anderson1975_general_relativity_jacobi_to_inertial_acc(&mut particles, &mut more_particles, jacobi_star_mass, jacobi_star_acceleration, jacobi_particles_accelerations);


    // This algorithm computes general_relativity_acceleration in the inertial frame,
    // which is the same coordinate system that is expressed all the rest of additional
    // effects
    for (particle, particle_acceleration) in particles.iter_mut().chain(more_particles.iter_mut()).zip(particles_accelerations.iter()) {
        particle.general_relativity_acceleration.x = particle_acceleration.x;
        particle.general_relativity_acceleration.y = particle_acceleration.y;
        particle.general_relativity_acceleration.z = particle_acceleration.z;
    }
    host_particle.general_relativity_acceleration.x = star_acceleration.x;
    host_particle.general_relativity_acceleration.y = star_acceleration.y;
    host_particle.general_relativity_acceleration.z = star_acceleration.z;
}

fn anderson1975_general_relativity_inertial_to_jacobi_posvelacc(host_particle: &Particle, particles: &[Particle], more_particles: &[Particle], host_newtonian_inertial_accelerations: Axes, newtonian_inertial_accelerations: [Axes; MAX_PARTICLES-1]) -> (f64, Axes, Axes, Axes, [Axes; MAX_PARTICLES-1], [Axes; MAX_PARTICLES-1], [Axes; MAX_PARTICLES-1]) {
    let jacobi_star_mass;
    let mut jacobi_star_position = Axes{x:0., y:0., z:0. };
    let mut jacobi_star_velocity = Axes{x:0., y:0., z:0. };
    let mut jacobi_star_acceleration = Axes{x:0., y:0., z:0. };
    let mut jacobi_particles_positions = [Axes{x:0., y:0., z:0. }; MAX_PARTICLES-1];
    let mut jacobi_particles_velocities = [Axes{x:0., y:0., z:0. }; MAX_PARTICLES-1];
    let mut jacobi_particles_accelerations = [Axes{x:0., y:0., z:0. }; MAX_PARTICLES-1];

    let m0 = host_particle.mass;
    let mut eta = m0;
    let mut s_x = eta * host_particle.inertial_position.x;
    let mut s_y = eta * host_particle.inertial_position.y;
    let mut s_z = eta * host_particle.inertial_position.z;
    let mut s_vx = eta * host_particle.inertial_velocity.x;
    let mut s_vy = eta * host_particle.inertial_velocity.y;
    let mut s_vz = eta * host_particle.inertial_velocity.z;
    let mut s_ax = eta * host_newtonian_inertial_accelerations.x;
    let mut s_ay = eta * host_newtonian_inertial_accelerations.y;
    let mut s_az = eta * host_newtonian_inertial_accelerations.z;
    for ((((particle, particle_newtonian_inertial_accelerations), jacobi_particle_position), jacobi_particle_velocity), jacobi_particle_acceleration) in 
            particles.iter().chain(more_particles.iter()) // zip will pick the lowest common number of elements
                .zip(newtonian_inertial_accelerations.iter())
                .zip(jacobi_particles_positions.iter_mut())
                .zip(jacobi_particles_velocities.iter_mut())
                .zip(jacobi_particles_accelerations.iter_mut()) {
        let ei = 1./eta;
        eta += particle.mass;
        let pme = eta*ei;
        jacobi_particle_position.x = particle.inertial_position.x - s_x*ei;
        jacobi_particle_position.y = particle.inertial_position.y - s_y*ei;
        jacobi_particle_position.z = particle.inertial_position.z - s_z*ei;
        jacobi_particle_velocity.x = particle.inertial_velocity.x - s_vx*ei;
        jacobi_particle_velocity.y = particle.inertial_velocity.y - s_vy*ei;
        jacobi_particle_velocity.z = particle.inertial_velocity.z - s_vz*ei;
        jacobi_particle_acceleration.x = particle_newtonian_inertial_accelerations.x - s_ax*ei;
        jacobi_particle_acceleration.y = particle_newtonian_inertial_accelerations.y - s_ay*ei;
        jacobi_particle_acceleration.z = particle_newtonian_inertial_accelerations.z - s_az*ei;
        s_x  = s_x  * pme + particle.mass*jacobi_particle_position.x ;
        s_y  = s_y  * pme + particle.mass*jacobi_particle_position.y ;
        s_z  = s_z  * pme + particle.mass*jacobi_particle_position.z ;
        s_vx = s_vx * pme + particle.mass*jacobi_particle_velocity.x;
        s_vy = s_vy * pme + particle.mass*jacobi_particle_velocity.y;
        s_vz = s_vz * pme + particle.mass*jacobi_particle_velocity.z;
        s_ax = s_ax * pme + particle.mass*jacobi_particle_acceleration.x;
        s_ay = s_ay * pme + particle.mass*jacobi_particle_acceleration.y;
        s_az = s_az * pme + particle.mass*jacobi_particle_acceleration.z;
    }
    let mtot = eta;
    let mtot_i = 1./mtot;
    jacobi_star_mass = mtot;
    jacobi_star_position.x = s_x * mtot_i;
    jacobi_star_position.y = s_y * mtot_i;
    jacobi_star_position.z = s_z * mtot_i;
    jacobi_star_velocity.x = s_vx * mtot_i;
    jacobi_star_velocity.y = s_vy * mtot_i;
    jacobi_star_velocity.z = s_vz * mtot_i;
    jacobi_star_acceleration.x = s_ax * mtot_i;
    jacobi_star_acceleration.y = s_ay * mtot_i;
    jacobi_star_acceleration.z = s_az * mtot_i;
    return(jacobi_star_mass, jacobi_star_position, jacobi_star_velocity, jacobi_star_acceleration, jacobi_particles_positions, jacobi_particles_velocities, jacobi_particles_accelerations)
}

fn anderson1975_general_relativity_jacobi_to_inertial_acc(particles: &mut [Particle], more_particles: &mut [Particle], jacobi_star_mass: f64, jacobi_star_acceleration: Axes, jacobi_particles_accelerations: [Axes; MAX_PARTICLES-1]) -> (Axes, [Axes; MAX_PARTICLES-1]) {
    let mut star_acceleration = Axes{x:0., y:0., z:0. };
    let mut particles_accelerations = [Axes{x:0., y:0., z:0. }; MAX_PARTICLES-1];
    let n_particles = particles.len() + more_particles.len();

    let mut eta = jacobi_star_mass;
    let mut s_ax = eta * jacobi_star_acceleration.x;
    let mut s_ay = eta * jacobi_star_acceleration.y;
    let mut s_az = eta * jacobi_star_acceleration.z;
    for ((particle, particle_acceleration), jacobi_particle_acceleration) in particles.iter().chain(more_particles.iter()).rev()
                                                                                .zip(particles_accelerations[..n_particles].iter_mut().rev())
                                                                                .zip(jacobi_particles_accelerations[..n_particles].iter().rev()) {
        let ei = 1./eta;
        s_ax = (s_ax - particle.mass*jacobi_particle_acceleration.x) * ei;
        s_ay = (s_ay - particle.mass*jacobi_particle_acceleration.y) * ei;
        s_az = (s_az - particle.mass*jacobi_particle_acceleration.z) * ei;
        particle_acceleration.x = jacobi_particle_acceleration.x + s_ax;
        particle_acceleration.y = jacobi_particle_acceleration.y + s_ay;
        particle_acceleration.z = jacobi_particle_acceleration.z + s_az;
        eta -= particle.mass;
        s_ax *= eta;
        s_ay *= eta;
        s_az *= eta;
    }
    let mtot = eta;
    let mtot_i = 1./mtot;
    star_acceleration.x = s_ax * mtot_i;
    star_acceleration.y = s_ay * mtot_i;
    star_acceleration.z = s_az * mtot_i;
    return(star_acceleration, particles_accelerations)
}
// [end] General Relativity based on REBOUNDx gr.c
//--------------------------------------------------------------------------
////////////////////////////////////////////////////////////////////////////

fn get_anderson1975_newhall1983_newtonian_inertial_accelerations(host_particle: &mut Particle, particles: &mut [Particle], more_particles: &mut [Particle], ignored_gravity_terms: IgnoreGravityTerms) -> (Axes, [Axes; MAX_PARTICLES-1]) {
    let mut host_newtonian_inertial_accelerations = Axes{x:0., y:0., z:0. };
    let mut newtonian_inertial_accelerations = [Axes{x:0., y:0., z:0. }; MAX_PARTICLES-1]; 
    host_newtonian_inertial_accelerations.x = host_particle.inertial_acceleration.x;
    host_newtonian_inertial_accelerations.y = host_particle.inertial_acceleration.y;
    host_newtonian_inertial_accelerations.z = host_particle.inertial_acceleration.z;
    for (newtonian_acceleration, particle) in newtonian_inertial_accelerations.iter_mut().zip(particles.iter_mut().chain(more_particles.iter_mut())) {
        newtonian_acceleration.x = particle.inertial_acceleration.x;
        newtonian_acceleration.y = particle.inertial_acceleration.y;
        newtonian_acceleration.z = particle.inertial_acceleration.z;
    }
    // If some terms where ignored by the integrator, they should be added
    if ignored_gravity_terms == IgnoreGravityTerms::WHFastOne || ignored_gravity_terms == IgnoreGravityTerms::WHFastTwo {
        let n_particles;
        if ignored_gravity_terms == IgnoreGravityTerms::WHFastOne {
            n_particles = 2 - 1; // Only host - next particle interaction, and the host is already out of the particles vector
        } else {
            n_particles = particles.len() + more_particles.len();
        }
        for (newtonian_acceleration, particle) in newtonian_inertial_accelerations[..n_particles].iter_mut().zip(particles.iter_mut().chain(more_particles.iter_mut())) {
            let dx = host_particle.inertial_position.x - particle.position.x;
            let dy = host_particle.inertial_position.y - particle.position.y;
            let dz = host_particle.inertial_position.z - particle.position.z;
            let r2 = dx.powi(2) + dy.powi(2) + dz.powi(2);
            let r = r2.sqrt();
            let prefac = G/(r2*r);
            let prefac_mass_star = prefac*host_particle.mass;
            let prefac_mass_particle = prefac*particle.mass;
            host_newtonian_inertial_accelerations.x -= prefac_mass_particle*dx;
            host_newtonian_inertial_accelerations.y -= prefac_mass_particle*dy;
            host_newtonian_inertial_accelerations.z -= prefac_mass_particle*dz;
            newtonian_acceleration.x += prefac_mass_star*dx;
            newtonian_acceleration.y += prefac_mass_star*dy;
            newtonian_acceleration.z += prefac_mass_star*dz;
        }
    }
    return (host_newtonian_inertial_accelerations, newtonian_inertial_accelerations);
}

////////////////////////////////////////////////////////////////////////////
//--------------------------------------------------------------------------
// [start] General Relativity FULL based on REBOUNDx gr.c
pub fn calculate_newhall1983_general_relativity_acceleration(host_particle: &mut Particle, particles: &mut [Particle], more_particles: &mut [Particle], ignored_gravity_terms: IgnoreGravityTerms) {
    let mut host_a_const = [Axes{x:0., y:0., z:0. }; 1]; // array that stores the value of the constant term
    let mut host_a_new = [Axes{x:0., y:0., z:0. }; 1]; // stores the newly calculated term
    let mut host_rs = [[0.; MAX_PARTICLES]; 1];
    let mut host_drs = [[Axes{x:0., y:0., z:0. }; MAX_PARTICLES]; 1];
    let mut a_const = [Axes{x:0., y:0., z:0. }; MAX_PARTICLES]; // array that stores the value of the constant term
    let mut a_new = [Axes{x:0., y:0., z:0. }; MAX_PARTICLES]; // stores the newly calculated term
    let mut rs = [[0.; MAX_PARTICLES]; MAX_PARTICLES];
    let mut drs = [[Axes{x:0., y:0., z:0. }; MAX_PARTICLES]; MAX_PARTICLES];
    
    let mut host_particle = host_particle;
    let mut particles = particles;
    let mut more_particles = more_particles;
    let (host_newtonian_inertial_accelerations, newtonian_inertial_accelerations) = get_anderson1975_newhall1983_newtonian_inertial_accelerations(&mut host_particle, &mut particles, &mut more_particles, ignored_gravity_terms);

    for (i, ((particle_i, drs_i), rs_i)) in 
                                    iter::once(&*host_particle).chain(particles.iter()).chain(more_particles.iter()) // zip will pick the lowest common number of elements
                                    .zip(host_drs.iter_mut().chain(drs.iter_mut()))
                                    .zip(host_rs.iter_mut().chain(rs.iter_mut()))
                                    .enumerate() {
        // compute distances
        for (j, (particle_j, drs_i_j)) in iter::once(&*host_particle).chain(particles.iter()).chain(more_particles.iter()) // zip will pick the lowest common number of elements
                                        .zip(drs_i.iter_mut())
                                        .enumerate() {
            if j != i{
                drs_i_j.x = particle_i.inertial_position.x - particle_j.inertial_position.x;
                drs_i_j.y = particle_i.inertial_position.y - particle_j.inertial_position.y;
                drs_i_j.z = particle_i.inertial_position.z - particle_j.inertial_position.z;
                rs_i[j] = (drs_i_j.x.powi(2) + drs_i_j.y.powi(2) + drs_i_j.z.powi(2)).sqrt();
                //println!("i j: {} {} {:e}", i, j, rs_i[j]);
            }
        }
    }

    for (i, (((particle_i, a_const_i), drs_i), rs_i)) in iter::once(&*host_particle).chain(particles.iter()).chain(more_particles.iter()) // zip will pick the lowest common number of elements
                                            .zip(host_a_const.iter_mut().chain(a_const.iter_mut()))
                                            .zip(host_drs.iter_mut().chain(drs.iter_mut()))
                                            .zip(host_rs.iter().chain(rs.iter()))
                                            .enumerate() {

        // then compute the constant terms:
        let mut a_constx = 0.;
        let mut a_consty = 0.;
        let mut a_constz = 0.;
        // 1st constant part
        for (j, (particle_j, drs_i_j)) in iter::once(&*host_particle).chain(particles.iter()).chain(more_particles.iter()) // zip will pick the lowest common number of elements
                                        .zip(drs_i.iter())
                                        .enumerate() {
            if j != i {
                let dxij = drs_i_j.x;
                let dyij = drs_i_j.y;
                let dzij = drs_i_j.z;
                let rij2 = rs_i[j].powi(2);
                let rij3 = rij2*rs_i[j];

                let mut a1 = 0.;
                for (k, (particle_k, rs_i_k)) in iter::once(&*host_particle).chain(particles.iter()).chain(more_particles.iter()) // zip will pick the lowest common number of elements
                                                .zip(rs_i.iter())
                                                    .enumerate() {
                    if k != i {
                        a1 += (4./(SPEED_OF_LIGHT_2)) * G*particle_k.mass/rs_i_k;
                    }
                }

                let mut a2 = 0.;
                for (l, (particle_l, rs_l)) in iter::once(&*host_particle).chain(particles.iter()).chain(more_particles.iter()) // zip will pick the lowest common number of elements
                                                .zip(host_rs.iter().chain(rs.iter()))
                                                    .enumerate() {
                    if l != j {
                        a2 += (1./(SPEED_OF_LIGHT_2)) * G*particle_l.mass/rs_l[j];
                    }
                }

                let vi2= particle_i.inertial_velocity.x.powi(2) + particle_i.inertial_velocity.y.powi(2) + particle_i.inertial_velocity.z.powi(2);
                let a3 = -vi2/SPEED_OF_LIGHT_2;
                
                let vj2 = particle_j.inertial_velocity.x.powi(2) + particle_j.inertial_velocity.y.powi(2) + particle_j.inertial_velocity.z.powi(2);
                let a4 = -2.*vj2/SPEED_OF_LIGHT_2;

                let a5 = (4./SPEED_OF_LIGHT_2) * (particle_i.inertial_velocity.x*particle_j.inertial_velocity.x + particle_i.inertial_velocity.y*particle_j.inertial_velocity.y + particle_i.inertial_velocity.z*particle_j.inertial_velocity.z);

                let a6_0 = dxij*particle_j.inertial_velocity.x + dyij*particle_j.inertial_velocity.y + dzij*particle_j.inertial_velocity.z;
                let a6 = (3./(2.*SPEED_OF_LIGHT_2)) * a6_0.powi(2)/rij2;

                let factor1 = a1 + a2 + a3 + a4 + a5 + a6;
                //println!("factors {:e} {:e} {:e} {:e} {:e} {:e}", a1, a2, a3, a4, a5, a6);
                a_constx += G*particle_j.mass*dxij*factor1/rij3;
                a_consty += G*particle_j.mass*dyij*factor1/rij3;
                a_constz += G*particle_j.mass*dzij*factor1/rij3;

                // 2nd constant part
                let dvxij = particle_i.inertial_velocity.x - particle_j.inertial_velocity.x; 
                let dvyij = particle_i.inertial_velocity.y - particle_j.inertial_velocity.y; 
                let dvzij = particle_i.inertial_velocity.z - particle_j.inertial_velocity.z; 

                let factor2 = dxij*(4.*particle_i.inertial_velocity.x - 3.*particle_j.inertial_velocity.x) + dyij*(4.*particle_i.inertial_velocity.y -3.*particle_j.inertial_velocity.y) + dzij*(4.*particle_i.inertial_velocity.z - 3.*particle_j.inertial_velocity.z);

                a_constx += G*particle_j.mass*factor2*dvxij/rij3/SPEED_OF_LIGHT_2;
                a_consty += G*particle_j.mass*factor2*dvyij/rij3/SPEED_OF_LIGHT_2;
                a_constz += G*particle_j.mass*factor2*dvzij/rij3/SPEED_OF_LIGHT_2;
            }
        }
        a_const_i.x = a_constx;
        a_const_i.y = a_consty;
        a_const_i.z = a_constz;
        //println!("a_const_i {:?}", a_const_i);
    }


    let n_particles = particles.len() + more_particles.len();
    let dev_limit = 1.0e-30;
    let max_iterations = 10;
    // Now running the substitution again and again through the loop below
    for k in 0..max_iterations {
        let host_a_old = host_a_new.clone();
        let a_old = a_new.clone();
        // now add on the non-constant term
        for (i, (((a_new_i, drs_i), rs_i), a_const_i)) in host_a_new.iter_mut().chain(a_new[..n_particles].iter_mut()) // zip will pick the lowest common number of elements
                                        .zip(host_drs.iter_mut().chain(drs.iter_mut()))
                                        .zip(host_rs.iter_mut().chain(rs.iter_mut()))
                                        .zip(host_a_const.iter().chain(a_const.iter()))
                                        .enumerate() {
            let mut non_constx = 0.;
            let mut non_consty = 0.;
            let mut non_constz = 0.;
            for (j, ((((particle_j, a_old_j), a_newton_j), drs_i_j), rs_i_j)) in iter::once(&*host_particle).chain(particles.iter()).chain(more_particles.iter()) // zip will pick the lowest common number of elements
                                                                                        .zip(host_a_old.iter().chain(a_old.iter()))
                                                                                        .zip(iter::once(&host_newtonian_inertial_accelerations).chain(newtonian_inertial_accelerations.iter()))
                                                                                        .zip(drs_i.iter())
                                                                                        .zip(rs_i.iter())
                                                                                        .enumerate() {
                if j != i {
                    let dxij = drs_i_j.x;
                    let dyij = drs_i_j.y;
                    let dzij = drs_i_j.z;
                    let rij = rs_i_j;
                    let rij2 = rij.powi(2);
                    let rij3 = rij2*rij;
                    non_constx += (G*particle_j.mass*dxij/rij3)*(dxij*(a_newton_j.x+a_old_j.x)+dyij*(a_newton_j.y+a_old_j.y)+
                                dzij*(a_newton_j.z+a_old_j.z))/(2.*SPEED_OF_LIGHT_2) + (7./(2.*SPEED_OF_LIGHT_2))*G*particle_j.mass*(a_newton_j.x+a_old_j.x)/rij;
                    non_consty += (G*particle_j.mass*dyij/rij3)*(dxij*(a_newton_j.x+a_old_j.x)+dyij*(a_newton_j.y+a_old_j.y)+
                                dzij*(a_newton_j.z+a_old_j.z))/(2.*SPEED_OF_LIGHT_2) + (7./(2.*SPEED_OF_LIGHT_2))*G*particle_j.mass*(a_newton_j.y+a_old_j.y)/rij;
                    non_constz += (G*particle_j.mass*dzij/rij3)*(dxij*(a_newton_j.x+a_old_j.x)+dyij*(a_newton_j.y+a_old_j.y)+
                                dzij*(a_newton_j.z+a_old_j.z))/(2.*SPEED_OF_LIGHT_2) + (7./(2.*SPEED_OF_LIGHT_2))*G*particle_j.mass*(a_newton_j.z+a_old_j.z)/rij;
                }
            }
            a_new_i.x = a_const_i.x + non_constx;
            a_new_i.y = a_const_i.y + non_consty;
            a_new_i.z = a_const_i.z + non_constz;
            //println!("non_constx {:?}", non_constx);
            //println!("non_consty {:?}", non_consty);
            //println!("non_constz {:?}", non_constz);
        }
        
        // break out loop if a_new is converging
        let mut maxdev = 0.;
        let mut dx = 0.;
        let mut dy = 0.;
        let mut dz = 0.;
        for (a_new_i, a_old_i) in host_a_new.iter_mut().chain(a_new[..n_particles].iter_mut()) // zip will pick the lowest common number of elements
                                .zip(host_a_old.iter().chain(a_old.iter())) {
            if a_new_i.x.abs() < dev_limit {
                dx = (a_new_i.x - a_old_i.x).abs() / a_new_i.x;
            }
            if a_new_i.y.abs() < dev_limit {
                dy = (a_new_i.y - a_old_i.y).abs() / a_new_i.y;
            }
            if a_new_i.z.abs() < dev_limit {
                dz = (a_new_i.z - a_old_i.z).abs() / a_new_i.z;
            }
            if dx > maxdev { maxdev = dx; }
            if dy > maxdev { maxdev = dy; }
            if dz > maxdev { maxdev = dz; }

        }

        if maxdev < dev_limit {
            break;
        } else if k == max_iterations {
            println!("[WARNING {} UTC] {} iterations in general relativity failed to converge.", time::now_utc().strftime("%Y.%m.%d %H:%M:%S").unwrap(), max_iterations);
        }

    }
    
    //// update acceleration in particles
    // This algorithm computes general_relativity_acceleration in the inertial frame,
    // which is the same coordinate system that is expressed all the rest of additional
    // effects
    for (particle, a_new_particle) in particles.iter_mut().chain(more_particles.iter_mut())
                        .zip(a_new.iter()){
        particle.general_relativity_acceleration.x = a_new_particle.x;
        particle.general_relativity_acceleration.y = a_new_particle.y;
        particle.general_relativity_acceleration.z = a_new_particle.z;
    }
    host_particle.general_relativity_acceleration.x = host_a_new[0].x;
    host_particle.general_relativity_acceleration.y = host_a_new[0].y;
    host_particle.general_relativity_acceleration.z = host_a_new[0].z;

}
// [end] General Relativity FULL based on REBOUNDx gr.c
//--------------------------------------------------------------------------
////////////////////////////////////////////////////////////////////////////
