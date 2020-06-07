#[derive(Debug)]
pub struct Triangle (pub Vertex, pub Vertex, pub Vertex);

#[derive(Debug)]
pub struct Vertex {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}