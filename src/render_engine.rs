// inspired by: https://github.com/OneLoneCoder/Javidx9/tree/master/ConsoleGameEngine/BiggerProjects/Engine3D

use embedded_graphics::primitives::Primitive;
use embedded_graphics::{
    pixelcolor::Rgb888, prelude::*, primitives::{PrimitiveStyle, Triangle},
};
use nalgebra::{
    ArrayStorage, Const, Isometry3, Matrix, Matrix4, Perspective3, Point3, Rotation3, Translation3,
    Unit, UnitQuaternion, Vector3,
};
use std::fmt::Debug;
use std::{fs, process};
use std::io::{Error, ErrorKind};
use wavefront_obj::obj::{parse, Primitive::Triangle as ObjTriangle};

struct Triangle3 {
    pub(crate) triangle: [Point3<f32>; 3],
    pub(crate) lum: Option<f32>
}

struct Mesh {
    pub(crate) triangles: Vec<Triangle3>
}


pub struct Engine {
    mesh: Mesh,
    _projection: Perspective3<f32>,
    _screen: Matrix<f32, Const<4>, Const<4>, ArrayStorage<f32, 4, 4>>,
    _light_direction: Matrix<f32, Const<3>, Const<1>, ArrayStorage<f32, 3, 1>>,
}

impl Engine {
    pub fn new<D>(path: &str, display: &mut D) -> Engine
    where
        D: DrawTarget<Color = Rgb888>,
        D::Error: Debug,
    {
        let model = parse(
            fs::read_to_string(&path).unwrap()
        ).unwrap();

        let obj = &model.objects[0];
        let mut triangles: Vec<Triangle3> = Vec::new();
        let shapes = obj.geometry
            .iter()
            .map(|x| &x.shapes)
            .flatten();
        for shape in shapes {
            if let ObjTriangle(tri0, tri1, tri2) = shape.primitive {
                let triangle = [tri0, tri1, tri2]
                    .map(|idx| obj.vertices[idx.0])
                    .map(|v| Point3::from_slice(
                        &[v.x, v.y, v.z].map(|i| i as f32)
                    ));
                triangles.push(Triangle3 {
                    triangle,
                    lum: None
                })
            } else {
                eprintln!("{}", Error::new(
                    ErrorKind::Other, "Loaded object files must triangulate all faces.")
                );
                process::exit(1);
            }
        }
        let screen_width = display.bounding_box().size.width as f32;
        let screen_height = display.bounding_box().size.height as f32;

        Engine {
            mesh: Mesh { triangles },
            // Camera space to normalized device coordinates (ndc)
            _projection: Perspective3::new(
                screen_width / screen_height,
                std::f32::consts::PI / 2.0, // 90°
                0.1,
                1000.0
            ),
            // Ndc to screen space
            _screen: Matrix4::new(
                0.5 * screen_width, 0.0, 0.0, 0.5 * screen_width,
                0.0, 0.5 * screen_height, 0.0, 0.5 * screen_height,
                0.0, 0.0, 0.5, 0.5,
                0.0, 0.0, 0.0, 1.0
            ),
            _light_direction: Vector3::new(0.0, 0.0, -1.0).normalize(),
        }
    }

    pub fn draw<D>(&self, display: &mut D, f_theta: f32)
    where
        D: DrawTarget<Color = Rgb888>,
        D::Error: Debug,
    {
        let f_theta = f_theta * 1.8;
        let rot_z = Rotation3::from_axis_angle(
            &-Vector3::y_axis(),
            f_theta
        );
        let rot_x = Rotation3::from_axis_angle(
            &Vector3::x_axis(),
            (f_theta * 0.4).sin() * 0.3
        );
        // Model space to world space
        let model = Isometry3::from_parts(
            Translation3::new(0.0,  (f_theta*0.4).sin()*1.5, 8.0),
            UnitQuaternion::from(rot_x * rot_z)
        );

        // World space to camera space
        let eye: Point3<f32> = Point3::new(0.0, 0.0, 0.0);
        let view = Isometry3::look_at_lh(
            &eye,
            &Point3::new(0.0, 0.0, 1.0),
            &Vector3::y()
        );

        let mut draw_order: Vec<Triangle3> = Vec::new();

        // Project triangles
        'project: for tri in &self.mesh.triangles {
            let tri_view = tri.triangle.map(
                |p| (view * model).transform_point(&p)
            );
            let line1 = tri_view[1] - tri_view[0];
            let line2 = tri_view[2] - tri_view[0];
            let norm = Unit::new_normalize(line1.cross(&line2));

            if !(norm.dot(&(tri_view[0] - eye)) < 0.0) {
                continue 'project;
            }

            let tri_projected = tri_view.map(
                |p| self._projection.project_point(&p)
            );
            draw_order.push(Triangle3 {
                triangle: tri_projected,
                lum: Option::from(norm.dot(&self._light_direction))
            });
        }

        draw_order.sort_by(|tri_0, tri_1| {
            let tri_0_z = tri_0.triangle.iter().map(|i| i.z).sum::<f32>();
            let tri_1_z = tri_1.triangle.iter().map(|i| i.z).sum();
            tri_0_z.total_cmp(&tri_1_z)
        });

        // Draw triangles
        for tri in draw_order {
            let tri_screen = tri.triangle.map(|p| {
                let p = self._screen.transform_point(&p);
                Point::new(p.x as i32, p.y as i32)
            });

            // Create styles used by the drawing operations.
            let rgb = [1.0, 0.4, 0.1]
                .map(|elem| (elem * tri.lum.unwrap() * 0xFF as f32) as u8);
            let fill = PrimitiveStyle::with_fill(
                Rgb888::new(rgb[0], rgb[1], rgb[2])
            );
            Triangle::from_slice(&tri_screen)
                .into_styled(fill)
                .draw(display)
                .unwrap();
        }
    }
}
