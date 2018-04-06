extern crate image;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::io::prelude::*;
use std::fs::File;

use image::*;

mod world;
mod point;
mod vector;
mod geometry;
mod light;
mod colour;

use world::*;
use point::Point;
use vector::Vector;
use light::Light;
use colour::Colour;
use geometry::Intersectable;

fn main() {
    let scene = read_scene();

    let mut fout = File::create("test.png").unwrap();
    let img = render(&scene);
    img.save(&mut fout, image::PNG).unwrap();
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
    let black = Rgba::from_channels(0, 0, 0, 255);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = scene.camera.create_prime(x, y, scene);
            match &scene.trace(&ray) {
                &Some(ref intersection) => {
                    let phit = ray.phit(intersection.distance);
                    let normal = intersection.object.surface_normal(&phit);
                    let mut colour = Colour {
                        red: 0.0,
                        green: 0.0,
                        blue: 0.0,
                    };
                    for light in &scene.lights {
                        let direction_to_light = light.direction(&phit);
                        if in_light(&phit, light, scene) {
                            let light_intensity = light.intensity(&phit);
                            let light_power =
                                (normal.dot(&direction_to_light) as f32) * light_intensity;
                            let light_reflected =
                                intersection.object.albedo() / std::f32::consts::PI;
                            let light_colour = intersection.object.colour() * light.colour()
                                * light_power
                                * light_reflected;
                            colour = colour + (&light_colour * intersection.object.colour());
                        }
                    }
                    img.put_pixel(x, y, colour.to_rgba());
                }
                &None => img.put_pixel(x, y, black),
            }
        }
    }
    img
}

fn in_light(phit: &Point, light: &Light, scene: &Scene) -> bool {
    let shadow_ray = Ray {
        origin: phit,
        direction: light.direction(phit),
    };
    return match scene.trace(&shadow_ray) {
        None => true,
        Some(ref sh_intersect) => sh_intersect.distance > light.distance(&phit),
    };
}
