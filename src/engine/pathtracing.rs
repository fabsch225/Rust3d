use sdl2::pixels::Color;

use crate::engine::lighting::Light;
use crate::engine::utils::{rendering::{RayRenderable, Collision}, transformation::Transformable};
use crate::geometry::vector3::Vector3 as V3;

fn fract(x: f64) -> f64 {
    x - x.floor()
}

fn hash3(seed: f64) -> f64 {
    fract(f64::sin(seed * 12.9898) * 43758.5453123)
}

fn shade_point(
    lights: &Vec<Light>,
    objects: &Vec<Box<dyn RaytracingObject + Send + Sync>>,
    base_collision: Collision,
    mut normal: V3,
    view_origin: V3,
    samples: usize,
    ambient_light: Color,
) -> Color {
    normal.normalize();

    let ar = ambient_light.r as f64 / 255.0;
    let ag = ambient_light.g as f64 / 255.0;
    let ab = ambient_light.b as f64 / 255.0;

    let mut r = base_collision.c.r as f64 * ar;
    let mut g = base_collision.c.g as f64 * ag;
    let mut b = base_collision.c.b as f64 * ab;

    let samples = usize::max(samples, 1);

    for (light_index, light) in lights.iter().enumerate() {
        let mut to_light = light.position;
        to_light.subtract(base_collision.p);
        let light_dist = to_light.norm();

        let mut base_light_dir = to_light;
        base_light_dir.normalize();

        let mut tangent = if f64::abs(normal.x) < 0.9 {
            V3::new(1.0, 0.0, 0.0)
        } else {
            V3::new(0.0, 1.0, 0.0)
        };
        tangent.cross(normal);
        tangent.normalize();

        let mut bitangent = normal;
        bitangent.cross(tangent);
        bitangent.normalize();

        let mut local_r = 0.0;
        let mut local_g = 0.0;
        let mut local_b = 0.0;

        for sample in 0..samples {
            let seed = base_collision.p.x * 13.0
                + base_collision.p.y * 17.0
                + base_collision.p.z * 19.0
                + light_index as f64 * 23.0
                + sample as f64 * 29.0;

            let ru = hash3(seed * 1.31) * 2.0 - 1.0;
            let rv = hash3(seed * 1.73) * 2.0 - 1.0;

            let mut jitter_u = tangent;
            jitter_u.scale(ru * 0.08);
            let mut jitter_v = bitangent;
            jitter_v.scale(rv * 0.08);

            let mut light_dir = base_light_dir;
            light_dir.add(jitter_u);
            light_dir.add(jitter_v);
            light_dir.normalize();

            // hard shadow ray
            let mut normal_offset = normal;
            normal_offset.scale(1e-4);

            let mut shadow_origin = base_collision.p;
            shadow_origin.add(normal_offset);

            let mut shadowed = false;
            for object in objects.iter() {
                let shadow_coll = object.get_collision(shadow_origin, light_dir);
                if shadow_coll.hit && shadow_coll.d > 1e-6 && shadow_coll.d < light_dist - 1e-4 {
                    shadowed = true;
                    break;
                }
            }

            if shadowed {
                continue;
            }

            let ndotl = f64::max(normal.dt(light_dir), 0.0);
            let diffuse = light.intensity * ndotl;

            let mut view_dir = view_origin;
            view_dir.subtract(base_collision.p);
            view_dir.normalize();

            let mut half_vec = light_dir;
            half_vec.add(view_dir);
            half_vec.normalize();

            let specular = f64::powf(f64::max(normal.dt(half_vec), 0.0), 36.0) * light.intensity * 0.35;

            let lr = light.color.r as f64 / 255.0;
            let lg = light.color.g as f64 / 255.0;
            let lb = light.color.b as f64 / 255.0;

            local_r += base_collision.c.r as f64 * diffuse * lr + 255.0 * specular * lr;
            local_g += base_collision.c.g as f64 * diffuse * lg + 255.0 * specular * lg;
            local_b += base_collision.c.b as f64 * diffuse * lb + 255.0 * specular * lb;
        }

        r += local_r / samples as f64;
        g += local_g / samples as f64;
        b += local_b / samples as f64;
    }

    Color::RGB(
        f64::clamp(r, 0.0, 255.0) as u8,
        f64::clamp(g, 0.0, 255.0) as u8,
        f64::clamp(b, 0.0, 255.0) as u8,
    )
}


pub trait RaytracingObject : Transformable {
    fn d(&self, p: V3) -> f64;
    fn color(&self, p : V3) -> Color;
    fn is_colliding(&mut self, p0 : V3, p : V3) -> bool; //Todo
    fn get_collision(&self, p0 : V3, p : V3) -> Collision;
    fn get_collision_with_normal(&self, p0 : V3, p : V3) -> (Collision, Option<V3>) {
        (self.get_collision(p0, p), None)
    }
    fn reflectivity(&self, _p: V3) -> f64 {
        0.0
    }
    fn clone(&self) -> Box<dyn RaytracingObject + Send + Sync>;
}

pub use RaytracingObject as PathtracingObject;

