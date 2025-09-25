use crate::sandbox::Material;

#[derive(Clone, Copy, Debug, PartialEq)]

pub struct Cell {
  pub material: Material,
  pub temperature: i8
}

impl Cell {
  pub fn oob() -> Self {
    Self {
      material: Material::OOB,
      temperature: 0
    }
  }

  pub fn air() -> Self {
    Self {
      material: Material::AIR,
      temperature: 0
    }
  }

  pub fn sand() -> Self {
    Self {
      material: Material::SAND,
      temperature: 20
    }
  }
}