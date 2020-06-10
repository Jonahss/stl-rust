#![feature(test)]
extern crate test;
extern crate stl_rust;
use test::Bencher;

const TEST_ASSET_PATH: &str = "./testAssets";
const FILE_NAME: &str = "sculpted_sphere.stl";


#[bench]
fn benchmark_parse_and_load(b: &mut Bencher) {
  b.iter(|| {
    let model = stl_rust::load(&format!("{}/{}", TEST_ASSET_PATH, FILE_NAME));
    assert_eq!(model.name, "Exported from Blender-2.83.0")
  });
}