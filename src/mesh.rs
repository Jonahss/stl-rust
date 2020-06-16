use std::collections::{HashMap, HashSet};
use std::fmt;
use crate::stl::{Solid};
use crate::stl::datatypes::{Vertex};

// While Solid stores a surface as triangles, the way an STL file does, Mesh stores a list of vertices and edges.
// Let's try storing them in an adjacency list
pub struct Mesh<'a> {
  vertices: HashMap<Vertex, usize>,
  adjacency_list: HashMap<usize, HashSet<&'a Vertex>>,
}

impl<'a> Mesh<'a> {
  fn add_triangle(& mut self, a: Vertex, b: Vertex, c: Vertex) {
    let a_id_option = self.vertices.get(&a);
    let a_id;
    match a_id_option {
      None => {
        self.vertices.insert(a, self.vertices.len());
        a_id = self.vertices.len()-1;
      },
      Some(x) => {
        a_id = *x;
      }
    }

    let b_id_option = self.vertices.get(&b);
    let b_id;
    match b_id_option {
      None => {
        self.vertices.insert(b, self.vertices.len());
        b_id = self.vertices.len()-1;
      },
      Some(x) => {
        b_id = *x;
      }
    }
    
    let c_id_option = self.vertices.get(&c);
    let c_id;
    match c_id_option {
      None => {
        self.vertices.insert(c, self.vertices.len());
        c_id = self.vertices.len()-1;
      },
      Some(x) => {
        c_id = *x;
      }
    }
    
    let a_neighbors = self.adjacency_list.entry(a_id).or_insert_with(HashSet::new);
    //a_neighbors.insert(b_id);
    //a_neighbors.insert(c_id);

    let b_neighbors = self.adjacency_list.entry(b_id).or_insert_with(HashSet::new);
    //b_neighbors.insert(a_id);
    //b_neighbors.insert(c_id);

    let c_neighbors = self.adjacency_list.entry(c_id).or_insert_with(HashSet::new);
    //c_neighbors.insert(a_id);
    //c_neighbors.insert(b_id);
  }
}

impl<'a> Mesh<'a> {
  pub fn from_solid(solid: Solid) -> Mesh<'a> {
    let mut mesh = Mesh {
      vertices: HashMap::new(),
      adjacency_list: HashMap::new(),
    };

    for v in solid.triangles {
      mesh.add_triangle(v.0, v.1, v.2);
    }

    mesh
  }
}

impl fmt::Display for Mesh<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let num_edges = self.adjacency_list.values().fold(0, |acc, x| acc + x.len());
    let first = self.vertices.keys().next().expect("no vertices in mesh!");
    let vertex_display = format!("{:?}", first);
    let first_edges = self.adjacency_list.get(&0).expect("no edges");

    write!(f, "Mesh with {} vertices, {} edges\na vertex: {:?}, {:?}", self.vertices.len(), num_edges, vertex_display, first_edges)
  }
}