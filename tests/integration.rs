#![feature(test)]
extern crate test;
extern crate stl_rust;
use test::Bencher;

const TEST_ASSET_PATH: &str = "./testAssets";

struct TestFiles {
  stacked_cones: String,
  cube_minus_sphere: String,
  huge_file: String,
}

impl TestFiles {
  pub fn new() -> TestFiles {
    TestFiles {
      stacked_cones: format!("{}/{}", TEST_ASSET_PATH, "stacked_cones_slices_2.stl"),
      cube_minus_sphere: format!("{}/{}", TEST_ASSET_PATH, "cubeMinusSphere.stl"),
      huge_file: format!("{}/{}", TEST_ASSET_PATH, "sculpted_sphere.stl"),
    }
  }
}

#[test]
fn parse_and_load() {
  let test_files = TestFiles::new();
  let solid = stl_rust::load(&test_files.stacked_cones);
  assert_eq!(solid.triangles.len(), 4400)
}

#[test]
fn check_genus() {
  let test_files = TestFiles::new();
  let cube_minus_sphere = stl_rust::load(&test_files.cube_minus_sphere);
  let mesh = stl_rust::mesh::Mesh::from_solid(cube_minus_sphere);
  assert_eq!(mesh.genus(), 5)
}