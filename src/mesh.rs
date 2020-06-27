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
  num_triangles: i32,
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

    self.num_triangles += 1;
  }
}

impl Mesh {
  pub fn from_solid(solid: Solid) -> Mesh {
    let mut mesh = Mesh {
      vertices: HashMap::new(),
      vertex_lookup: HashMap::new(),
      adjacency_list: HashMap::new(),
      num_triangles: 0,
    };

    for v in solid.triangles {
      mesh.add_triangle(v.0, v.1, v.2);
    }

    mesh
  }

  pub fn num_edges(&self) -> i32 {
    (self.adjacency_list.values().fold(0, |acc, x| acc + x.len()) / 2) as i32 // each edge is listed twice, once for each starting vertex
  }

  pub fn num_vertices(&self) -> i32 {
    self.vertices.len() as i32
  }

  pub fn average_vertex_valence(&self) -> i32 {
    self.adjacency_list.values().map(|edges| edges.len()).sum::<usize>() as i32 / self.num_vertices()
  }

  pub fn genus(&self) -> i32 {
    let num_edges = self.num_edges();
    let num_vertices = self.num_vertices();
    1 - ((num_vertices - num_edges + self.num_triangles) / 2)
  }
}

impl fmt::Display for Mesh {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let num_edges = self.num_edges();
    let num_vertices = self.num_vertices();
    let first = self.vertices.keys().next().expect("no vertices in mesh!");
    let vertex_display = format!("{:?}", first);
    let first_edges = self.adjacency_list.get(&0).expect("no edges");
    let first_edges: Vec<&Vertex> = first_edges.iter().map(|id| self.vertex_lookup.get(id).unwrap()).collect();

    writeln!(f, "Mesh with {} vertices, {} edges, {} triangles", num_vertices, num_edges, self.num_triangles);
    writeln!(f, "F ≈ 2V ({} ≈ {})", self.num_triangles, 2 * num_vertices);
    writeln!(f, "E ≈ 3V ({} ≈ {})", self.num_edges(), 3 * num_vertices);
    writeln!(f, "Average Vertex Valence ≈ 6 ({} ≈ 6)", self.average_vertex_valence());
    writeln!(f, "Euler formula: V - E + F = 2(1-g)  (g = {})", self.genus());
    writeln!(f, "a vertex: {:?}, {:?}", vertex_display, first_edges)
  }
}