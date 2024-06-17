use nalgebra::{RealField, Unit, Vector3};
use num_traits::Float;
use qhull::Qh;
use rand::{
    distributions::{uniform::SampleUniform, Uniform},
    Rng,
};

/// A Voronoi cell.
#[derive(Debug, Clone)]
pub struct VoronoiCell<T> {
    /// The center of the cell. On a unit sphere, this is the normal.
    pub center: Vector3<T>,
    /// The vertices of the cell, sorted counterclockwise.
    pub vertices: Vec<Vector3<T>>,
}

/// Samples `n` points uniformly on a sphere of radius `r`.
pub fn sample_sphere<T: Float + SampleUniform>(n: usize, rng: &mut impl Rng) -> Vec<Vector3<T>> {
    let mut points = Vec::with_capacity(n);

    for _ in 0..n {
        let u = rng.sample(Uniform::from(T::zero()..T::one()));
        let v = rng.sample(Uniform::from(T::zero()..T::one()));

        let theta = T::from(2.0 * std::f64::consts::PI).unwrap() * u;
        let phi = (T::from(2.0).unwrap() * v - T::from(1.0).unwrap()).acos();

        let x = phi.sin() * theta.cos();
        let y = phi.sin() * theta.sin();
        let z = phi.cos();

        points.push(Vector3::new(x, y, z));
    }

    points
}

/// Sorts vertices counterclockwise around the given axis.
fn sort_vertices_ccw<T: Float + RealField + From<f64>>(
    vertices: Vec<Vector3<T>>,
    axis: Vector3<T>,
) -> Vec<Vector3<T>> {
    let e_z = Unit::new_normalize(axis);
    let e_x = if e_z.dot(&Vector3::z()) > 0.99.into() {
        Unit::new_normalize(e_z.cross(&Vector3::x()))
    } else {
        Unit::new_normalize(e_z.cross(&Vector3::z()))
    };
    let e_y = e_z.cross(&e_x);

    let mut points = vertices
        .iter()
        .map(|&point| {
            let p_perp = (point - e_z.into_inner() * point.dot(&e_z)).normalize();
            let angle = RealField::atan2(p_perp.dot(&e_y), p_perp.dot(&e_x));
            (angle, point)
        })
        .collect::<Vec<_>>();

    points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    points
        .into_iter()
        .map(|(_, point)| point)
        .collect::<Vec<_>>()
}

/// Computes the Voronoi diagram of the given points.
pub fn voronoi<T: Float + RealField + From<f64> + Into<f64>>(
    points: Vec<Vector3<T>>,
) -> Vec<VoronoiCell<T>> {
    let qh = Qh::builder()
        .compute(true)
        .build_from_iter(points.iter().map(|v| [v.x.into(), v.y.into(), v.z.into()]))
        .unwrap();
    let mut cells = Vec::new();

    for point in points.iter() {
        let simplices = qh
            .faces()
            .filter(|f| {
                f.vertices()
                    .unwrap()
                    .iter()
                    .any(|v| v.point() == [point.x.into(), point.y.into(), point.z.into()])
            })
            .collect::<Vec<_>>();
        let vertices = sort_vertices_ccw(
            simplices
                .iter()
                .map(|s| {
                    let normal = s.normal();
                    Vector3::new(normal[0].into(), normal[1].into(), normal[2].into())
                })
                .collect::<Vec<_>>(),
            *point,
        );

        cells.push(VoronoiCell {
            center: *point,
            vertices,
        });
    }

    cells
}

/// Computes the centroid of a Voronoi cell.
pub fn centroid<T: Float + RealField>(cell: VoronoiCell<T>) -> Vector3<T> {
    let mut centroid = Vector3::zeros();
    for i in 0..cell.vertices.len() {
        let v1 = cell.vertices[i];
        let v2 = cell.vertices[(i + 1) % cell.vertices.len()];
        let d: T = (v1 - v2).norm();
        centroid += (v1 + v2) * d;
    }

    centroid.normalize()
}

/// Relaxes the given Voronoi cells via LLoyd's algorithm.
pub fn relax<T: Float + RealField>(cells: Vec<VoronoiCell<T>>, weight: T) -> Vec<Vector3<T>> {
    let mut points = Vec::new();
    for cell in cells.iter() {
        points.push((centroid(cell.clone()) * weight + cell.center).normalize());
    }

    points
}

#[cfg(test)]
mod tests {
    use approx::{assert_relative_eq, relative_eq};

    use super::*;

    #[test]
    fn test_sample_sphere() {
        let n = 1024;
        let mut rng = rand::thread_rng();

        let points = sample_sphere::<f64>(n, &mut rng);

        assert_eq!(points.len(), n);
        for point in points.iter() {
            let dist = point.norm();
            assert_relative_eq!(dist, 1.0, epsilon = f32::EPSILON as f64);
        }
    }

