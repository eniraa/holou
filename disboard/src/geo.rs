use coordinates::three_dimensional::Spherical;
use num_traits::Float;
use rand::{
    distributions::{uniform::SampleUniform, Uniform},
    Rng,
};

pub fn random_spherical<T: Float + SampleUniform>(r: T, rng: &mut impl Rng) -> Spherical<T> {
    let theta_range =
        Uniform::from(T::zero()..T::from(std::f64::consts::PI * 2.0).unwrap_or_else(T::zero));
    let theta = rng.sample(theta_range);

    let cos_phi_range = Uniform::from(-T::one()..T::one());
    let cos_phi = rng.sample(cos_phi_range);
    let phi = cos_phi.acos();

    Spherical {
        radius: r,
        polar_angle: theta,
        azimuthal_angle: phi,
    }
}
