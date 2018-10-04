extern crate image;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::fs::File;
use std::io::prelude::*;

use image::*;

mod colour;
mod geometry;
mod light;
mod point;
mod vector;
mod world;

use colour::Colour;
use geometry::*;
use light::Light;
use point::Point;
use vector::Vector;
use world::*;

fn main() {
  let scene = read_scene();

  let mut fout = File::create("test.png").unwrap();
  let img = render(&scene);
  // img.save(&mut fout, image::PNG).unwrap();
  img.write_to(&mut fout, image::PNG).unwrap();
}

fn read_scene() -> Scene {
  let mut f = File::open("scene.yml").unwrap();
  let mut contents = String::new();
  f.read_to_string(&mut contents).expect("couldn't read file");
  let scene: Scene = serde_yaml::from_str(&contents).unwrap();
  scene
}

fn render(scene: &Scene) -> DynamicImage {
  let mut img = DynamicImage::new_rgb8(scene.width, scene.height);
  for x in 0..scene.width {
    for y in 0..scene.height {
      let ray = scene.camera.create_prime(x, y, scene);
      let colour = recurse_ray(&ray, scene, scene.max_reflections);
      img.put_pixel(x, y, colour.to_rgba());
    }
  }
  img
}

fn recurse_ray(ray: &Ray, scene: &Scene, depth: u8) -> Colour {
  let mut colour = Colour::black();
  if depth == 0 {
    return colour;
  }
  match scene.trace(ray) {
    Some(ref intersection) => {
      let phit = ray.phit(intersection.distance);
      let normal = intersection.object.surface_normal(&phit);
      for light in &scene.lights {
        let direction_to_light = light.direction(&phit);
        if in_light(&phit, light, scene, &direction_to_light) {
          let diffuse = diffuse_light(
            &normal,
            &direction_to_light,
            &phit,
            light,
            intersection.object,
          );
          colour = colour + diffuse;
          if let &SurfaceType::Reflective(reflectivity) = intersection.object.surface() {
            let reflected_light = reflective_light(&normal, &ray.direction, &phit, scene, depth);
            colour = colour + (reflected_light * reflectivity);
            let specular_light = phong_lighting(
              light,
              &direction_to_light,
              ray,
              &phit,
              &normal,
              intersection.object,
            );
            // let specular_light =
            //     blinn_light(&normal, &direction_to_light, &ray.direction, light, &phit);
            colour = colour + specular_light;
          }
        }
      }
      return colour;
    }
    None => return colour,
  };
}

fn in_light(phit: &Point, light: &Light, scene: &Scene, dir: &Vector) -> bool {
  let shadow_ray = Ray {
    origin: phit,
    direction: *dir,
  };
  return match scene.trace(&shadow_ray) {
    None => true,
    Some(ref sh_intersect) => sh_intersect.distance > light.distance(&phit),
  };
}

fn diffuse_light(
  surface_normal: &Vector,
  direction_to_light: &Vector,
  phit: &Point,
  light: &Light,
  obj: &Geometry,
) -> Colour {
  let light_intensity = light.intensity(&phit) * (obj.albedo() / std::f32::consts::PI);
  let light_power = (surface_normal.dot(direction_to_light) as f32) * light_intensity;
  let light_colour = obj.colour() * light.colour() * light_power;
  return light_colour;
}

fn reflective_light(
  surface_normal: &Vector,
  ray_direction: &Vector,
  phit: &Point,
  scene: &Scene,
  reflections: u8,
) -> Colour {
  let ray = Ray {
    origin: phit,
    direction: Vector::reflect(surface_normal, ray_direction),
  };
  let colour = recurse_ray(&ray, scene, reflections - 1);
  return colour;
}

fn phong_lighting(
  light: &Light,
  direction_to_light: &Vector,
  ray: &Ray,
  phit: &Point,
  surface_normal: &Vector,
  obj: &Geometry,
) -> Colour {
  let mut colour = Colour::black();
  let reflected = Vector::reflect(surface_normal, direction_to_light);
  let dot = reflected.dot(&ray.direction);
  if dot > 0.0 {
    let pow = dot.powf(obj.shininess() as f64);
    let specular = light.intensity(&phit) * (pow as f32) * 0.5;
    colour = colour + specular;
  }
  return colour;
}
// fn blinn_light(
//     surface_normal: &Vector,
//     direction_to_light: &Vector,
//     ray_direction: &Vector,
//     light: &Light,
//     phit: &Point,
// ) -> Colour {
//     let mut cos = surface_normal.dot(direction_to_light);
//     cos = cos.max(0.0).min(1.0);
//     let half = (direction_to_light + ray_direction).normalise();
//     let mut blinn = surface_normal.dot(&half);
//     blinn = blinn.max(0.0).min(1.0);
//     blinn = if cos != 0.0 { blinn } else { cos };
//     blinn = blinn.powf(7500 as f64);
//     let specular = blinn as f32;
//     // let specular = blinn as f32 * light.intensity(&phit);
//     return Colour::black() + specular;
// }
