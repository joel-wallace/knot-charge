use nalgebra::{Point3, Vector3, Scalar};
use std::ops::{Sub, Mul, Add};

// Epsilon for floating point comparisons
const EPSILON: f64 = 1e-7; // Adjusted for typical f64 precision

// (Plane struct from previous example can be kept if needed for other things,
// but Möller-Trumbore doesn't explicitly require its D constant)
#[derive(Debug)]
pub struct Plane {
    pub normal: Vector3<f64>,
    pub d: f64,
}

impl Plane {
    pub fn new(normal: Vector3<f64>, point_on_plane: Point3<f64>) -> Self {
        let d = normal.dot(&point_on_plane.coords);
        Plane { normal: normal.normalize(), d } // Store normalized normal
    }

    pub fn from_three_points(p0: &Point3<f64>, p1: &Point3<f64>, p2: &Point3<f64>) -> Option<Self> {
        let v1 = p1 - p0;
        let v2 = p2 - p0;
        let normal = v1.cross(&v2);
        if normal.norm_squared() < EPSILON * EPSILON {
            return None; // Collinear points
        }
        Some(Plane::new(normal, *p0))
    }

    pub fn is_on_plane(&self, point: &Point3<f64>) -> bool {
        (self.normal.dot(&point.coords) - self.d).abs() < EPSILON
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Self {
        Ray { origin, direction: direction.normalize() } // Store normalized direction
    }

    pub fn point_at_parameter(&self, t: f64) -> Point3<f64> {
        self.origin + self.direction * t
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    pub p0: Point3<f64>,
    pub p1: Point3<f64>,
    pub p2: Point3<f64>,
}

impl Triangle {
    pub fn new(p0: Point3<f64>, p1: Point3<f64>, p2: Point3<f64>) -> Self {
        Triangle { p0, p1, p2 }
    }

    /// Calculates the surface normal of the triangle (p1-p0) x (p2-p0).
    /// The length of the normal is twice the area of the triangle.
    /// It's not normalized by default here.
    pub fn normal(&self) -> Vector3<f64> {
        let edge1 = self.p1 - self.p0;
        let edge2 = self.p2 - self.p0;
        edge1.cross(&edge2)
    }

    /// Checks if a ray intersects this triangle.
    /// Uses the Möller-Trumbore algorithm.
    /// Returns Some(intersection_point) if the ray intersects the triangle (front-facing),
    /// None otherwise.
    ///
    /// - `ray`: The ray to test.
    /// - `cull_back_faces`: If true, intersections with back-facing triangles are ignored.
    ///                      A triangle is back-facing if the ray hits it from the side
    ///                      opposite to its normal (determined by p0,p1,p2 winding).
    pub fn intersect_ray(&self, ray: &Ray, cull_back_faces: bool) -> Option<Point3<f64>> {
        let edge1 = self.p1 - self.p0;
        let edge2 = self.p2 - self.p0;

        // P-vector: Perpendicular to ray direction and edge2
        let p_vec = ray.direction.cross(&edge2);

        // Determinant: If close to 0, ray is parallel to triangle plane
        let det = edge1.dot(&p_vec);

        // Back-face culling (optional)
        if cull_back_faces && det < EPSILON { // Ray hits back face or is parallel
            return None;
        }
        // If not culling, ray parallel to plane means no unique intersection point
        // (or ray is in plane which is a more complex case not typically handled by simple hit/no-hit)
        if !cull_back_faces && det.abs() < EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;

        // T-vector: Vector from P0 to ray origin
        let t_vec = ray.origin - self.p0;

        // Calculate u parameter (barycentric coordinate for edge1)
        // u = (T · P) * inv_det
        let u = t_vec.dot(&p_vec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None; // Intersection point is outside triangle along edge1 direction
        }

        // Q-vector
        let q_vec = t_vec.cross(&edge1);

        // Calculate v parameter (barycentric coordinate for edge2)
        // v = (D · Q) * inv_det
        let v = ray.direction.dot(&q_vec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None; // Intersection point is outside triangle along edge2 direction or combination
        }

        // Calculate t parameter (distance along the ray to intersection point)
        // t = (E2 · Q) * inv_det
        let t = edge2.dot(&q_vec) * inv_det;

        if t > EPSILON { // Intersection point is in front of ray origin
            Some(ray.point_at_parameter(t))
        } else {
            None // Intersection is behind ray origin or too close
        }
    }
}


/// Standalone function as per user's request syntax.
/// This function wraps the Triangle struct's method for convenience.
///
/// - `tri_p0`, `tri_p1`, `tri_p2`: Vertices of the triangle.
/// - `ray_origin`: The starting point of the vector/ray.
/// - `ray_direction`: The direction vector. The length of this vector does not matter
///   if interpreted as a ray, as it will be normalized. If interpreted as a segment,
///   the 't' value from Möller-Trumbore would need to be <= 1.0.
///   This implementation assumes an infinitely long ray.
/// - `cull_back_faces`: If true, only intersections with the "front" of the triangle
///   (defined by vertex winding order and the right-hand rule for the normal) are considered.
///
/// Returns `Some(intersection_point)` if the ray passes through the triangle, `None` otherwise.
pub fn does_vector_pass_through_triangle_section(
    tri_p0: &Point3<f64>,
    tri_p1: &Point3<f64>,
    tri_p2: &Point3<f64>,
    ray_origin: &Point3<f64>,
    ray_direction: &Vector3<f64>,
    cull_back_faces: bool,
) -> Option<Point3<f64>> {
    let triangle = Triangle::new(*tri_p0, *tri_p1, *tri_p2);
    // Ensure ray_direction is normalized if Ray::new expects it, or handle internally.
    // Ray::new already normalizes its direction.
    let ray = Ray::new(*ray_origin, *ray_direction);
    triangle.intersect_ray(&ray, cull_back_faces)
}
