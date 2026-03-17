use std::sync::{mpsc, Arc, RwLock};
use std::thread;
use std::time::Instant;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use rust3d::geometry::vector3::Vector3 as V;
use rust3d::engine::camera::{RayCamera};
use rust3d::engine::gameplay::movement::{MovementInputMap, PlayerMovementController};
use rust3d::engine::lighting::{Light, Material};
use rust3d::engine::raymarching::{RayMarchingObject, RayMarchingScene};
use rust3d::engine::utils::rendering::{RayRenderScene};
use rust3d::engine::utils::transformation::Transformable;
use rust3d::geometry::sphere::Sphere;

const W : usize = 600;
const H : usize = 600;

#[derive(Clone, Copy)]
struct MorphSphereCube {
    center: V,
    sphere_radius: f64,
    cube_half_extent: V,
    blend: f64,
    material: Material,
}

impl MorphSphereCube {
    fn new(center: V, sphere_radius: f64, cube_half_extent: V, blend: f64, material: Material) -> Self {
        MorphSphereCube {
            center,
            sphere_radius,
            cube_half_extent,
            blend,
            material,
        }
    }

    fn sphere_sdf(&self, p: V) -> f64 {
        p.d(self.center) - self.sphere_radius
    }

    fn box_sdf(&self, p: V) -> f64 {
        let qx = (p.x - self.center.x).abs() - self.cube_half_extent.x;
        let qy = (p.y - self.center.y).abs() - self.cube_half_extent.y;
        let qz = (p.z - self.center.z).abs() - self.cube_half_extent.z;

        let ox = qx.max(0.0);
        let oy = qy.max(0.0);
        let oz = qz.max(0.0);
        let outside = (ox * ox + oy * oy + oz * oz).sqrt();
        let inside = qx.max(qy.max(qz)).min(0.0);
        outside + inside
    }
}

impl Transformable for MorphSphereCube {
    fn rot(&mut self, r: V) {
        self.center.rotate(r);
    }

    fn rot_by(&mut self, p: V, r: V) {
        self.center.rot_by(p, r);
    }

    fn translate(&mut self, p: V) {
        self.center.add(p);
    }

    fn scale(&mut self, p: V) {
        self.sphere_radius *= p.x;
        self.cube_half_extent.x *= p.x;
        self.cube_half_extent.y *= p.y;
        self.cube_half_extent.z *= p.z;
    }

    fn transform(&mut self) -> Box<&mut dyn Transformable> {
        Box::new(self)
    }
}

impl RayMarchingObject for MorphSphereCube {
    fn sdf(&self, p: V) -> f64 {
        let ds = self.sphere_sdf(p);
        let db = self.box_sdf(p);
        ds * (1.0 - self.blend) + db * self.blend
    }

    fn clone(&self) -> Box<dyn RayMarchingObject + Send + Sync> {
        Box::new(*self)
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("rust3d", W as u32, H as u32)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    let mut event_pump = sdl_context.event_pump()?;
    let mut camera: RayCamera = RayCamera::new(V{x: -3.0, y: 0.0, z: 0.0}, 0.0, 0.0, 0.0);
    let p2 = Sphere::new(V{x: 2.0, y: 1.0, z: 1.0}, 1.2, Material{ color: Color::WHITE, diffuse: 1.0 });
    let p2_hole = Sphere::new(V{x: 1.0, y: 1.0, z: 1.0}, 0.8, Material{ color: Color::WHITE, diffuse: 1.0 });
    let morph_material = Material { color: Color::RGB(240, 180, 255), diffuse: 1.0 };
    let morph_center = V { x: 2.0, y: 1.0, z: -2.0 };
    let morph = MorphSphereCube::new(
        morph_center,
        1.0,
        V { x: 0.9, y: 0.9, z: 0.9 },
        0.0,
        morph_material,
    );
    let mut rm_objs : RayMarchingScene = RayMarchingScene::new(0.005);
    rm_objs.add(p2);
    rm_objs.add_negative(p2_hole);
    rm_objs.add(morph);
    rm_objs.add_light(Light {
        position: V{x: 2.0, y: 2.0, z: 5.0},
        color: Color::YELLOW,
        intensity: 1.0,
    });
    rm_objs.add_light(Light {
        position: V{x: -2.0, y: 2.0, z: 3.0},
        color: Color::BLUE,
        intensity: 1.0,
    });
    let rm_objs = Arc::new(RwLock::new(rm_objs));
    let mut movement_handler = PlayerMovementController::new(&mut event_pump, &mut camera, MovementInputMap::get_default());
    let mut line_t = 0.0f64;
    let mut line_dir = 1.0f64;
    let mut morph_t = 0.0f64;
    'running: loop {
        if movement_handler.handle_input() {
            break 'running;
        }
        let mut objs: RayRenderScene = RayRenderScene::new();
        line_t += line_dir * 0.012;
        if line_t >= 1.0 {
            line_t = 1.0;
            line_dir = -1.0;
        } else if line_t <= 0.0 {
            line_t = 0.0;
            line_dir = 1.0;
        }
        morph_t += 0.045;
        let blend = 0.5 + 0.5 * morph_t.sin();

        {
            let mut scene = rm_objs.write().unwrap();
            scene.lights[0].rot_by(V::new(2.0,1.0,1.0), V::new(0.01, 0.0, -0.05));

            let hole_x = 1.0 + line_t * 2.0;
            scene.negative_objects[0] = Box::new(Sphere::new(
                V { x: hole_x, y: 1.0, z: 1.0 },
                0.8,
                Material { color: Color::WHITE, diffuse: 1.0 },
            ));

            scene.objects[1] = Box::new(MorphSphereCube::new(
                morph_center,
                1.0,
                V { x: 0.9, y: 0.9, z: 0.9 },
                blend,
                morph_material,
            ));
        }

        objs.wrap(Box::new(RayMarchingScene::wrapup(&rm_objs.read().unwrap())));
        render_multi(&mut canvas, objs, movement_handler.get_camera(), &W, &H);
        canvas.present();
    }
    Ok(())
}

pub fn render_multi(canvas : &mut Canvas<Window>, objs : RayRenderScene, camera : RayCamera, w_ : &usize, h_ : &usize) {
    println!("Setting up threads...");
    let (tx, rx) = mpsc::channel::<(usize, Vec<Color>)>();
    let n = 5;
    let camera_arc = Arc::new(camera);
    let objs =  Arc::new(objs);

    for i in 0..n {
        let camera_arc = Arc::clone(&camera_arc);
        let objs = Arc::clone(&objs);
        let tx = tx.clone();
        let w_ = w_.clone();
        let h_ = h_.clone();

        thread::spawn(move || {
            let section = camera_arc.render_modulus_multi(objs, w_, h_, i, n);
            let _ = tx.send((i.to_owned(), section));
        });
    }
    let now = Instant::now();

    for _ in 0..n {
        let section = rx.recv().unwrap();
        camera.draw_modulus(&section.1, canvas, section.0, n, *w_, *h_);
    }

    println!("Render took {}ms", now.elapsed().as_millis());
}