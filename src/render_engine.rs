// inspiration taken from: https://github.com/OneLoneCoder/Javidx9/tree/master/ConsoleGameEngine/BiggerProjects/Engine3D

mod model;
use model::{CUBE};

use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyle, Triangle},
};
use std::fmt::Debug;
use nalgebra::{
    Perspective3,
    Vector3,
    Rotation3,
    Point3,
    Isometry3,
    Matrix4,
    Translation3,
    UnitQuaternion
};

pub fn draw<D>(display: &mut D, time_sec: f32)
where
    D: DrawTarget<Color = Rgb888>,
    D::Error: Debug,
{
    let screen_width = display.bounding_box().size.width as f32;
    let screen_height = display.bounding_box().size.height as f32;
    let f_theta = time_sec * 2.0;

    let rot_z = Rotation3::from_axis_angle(
        &-Vector3::z_axis(),
        f_theta * 0.4
    );
    let rot_x = Rotation3::from_axis_angle(
        &Vector3::x_axis(),
        f_theta
    );
    // Model space to world space
    let model = Isometry3::from_parts(
        Translation3::new(0.0, 0.0, 2.0),
        UnitQuaternion::from(rot_x * rot_z)
    );

    // World space to camera space
    let view= Isometry3::look_at_rh(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(0.0, 0.0, 1.0),
        &Vector3::y()
    );

    // Camera space to normalized device coordinates (ndc)
    let projection = Perspective3::new(
        screen_width / screen_height,
        std::f32::consts::PI / 2.0, // 90°
        0.1,
        1000.0
    );

    // Model space to ndc
    let model_view_projection = projection.into_inner() * (view * model).to_homogeneous();

    // ndc to screen space
    let screen = Matrix4::new(
        0.5 * screen_width, 0.0, 0.0, 0.5 * screen_width,
        0.0, 0.5 * screen_height, 0.0, 0.5 * screen_height,
        0.0, 0.0, 0.5, 0.5,
        0.0, 0.0, 0.0, 1.0);

    // Create styles used by the drawing operations.
    let stroke = PrimitiveStyle::with_stroke(
        Rgb888::new(0xFF, 100, 0x0),
        1
    );

    // Draw triangles
    for tri in CUBE.tris {
        let mut vec = Vec::new();
        for mut p in tri.p {
            p = (screen * model_view_projection).transform_point(&p);
            vec.push(Point::new(p.x as i32, p.y as i32));
        }
        Triangle::from_slice(vec.as_slice())
            .into_styled(stroke)
            .draw(display)
            .unwrap();
    }
}
