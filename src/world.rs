use point::Point;
use vector::Vector;
use geometry::*;
use light::Light;
use std::f64;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Scene {
  pub width: u32,
  pub height: u32,
  pub max_reflections: u8,
  pub camera: Camera,
  pub elements: Vec<Geometry>,
  pub lights: Vec<Light>,
}
impl Scene {
  pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
    self
      .elements
      .iter()
      .filter_map(|s| s.intersects(ray).map(|d| Intersection::new(d, s)))
      .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Camera {
  pub position: Point,
}
impl Camera {
  pub fn create_prime<'a>(&self, x: u32, y: u32, scene: &Scene) -> Ray {
    let fov = (45f64.to_radians() / 2.0).tan();
    let aspect_ratio = (scene.width as f64) / (scene.height as f64);
    let sensor_x = (((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio * fov;
    let sensor_y = 1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0 * fov;
    let p = Point {
      x: sensor_x,
      y: sensor_y,
      z: -1.0,
    };
    let direction = (p - self.position).normalise();
    Ray {
      direction: direction,
      origin: &self.position,
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct Ray<'a> {
  pub origin: &'a Point,
  pub direction: Vector,
}
impl<'a> Ray<'a> {
  pub fn phit(&self, t: f64) -> Point {
    let a = self.direction * t;
    return self.origin + a;
  }
}

pub struct Intersection<'a> {
  pub distance: f64,
  pub object: &'a Geometry,
}
impl<'a> Intersection<'a> {
  pub fn new<'b>(d: f64, o: &'b Geometry) -> Intersection {
    Intersection {
      distance: d,
      object: o,
    }
  }
}
