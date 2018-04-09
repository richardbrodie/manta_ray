use point::Point;
use vector::Vector;
use colour::Colour;
use std::{f32, f64};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectionalLight {
  pub direction: Vector,
  pub colour: Colour,
  pub intensity: f32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SphericalLight {
  pub position: Point,
  pub colour: Colour,
  pub intensity: f32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Light {
  Directional(DirectionalLight),
  Spherical(SphericalLight),
}
impl Light {
  pub fn direction(&self, p: &Point) -> Vector {
    match *self {
      Light::Directional(ref d) => -d.direction.normalise(),
      Light::Spherical(ref s) => (&s.position - p).normalise(),
    }
  }
  pub fn intensity(&self, p: &Point) -> f32 {
    match *self {
      Light::Directional(ref d) => d.intensity,
      Light::Spherical(ref s) => {
        let v = &s.position - p;
        let r2 = v.dot(&v) as f32;
        s.intensity / (4.0 * f32::consts::PI * r2)
      }
    }
  }
  pub fn colour(&self) -> &Colour {
    match *self {
      Light::Directional(ref s) => &s.colour,
      Light::Spherical(ref p) => &p.colour,
    }
  }
  pub fn distance(&self, p: &Point) -> f64 {
    match *self {
      Light::Directional(_) => f64::INFINITY,
      Light::Spherical(ref s) => (&s.position - p).magnitude(),
    }
  }
}
