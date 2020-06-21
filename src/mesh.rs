use std::collections::{HashMap, HashSet};
use std::fmt;
use crate::stl::{Solid};
use crate::stl::datatypes::{Vertex};

// While Solid stores a surface as triangles, the way an STL file does, Mesh stores a list of vertices and edges.
// Let's try storing them in an adjacency list
type VertexId = usize;
pub struct Mesh {
  vertices: HashMap<Vertex, VertexId>,
  vertex_lookup: HashMap<VertexId, Vertex>,
  adjacency_list: HashMap<VertexId, HashSet<VertexId>>,
}

impl Mesh {
  fn add_triangle(& mut self, a: Vertex, b: Vertex, c: Vertex) {
    let a_id_option = self.vertices.get(&a);
    let a_id;
    match a_id_option {
      None => {
        let a_copy = a.clone();
        self.vertices.insert(a, self.vertices.len());
        a_id = self.vertices.len()-1;
        self.vertex_lookup.insert(a_id, a_copy);
      },
      Some(x) => {
        a_id = *x;
      }
    }

    let b_id_option = self.vertices.get(&b);
    let b_id;
    match b_id_option {
      None => {
        let b_copy = b.clone();
        self.vertices.insert(b, self.vertices.len());
        b_id = self.vertices.len()-1;
        self.vertex_lookup.insert(b_id, b_copy);
      },
      Some(x) => {
        b_id = *x;
      }
    }
    
    let c_id_option = self.vertices.get(&c);
    let c_id;
    match c_id_option {
      None => {
        let c_copy = c.clone();
        self.vertices.insert(c, self.vertices.len());
        c_id = self.vertices.len()-1;
        self.vertex_lookup.insert(c_id, c_copy);
      },
      Some(x) => {
        c_id = *x;
      }
    }
    
    let a_neighbors = self.adjacency_list.entry(a_id).or_insert_with(HashSet::new);
    a_neighbors.insert(b_id);
    a_neighbors.insert(c_id);

    let b_neighbors = self.adjacency_list.entry(b_id).or_insert_with(HashSet::new);
    b_neighbors.insert(a_id);
    b_neighbors.insert(c_id);

    let c_neighbors = self.adjacency_list.entry(c_id).or_insert_with(HashSet::new);
    c_neighbors.insert(a_id);
    c_neighbors.insert(b_id);
  }
}

impl Mesh {
  pub fn from_solid(solid: Solid) -> Mesh {
    let mut mesh = Mesh {
      vertices: HashMap::new(),
      vertex_lookup: HashMap::new(),
      adjacency_list: HashMap::new(),
    };

    for v in solid.triangles {
      mesh.add_triangle(v.0, v.1, v.2);
    }

    mesh
  }
}

impl fmt::Display for Mesh {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let num_edges = self.adjacency_list.values().fold(0, |acc, x| acc + x.len());
    let first = self.vertices.keys().next().expect("no vertices in mesh!");
    let vertex_display = format!("{:?}", first);
    let first_edges = self.adjacency_list.get(&0).expect("no edges");
    let first_edges: Vec<&Vertex> = first_edges.iter().map(|id| self.vertex_lookup.get(id).unwrap()).collect();

    write!(f, "Mesh with {} vertices, {} edges\na vertex: {:?}, {:?}", self.vertices.len(), num_edges, vertex_display, first_edges)
  }
}