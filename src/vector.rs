use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct Vector {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}
impl Vector {
  pub fn normalise(&self) -> Vector {
    let inv_len = 1f64 / self.magnitude();
    return Vector {
      x: self.x * inv_len,
      y: self.y * inv_len,
      z: self.z * inv_len,
    };
  }
  pub fn magnitude(&self) -> f64 {
    let sqr = self.dot(&self);
    sqr.sqrt()
  }
  pub fn dot(&self, rhs: &Vector) -> f64 {
    return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z;
  }

  pub fn reflect(normal: &Vector, direction: &Vector) -> Vector {
    return direction - (normal * direction.dot(normal) * 2f64);
  }
}

impl Mul<f64> for Vector {
  type Output = Vector;
  fn mul(self, op: f64) -> Vector {
    Vector {
      x: self.x * op,
      y: self.y * op,
      z: self.z * op,
    }
  }
}
impl<'a> Mul<f64> for &'a Vector {
  type Output = Vector;
  fn mul(self, op: f64) -> Vector {
    Vector {
      x: self.x * op,
      y: self.y * op,
      z: self.z * op,
    }
  }
}

impl Neg for Vector {
  type Output = Vector;
  fn neg(self) -> Vector {
    Vector {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }
}

impl<'a, 'b> Sub<&'b Vector> for &'a Vector {
  type Output = Vector;
  fn sub(self, rhs: &'b Vector) -> Vector {
    Vector {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}
impl<'a> Sub<Vector> for &'a Vector {
  type Output = Vector;
  fn sub(self, rhs: Vector) -> Vector {
    Vector {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}

impl<'a, 'b> Add<&'b Vector> for &'a Vector {
  type Output = Vector;
  fn add(self, rhs: &'b Vector) -> Vector {
    Vector {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}
