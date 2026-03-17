use std::sync::Arc;

use image::{Rgb, RgbImage};
use rust3d::engine::camera::RayCamera;
use rust3d::engine::lighting::Light;
use rust3d::engine::pathtracing::RayTracingScene;
use rust3d::engine::simplex3d_sphere_tree::poly_tree::PolyTree;
use rust3d::engine::utils::transformation::{PI, Transformable};
use rust3d::geometry::quad::Quad;
use rust3d::geometry::simplex3d::Simplex3D;
use rust3d::geometry::sphere::Sphere;
use rust3d::geometry::vector3::Vector3 as V;
use sdl2::pixels::Color;

use rust3d::engine::lighting::Material;

fn main() {
    let width: usize = 1280;
    let height: usize = 720;

    let mut mesh = Simplex3D::parse_wavefront(
        &String::from("demo_assets/models/horse.obj"),
        &String::from("demo_assets/models/horse_tex.png"),
    );

    mesh.scale(V { x: 0.65, y: 0.65, z: 0.65 });
    let mut horse = *PolyTree::new(mesh);
    horse.goto(V { x: 0.8, y: 0.35, z: 0.0 });
    horse.rot(V { x: 0.0, y: 0.0, z: PI });

    let mut scene = RayTracingScene::new();
    scene.set_shading_samples(25);

    // Room as 6 separate thin quads -> corners/angles become clearly visible.
    let room_center = V { x: 0.6, y: 0.9, z: 0.0 };
    let room_size = V { x: 12.0, y: 7.0, z: 8.0 };
    let t = 0.08; // wall thickness

    let hx = room_size.x * 0.5;
    let hy = room_size.y * 0.5;
    let hz = room_size.z * 0.5;

    // Floor / ceiling
    scene.add(Quad::new(
        V { x: room_center.x, y: room_center.y - hy, z: room_center.z },
        V { x: room_size.x, y: t, z: room_size.z },
        Color::RGB(95, 90, 110),
    ));
    scene.add(Quad::new(
        V { x: room_center.x, y: room_center.y + 2.1, z: room_center.z },
        V { x: room_size.x, y: t, z: room_size.z },
        Color::WHITE,
    ));

    // Left / right walls
    scene.add(Quad::new(
        V { x: room_center.x - hx, y: room_center.y, z: room_center.z },
        V { x: t, y: room_size.y, z: room_size.z },
        Color::RGB(65, 78, 115),
    ));
    scene.add(Quad::new(
        V { x: room_center.x + hx, y: room_center.y, z: room_center.z },
        V { x: t, y: room_size.y, z: room_size.z },
        Color::RGB(110, 70, 112),
    ));

    // Back / front walls
    scene.add(Quad::new(
        V { x: room_center.x, y: room_center.y, z: room_center.z - hz },
        V { x: room_size.x, y: room_size.y, z: t },
        Color::RGB(76, 86, 120),
    ));
    scene.add(Quad::new(
        V { x: room_center.x, y: room_center.y, z: room_center.z + hz },
        V { x: room_size.x, y: room_size.y, z: t },
        Color::RGB(120, 78, 98),
    ));

    scene.set_ambient_light(Color::RGB(20, 20, 20));
    
    // Floating spheres
    scene.add(Sphere::new(
        V::new(-0.6, -0.2, -1.5),
        0.45,
        Material::new(Color::RGB(80, 220, 255), 1.0),
    ));
    scene.add(Sphere::new(
        V::new(1.4, 0.4, 1.35),
        0.35,
        Material::new(Color::RGB(255, 120, 230), 1.0),
    ));
    scene.add(Sphere::new(
        V::new(2.3, -0.1, -0.2),
        0.28,
        Material::new(Color::RGB(255, 220, 90), 1.0),
    ));

    scene.add(Sphere::mirror(
        V { x: 1.9, y: -0.15, z: 0.8 },
        0.55,
    ));

    scene.add(horse);

    scene.add_light(Light {
        position: V { x: -1.1, y: 2.8, z: -2.1 },
        color: Color::RGB(80, 130, 255),
        intensity: 1.25,
    });
    scene.add_light(Light {
        position: V { x: -0.4, y: 2.6, z: 2.2 },
        color: Color::RGB(185, 90, 255),
        intensity: 1.15,
    });
    scene.add_light(Light {
        position: V { x: 1.0, y: 2.2, z: 0.2 },
        color: Color::RGB(255, 228, 95),
        intensity: 1.35,
    });

    let camera = RayCamera::new(V { x: -5.2, y: -0.1, z: 0.0 }, -0.22, -0.40, 0.0);
    let pixels = camera.render_frame_multi(Arc::new(scene), width, height, 8);

    let mut image = RgbImage::new(width as u32, height as u32);
    let mut pos = 0usize;
    for x in 0..width {
        for y in 0..height {
            let c = pixels[pos];
            image.put_pixel(x as u32, y as u32, Rgb([c.r, c.g, c.b]));
            pos += 1;
        }
    }

    std::fs::create_dir_all("target/renders").expect("could not create output directory");
    let out = "target/renders/raytraced_scene.png";
    image.save(out).expect("failed to save raytraced image");
    println!("Saved raytraced image to {out}");
}
