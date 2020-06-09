#![feature(test)]
extern crate test;
extern crate stl_rust;
use test::Bencher;

const TEST_ASSET_PATH: &str = "./testAssets";
const FILE_NAME: &str = "stacked_cones_slices_2.stl";

#[test]
fn parse_and_load() {
  let solid = stl_rust::load(&format!("{}/{}", TEST_ASSET_PATH, FILE_NAME));
  assert_eq!(solid.triangles.len(), 4400)
}

#[bench]
fn benchmark_parse_and_load(b: &mut Bencher) {
  b.iter(|| stl_rust::load(&format!("{}/{}", TEST_ASSET_PATH, FILE_NAME)));
}