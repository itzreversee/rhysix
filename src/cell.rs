use crate::sandbox::Material;

#[derive(Clone, Copy, Debug, PartialEq)]

pub struct Cell {
  pub material: Material,
  pub weight: usize,
  pub temperature: i8
}

impl Cell {
  pub fn oob() -> Self {
    Self {
      material: Material::OOB,
      weight: 999,
      temperature: 0
    }
  }

  pub fn air() -> Self {
    Self {
      material: Material::AIR,
      weight: 0,
      temperature: 0
    }
  }

  pub fn sand() -> Self {
    Self {
      material: Material::SAND,
      weight: 2,
      temperature: 20
    }
  }
  
  pub fn stone() -> Self {
    Self {
      material: Material::STONE,
      weight: 2,
      temperature: 20
    }
  }
  
  pub fn water() -> Self {
    Self {
      material: Material::WATER,
      weight: 1,
      temperature: 20
    }
  }
}