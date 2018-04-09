use std::ops::{Add, Mul};
use image::{Pixel, Rgba};
const GAMMA: f32 = 2.2;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Colour {
  pub red: f32,
  pub green: f32,
  pub blue: f32,
}
impl Colour {
  pub fn to_rgba(&self) -> Rgba<u8> {
    Rgba::from_channels(
      (self.red.min(1.0).max(0.0).powf(1.0 / GAMMA) * 255.0) as u8,
      (self.green.min(1.0).max(0.0).powf(1.0 / GAMMA) * 255.0) as u8,
      (self.blue.min(1.0).max(0.0).powf(1.0 / GAMMA) * 255.0) as u8,
      255,
    )
  }
  pub fn black() -> Colour {
    return Colour {
      red: 0.0,
      green: 0.0,
      blue: 0.0,
    };
  }
}
impl<'a, 'b> Mul<&'b Colour> for &'a Colour {
  type Output = Colour;
  fn mul(self, rhs: &'b Colour) -> Colour {
    Colour {
      red: self.red * rhs.red,
      green: self.green * rhs.green,
      blue: self.blue * rhs.blue,
    }
  }
}
impl Mul for Colour {
  type Output = Colour;
  fn mul(self, rhs: Colour) -> Colour {
    Colour {
      red: self.red * rhs.red,
      green: self.green * rhs.green,
      blue: self.blue * rhs.blue,
    }
  }
}
impl Mul<f32> for Colour {
  type Output = Colour;
  fn mul(self, op: f32) -> Colour {
    Colour {
      red: self.red * op,
      green: self.green * op,
      blue: self.blue * op,
    }
  }
}
impl<'a> Mul<f32> for &'a Colour {
  type Output = Colour;
  fn mul(self, op: f32) -> Colour {
    Colour {
      red: self.red * op,
      green: self.green * op,
      blue: self.blue * op,
    }
  }
}
impl Add for Colour {
  type Output = Colour;
  fn add(self, rhs: Colour) -> Colour {
    Colour {
      red: self.red + rhs.red,
      green: self.green + rhs.green,
      blue: self.blue + rhs.blue,
    }
  }
}
impl Add<f32> for Colour {
  type Output = Colour;
  fn add(self, rhs: f32) -> Colour {
    Colour {
      red: self.red + rhs,
      green: self.green + rhs,
      blue: self.blue + rhs,
    }
  }
}
