use point::Point;
use vector::Vector;
use colour::Colour;
use world::Ray;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Sphere {
  pub center: Point,
  pub radius: f64,
  pub colour: Colour,
  pub albedo: f32,
}
impl Intersectable for Sphere {
  fn intersects(&self, ray: &Ray) -> Option<f64> {
    let l: Vector = &self.center - ray.origin;
    let tca = l.dot(&ray.direction);
    if tca < 0.0 {
      return None;
    }
    let d2 = l.dot(&l) - (tca * tca);
    let radius2 = self.radius * self.radius;
    if d2 > radius2 {
      return None;
    }
    let thc = (radius2 - d2).sqrt();
    let t0 = tca - thc;
    let t1 = tca + thc;
    if t0 < 0.0 && t1 < 0.0 {
      return None;
    }
    let distance = t0.min(t1);
    return Some(distance);
  }
  fn surface_normal(&self, p: &Point) -> Vector {
    return p.normal(&self.center);
  }
  // fn colour(&self) -> &Colour {
  //   return &self.colour;
  // }
  // fn albedo(&self) -> f32 {
  //   return self.albedo;
  // }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Plane {
  pub origin: Point,
  pub normal: Vector,
  pub colour: Colour,
  pub albedo: f32,
}
impl Intersectable for Plane {
  fn intersects(&self, ray: &Ray) -> Option<f64> {
    let denom = self.normal.dot(&ray.direction);
    if denom > 1e-6 {
      let v = &self.origin - ray.origin;
      let dist = v.dot(&self.normal) / denom;
      if dist >= 0f64 {
        return Some(dist);
      }
    }
    return None;
  }
  fn surface_normal(&self, _: &Point) -> Vector {
    return -self.normal;
  }
  // fn colour(&self) -> &Colour {
  //   return &self.colour;
  // }
  // fn albedo(&self) -> f32 {
  //   return self.albedo;
  // }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Geometry {
  Sphere(Sphere),
  Plane(Plane),
}
impl Intersectable for Geometry {
  fn intersects(&self, ray: &Ray) -> Option<f64> {
    match *self {
      Geometry::Sphere(ref s) => s.intersects(ray),
      Geometry::Plane(ref p) => p.intersects(ray),
    }
  }
  fn surface_normal(&self, phit: &Point) -> Vector {
    match *self {
      Geometry::Sphere(ref s) => s.surface_normal(phit),
      Geometry::Plane(ref p) => p.surface_normal(phit),
    }
  }
}
impl Geometry {
  pub fn colour(&self) -> &Colour {
    match *self {
      Geometry::Sphere(ref s) => &s.colour,
      Geometry::Plane(ref p) => &p.colour,
    }
  }
  pub fn albedo(&self) -> f32 {
    match *self {
      Geometry::Sphere(ref s) => s.albedo,
      Geometry::Plane(ref p) => p.albedo,
    }
  }
}

pub trait Intersectable {
  fn intersects(&self, &Ray) -> Option<f64>;
  fn surface_normal(&self, &Point) -> Vector;
  // fn colour(&self) -> &Colour;
  // fn albedo(&self) -> f32;
}
