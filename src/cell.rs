use crate::sandbox::Material;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellType {
  NOTHING,
  SOLID,
  LIQUID
}

#[derive(Clone, Copy, Debug, PartialEq)]

pub struct Cell {
  pub material: Material,
  pub weight: usize,
  pub cell_type: CellType,
  pub temperature: i8
}

impl Cell {
  pub fn oob() -> Self {
    Self {
      material: Material::OOB,
      weight: 999,
      cell_type: CellType::NOTHING,
      temperature: 0
    }
  }

  pub fn air() -> Self {
    Self {
      material: Material::AIR,
      weight: 0,
      cell_type: CellType::NOTHING,
      temperature: 0
    }
  }

  pub fn sand() -> Self {
    Self {
      material: Material::SAND,
      weight: 2,
      cell_type: CellType::SOLID,
      temperature: 20
    }
  }
  
  pub fn stone() -> Self {
    Self {
      material: Material::STONE,
      weight: 2,
      cell_type: CellType::SOLID,
      temperature: 20
    }
  }
  
  pub fn water() -> Self {
    Self {
      material: Material::WATER,
      weight: 1,
      cell_type: CellType::LIQUID,
      temperature: 20
    }
  }
}