use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Triangle (pub Vertex, pub Vertex, pub Vertex);

#[derive(Debug)]
pub struct Vertex {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Eq for Vertex {}

impl PartialEq for Vertex {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x &&
    self.y == other.y &&
    self.z == other.z
  }
}

impl Hash for Vertex {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.x.to_bits().hash(state);
    self.y.to_bits().hash(state);
    self.z.to_bits().hash(state);
  }
}