pub struct RayTracingScene {
    pub objects: Vec<Box<dyn RaytracingObject + Send + Sync>>,
    pub lights: Vec<Light>,
    pub shading_samples: usize,
    pub ambient_light: Color,
}

pub type PathTracingScene = RayTracingScene;

impl RayTracingScene {
    pub fn new() -> Self {
        RayTracingScene {
            objects: Vec::new(),
            lights: Vec::new(),
            shading_samples: 1,
            ambient_light: Color::RGB(31, 31, 31),
        }
    }

    pub fn wrapup(old : &RayTracingScene) -> Self {
        let mut objects_vec: Vec<Box<dyn RaytracingObject + Send + Sync>> = Vec::new();
        for i in 0..old.objects.len() {
            objects_vec.push(old.objects[i].clone());
        }
        RayTracingScene {
            objects: objects_vec,
            lights: old.lights.clone(),
            shading_samples: old.shading_samples,
            ambient_light: old.ambient_light,
        }
    }

    pub fn get(&mut self, i : usize) -> &mut Box<dyn RaytracingObject + 'static + Send + Sync> {
        &mut self.objects[i]
    }

    pub fn remove(&mut self, i : usize) {
        self.objects.remove(i);
    }

    pub fn remove_and_clone(&mut self, i : usize) -> Box<dyn RaytracingObject> {
        let obj = self.objects[i].clone();
        self.objects.remove(i);
        obj
    }

    pub fn add(&mut self, obj : impl RaytracingObject + 'static + Send + Sync) {
        self.objects.push(Box::new(obj));
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn set_shading_samples(&mut self, samples: usize) {
        self.shading_samples = usize::max(samples, 1);
    }

    pub fn set_ambient_light(&mut self, ambient_light: Color) {
        self.ambient_light = ambient_light;
    }

    fn trace_closest(&self, p0: V3, p: V3) -> (Collision, Option<V3>, f64) {
        let mut c: Collision = Collision::empty();
        let mut bd: f64 = f64::MAX;
        let mut n: Option<V3> = None;
        let mut reflectivity: f64 = 0.0;

        for po in self.objects.iter() {
            let (c_, n_) = po.get_collision_with_normal(p0, p);
            if c_.hit && c_.d < bd {
                c = c_;
                bd = c_.d;
                n = n_;
                reflectivity = po.reflectivity(c_.p);
            }
        }

        (c, n, reflectivity)
    }
}

impl RayRenderable for RayTracingScene {
	fn get_collision(&self, p0 : V3, p : V3, radius : f64) -> Collision {
        let (mut c, n, reflectivity) = self.trace_closest(p0, p);

        if c.hit {
            if let Some(normal) = n {
                if !self.lights.is_empty() {
                    let direct = shade_point(
                        &self.lights,
                        &self.objects,
                        c,
                        normal,
                        p0,
                        self.shading_samples,
                        self.ambient_light,
                    );

                    let refl = f64::clamp(reflectivity, 0.0, 1.0);
                    if refl > 0.0 {
                        let mut in_dir = p;
                        in_dir.normalize();

                        let mut nrm = normal;
                        nrm.normalize();

                        // Fresnel-Schlick: stronger reflection at grazing angles.
                        let mut view_dir = in_dir;
                        view_dir.scale(-1.0);
                        let cos_theta = f64::clamp(nrm.dt(view_dir), 0.0, 1.0);
                        let fresnel = refl + (1.0 - refl) * f64::powf(1.0 - cos_theta, 5.0);

                        let proj = 2.0 * in_dir.dt(nrm);
                        let mut refl_dir = in_dir;
                        let mut n_scaled = nrm;
                        n_scaled.scale(proj);
                        refl_dir.subtract(n_scaled);
                        refl_dir.normalize();

                        let mut offset = nrm;
                        offset.scale(1e-4);
                        let mut refl_origin = c.p;
                        refl_origin.add(offset);

                        let (mut rc, rn, _rr) = self.trace_closest(refl_origin, refl_dir);
                        if rc.hit {
                            if let Some(rn_) = rn {
                                rc.c = shade_point(
                                    &self.lights,
                                    &self.objects,
                                    rc,
                                    rn_,
                                    refl_origin,
                                    self.shading_samples,
                                    self.ambient_light,
                                );
                            }
                        }

                        // Keep non-reflected shading visible even at grazing angles.
                        let min_direct = 0.35;
                        let reflection_mix = f64::clamp(fresnel * (1.0 - min_direct), 0.0, 1.0 - min_direct);

                        c.c = Color::RGB(
                            f64::clamp(direct.r as f64 * (1.0 - reflection_mix) + rc.c.r as f64 * reflection_mix, 0.0, 255.0) as u8,
                            f64::clamp(direct.g as f64 * (1.0 - reflection_mix) + rc.c.g as f64 * reflection_mix, 0.0, 255.0) as u8,
                            f64::clamp(direct.b as f64 * (1.0 - reflection_mix) + rc.c.b as f64 * reflection_mix, 0.0, 255.0) as u8,
                        );
                    } else {
                        c.c = direct;
                    }
                }
            }
        }
		c
	}
}