    #[test]
    fn test_voronoi() {
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        let icosahedron = vec![
            Vector3::new(phi, 1.0, 0.0),
            Vector3::new(-phi, 1.0, 0.0),
            Vector3::new(phi, -1.0, 0.0),
            Vector3::new(-phi, -1.0, 0.0),
            Vector3::new(1.0, 0.0, phi),
            Vector3::new(1.0, 0.0, -phi),
            Vector3::new(-1.0, 0.0, phi),
            Vector3::new(-1.0, 0.0, -phi),
            Vector3::new(0.0, phi, 1.0),
            Vector3::new(0.0, -phi, 1.0),
            Vector3::new(0.0, phi, -1.0),
            Vector3::new(0.0, -phi, -1.0),
        ]
        .iter()
        .map(Vector3::normalize)
        .collect::<Vec<_>>();
        let cells = voronoi(icosahedron.clone());

        // check that they match
        for point in icosahedron.iter() {
            assert!(cells.iter().any(|cell| relative_eq!(cell.center, *point)));
        }

        for cell in cells.iter() {
            assert!(icosahedron
                .iter()
                .any(|point| relative_eq!(*point, cell.center)));
        }

        let dodecahedron = vec![
            Vector3::new(1.0, 1.0, 1.0),
            Vector3::new(-1.0, 1.0, 1.0),
            Vector3::new(1.0, -1.0, 1.0),
            Vector3::new(-1.0, -1.0, 1.0),
            Vector3::new(1.0, 1.0, -1.0),
            Vector3::new(-1.0, 1.0, -1.0),
            Vector3::new(1.0, -1.0, -1.0),
            Vector3::new(-1.0, -1.0, -1.0),
            Vector3::new(phi.recip(), phi, 0.0),
            Vector3::new(-phi.recip(), phi, 0.0),
            Vector3::new(phi.recip(), -phi, 0.0),
            Vector3::new(-phi.recip(), -phi, 0.0),
            Vector3::new(0.0, phi.recip(), phi),
            Vector3::new(0.0, -phi.recip(), phi),
            Vector3::new(0.0, phi.recip(), -phi),
            Vector3::new(0.0, -phi.recip(), -phi),
            Vector3::new(phi, 0.0, phi.recip()),
            Vector3::new(-phi, 0.0, phi.recip()),
            Vector3::new(phi, 0.0, -phi.recip()),
            Vector3::new(-phi, 0.0, -phi.recip()),
        ]
        .iter()
        .map(Vector3::normalize)
        .collect::<Vec<_>>();

        // check that they match
        for vertex in dodecahedron.iter() {
            assert!(cells
                .iter()
                .any(|cell| cell.vertices.iter().any(|v| relative_eq!(v, vertex))));
        }

        for cell in cells.iter() {
            assert!(dodecahedron
                .iter()
                .any(|vertex| { cell.vertices.iter().any(|v| relative_eq!(*v, *vertex)) }));
        }

        // check that each face has correct vertices
        for cell in cells.iter() {
            assert_eq!(cell.vertices.len(), 5);
            for vertex in cell.vertices.iter() {
                assert_relative_eq!(
                    (cell.center - vertex).norm(),
                    (2.0 - 2.0 * (1.0 / 15.0 * (5.0 + 2.0 * f64::sqrt(5.0))).sqrt()).sqrt() // trust
                );
            }
        }

        // check that vertices are counterclockwise
        for cell in cells.iter() {
            for i in 0..cell.vertices.len() {
                let v1 = cell.vertices[i];
                let v2 = cell.vertices[(i + 1) % cell.vertices.len()];
                let v3 = cell.vertices[(i + 2) % cell.vertices.len()];

                assert_relative_eq!((v2 - v1).cross(&(v3 - v2)).normalize(), cell.center);
            }
        }
    }

    #[test]
    fn test_centroid() {
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        let icosahedron = vec![
            Vector3::new(phi, 1.0, 0.0),
            Vector3::new(-phi, 1.0, 0.0),
            Vector3::new(phi, -1.0, 0.0),
            Vector3::new(-phi, -1.0, 0.0),
            Vector3::new(1.0, 0.0, phi),
            Vector3::new(1.0, 0.0, -phi),
            Vector3::new(-1.0, 0.0, phi),
            Vector3::new(-1.0, 0.0, -phi),
            Vector3::new(0.0, phi, 1.0),
            Vector3::new(0.0, -phi, 1.0),
            Vector3::new(0.0, phi, -1.0),
            Vector3::new(0.0, -phi, -1.0),
        ]
        .iter()
        .map(Vector3::normalize)
        .collect::<Vec<_>>();
        let cells = voronoi(icosahedron.clone());

        for cell in cells.iter() {
            let c = centroid(cell.clone());
            assert_relative_eq!(c, cell.center);
        }
    }
}
