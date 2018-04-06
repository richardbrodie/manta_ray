use std::ops::{Add, Sub};
use vector::Vector;

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct Point {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}
impl Point {
  pub fn normal(&self, p: &Point) -> Vector {
    let vec = self - p;
    return vec.normalise();
  }
}
impl Sub for Point {
  type Output = Vector;
  fn sub(self, rhs: Point) -> Vector {
    Vector {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}
impl<'a, 'b> Sub<&'b Point> for &'a Point {
  type Output = Vector;
  fn sub(self, rhs: &'b Point) -> Vector {
    Vector {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}
impl Add<Vector> for Point {
  type Output = Point;
  fn add(self, rhs: Vector) -> Point {
    Point {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}
impl<'a> Add<Vector> for &'a Point {
  type Output = Point;
  fn add(self, rhs: Vector) -> Point {
    Point {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}
