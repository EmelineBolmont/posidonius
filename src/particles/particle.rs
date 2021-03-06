use super::{EvolutionType};
use super::{Axes};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct Particle {
    pub id: usize, // Unique internal identifier
    pub mass: f64,
    pub mass_g: f64,
    pub radius: f64,
    pub scaled_dissipation_factor: f64, // sigma
    pub dissipation_factor_scale: f64, // to scale the dissipation factor (multiply)
    pub radius_of_gyration_2: f64,  // radius of gyration square can be computed in terms of the mass moment of inertia, which 
                                    // depends on the shape of the body and determines the torque needed for a desired angular acceleration
    pub love_number: f64,   // Love number of degree 2 (i.e., k2). Dimensionless parameters that measure the rigidity of a planetary body and the 
                            // susceptibility of its shape to change in response to a tidal potential.
    pub fluid_love_number: f64,   // love number for a completely fluid planet (used for rotational flattening effects)
    //
    // In the heliocentric frame the star is at rest with respect to the origin of the coordinate system
    pub position: Axes,
    pub velocity: Axes,
    pub acceleration: Axes,
    // In the inertial frame the center of mass of a system is at rest with respect to the origin of the coordinate system
    // (i.e., barycentric frame)
    pub inertial_position: Axes,
    pub inertial_velocity: Axes,
    pub inertial_acceleration: Axes,
    //
    pub radial_velocity: f64,
    pub norm_velocity_vector: f64,
    pub norm_velocity_vector_2: f64,
    pub norm_spin_vector_2: f64,
    pub distance: f64,
    // Tides
    pub scalar_product_of_vector_position_with_stellar_spin: f64,
    pub scalar_product_of_vector_position_with_planetary_spin: f64,
    pub radial_component_of_the_force_induced_by_rotation: f64,
    pub orthogonal_component_of_the_tidal_force_due_to_stellar_tide: f64,
    pub orthogonal_component_of_the_tidal_force_due_to_planetary_tide: f64,
    pub factor_for_the_force_induced_by_star_rotation: f64,
    pub factor_for_the_force_induced_by_planet_rotation: f64,
    pub radial_component_of_the_tidal_force: f64,
    pub orthogonal_component_of_the_force_induced_by_star_rotation: f64,
    pub orthogonal_component_of_the_force_induced_by_planet_rotation: f64,
    pub denergy_dt: f64,
    pub radial_component_of_the_tidal_force_dissipative_part_when_star_as_point_mass: f64, // needed to compute denergy_dt
    pub dangular_momentum_dt_due_to_tides: Axes, // Force
    pub dangular_momentum_dt_induced_by_rotational_flattening: Axes, // Force
    pub dangular_momentum_dt: Axes, // Force
    pub spin: Axes,
    pub dangular_momentum_dt_per_moment_of_inertia: Axes,
    pub moment_of_inertia_ratio: f64, // Spin related
    pub moment_of_inertia: f64, // Spin related
    //
    pub wind_k_factor: f64,
    pub wind_rotation_saturation: f64,
    pub wind_rotation_saturation_2: f64,
    pub wind_factor: f64, // Spin related
    //
    pub tidal_acceleration: Axes,
    // Rotational flattening
    pub acceleration_induced_by_rotational_flattering: Axes,
    // General Relativity
    pub general_relativity_factor: f64,
    pub general_relativity_acceleration: Axes,
    // Evolution
    pub evolution_type: EvolutionType,
    pub lag_angle: f64, // MathisSolarLike
}

