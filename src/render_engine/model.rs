use nalgebra::{point, Point3};

pub struct Triangle3 {
    pub(crate) p: [Point3<f32>; 3]
}

pub struct Mesh<const S: usize> {
    pub(crate) tris: [Triangle3; S]
}

pub const CUBE: Mesh<12> = Mesh { tris: [
    // South
    Triangle3{ p:[point![0.0, 0.0, 0.0], point![0.0, 1.0, 0.0], point![1.0, 1.0, 0.0]] },
    Triangle3{ p:[point![0.0, 0.0, 0.0], point![1.0, 1.0, 0.0], point![1.0, 0.0, 0.0]] },
    // East
    Triangle3{ p:[point![1.0, 0.0, 0.0], point![1.0, 1.0, 0.0], point![1.0, 1.0, 1.0]] },
    Triangle3{ p:[point![1.0, 0.0, 0.0], point![1.0, 1.0, 1.0], point![1.0, 0.0, 1.0]] },
    // North
    Triangle3{ p:[point![1.0, 0.0, 1.0], point![1.0, 1.0, 1.0], point![0.0, 1.0, 1.0]] },
    Triangle3{ p:[point![1.0, 0.0, 1.0], point![0.0, 1.0, 1.0], point![0.0, 0.0, 1.0]] },
    // West
    Triangle3{ p:[point![0.0, 0.0, 1.0], point![0.0, 1.0, 1.0], point![0.0, 1.0, 0.0]] },
    Triangle3{ p:[point![0.0, 0.0, 1.0], point![0.0, 1.0, 0.0], point![0.0, 0.0, 0.0]] },
    // Top
    Triangle3{ p:[point![0.0, 1.0, 0.0], point![0.0, 1.0, 1.0], point![1.0, 1.0, 1.0]] },
    Triangle3{ p:[point![0.0, 1.0, 0.0], point![1.0, 1.0, 1.0], point![1.0, 1.0, 0.0]] },
    // Bottom
    Triangle3{ p:[point![1.0, 0.0, 1.0], point![0.0, 0.0, 1.0], point![0.0, 0.0, 0.0]] },
    Triangle3{ p:[point![1.0, 0.0, 1.0], point![0.0, 0.0, 0.0], point![1.0, 0.0, 0.0]] },
]};
