extern crate stl_rust;

const TEST_ASSET_PATH: &str = "./testAssets";
const FILE_NAME: &str = "stacked_cones_slices_2.stl";

#[test]
fn parse_and_load() {
  let solid = stl_rust::load(&format!("{}/{}", TEST_ASSET_PATH, FILE_NAME));
  assert_eq!(solid.triangles.len(), 4400)
}