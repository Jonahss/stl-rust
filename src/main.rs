use stl_rust as stl;

use std::fmt;

#[derive(Debug)]
struct Moo(i32);

impl fmt::Display for Moo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "moo")
    }
}

#[derive(Debug)]
struct Foo(Moo);


const TEST_ASSET_PATH: &str = "./testAssets";
const FILE_NAME: &str = "sculpted_sphere.stl";

fn main() {
    let a = "a";
    println!("Hello, world!");
    println!("{:?}", a);

    let b = Moo(3);
    println!("{}", b);

    println!("{}", f64::MAX);

    println!("{}", "-2e2".parse::<f32>().unwrap());

    let solid = stl::load(&format!("{}/{}", TEST_ASSET_PATH, FILE_NAME));

    println!("{:?}", solid.triangles.len())
}