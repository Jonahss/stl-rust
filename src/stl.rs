mod datatypes;

use std::fs;

#[derive(Debug)]
pub struct Solid {
  pub triangles: Vec<datatypes::Triangle>,
  pub name: String,
}

impl Solid {
  pub fn new(triangles: Vec<datatypes::Triangle>, name: String) -> Solid {
    Solid {
      triangles,
      name,
    }
  }
}

pub fn load(path: &str) -> Solid {
  println!("load {}", path);

  let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
  
  println!("length {}", contents.len());
  println!("first 200:\n{}", &contents[..200]);
  println!("last 200:\n{}", &contents[contents.len()-200..]);
  
  let mut chars = contents.chars().peekable();

  let mut triangles = Vec::new();

  consume_solid(&mut chars);
  consume_whitespace(&mut chars);
  let title = consume_title(&mut chars);
  consume_whitespace(&mut chars);
  loop {
    if peek_endsolid(&mut chars) {
      break;
    }
    let triangle = consume_triangle(&mut chars);
    triangles.push(triangle);
  }

  println!("title is {}", title);
  println!("num triangles {}", triangles.len());
  println!("next char {:?}", chars.peek());

  Solid::new(triangles, title)
}

fn consume_solid(chars: &mut std::iter::Peekable<std::str::Chars>) {
  for s in "solid".chars() {
    match chars.next() {
      Some(x) if x == s => continue,
      _ => panic!("ended in solid")
    };
  }
}
// todo: wrap each function in a whitespace trimmer
fn consume_triangle(chars: &mut std::iter::Peekable<std::str::Chars>) -> datatypes::Triangle {
  consume_whitespace(chars);
  consume_facet(chars);
  consume_whitespace(chars);
  consume_normal_vector(chars);
  consume_whitespace(chars);
  consume_outer(chars);
  consume_whitespace(chars);
  consume_loop(chars);
  consume_whitespace(chars);
  let a = consume_vertex_vector(chars);
  consume_whitespace(chars);
  let b = consume_vertex_vector(chars);
  consume_whitespace(chars);
  let c = consume_vertex_vector(chars);
  consume_whitespace(chars);
  consume_endloop(chars);
  consume_whitespace(chars);
  consume_endfacet(chars);
  consume_whitespace(chars);

  datatypes::Triangle(a, b, c)
}

// todo: let's have these literals be generated as closures
// need to learn about lifetimes for that
fn consume_facet(chars: &mut std::iter::Peekable<std::str::Chars>) {
  for s in "facet".chars() {
    match chars.next() {
      Some(x) if x == s => continue,
      _ => panic!("ended in facet")
    };
  }
}

fn consume_normal(chars: &mut std::iter::Peekable<std::str::Chars>) {
  for s in "normal".chars() {
    match chars.next() {
      Some(x) if x == s => continue,
      _ => panic!("ended in normal")
    };
  }
}

fn consume_outer(chars: &mut std::iter::Peekable<std::str::Chars>) {
  for s in "outer".chars() {
    match chars.next() {
      Some(x) if x == s => continue,
      _ => panic!("ended in outer")
    };
  }
}

fn consume_loop(chars: &mut std::iter::Peekable<std::str::Chars>) {
  for s in "loop".chars() {
    match chars.next() {
      Some(x) if x == s => continue,
      _ => panic!("ended in loop")
    };
  }
}

fn consume_vertex(chars: &mut std::iter::Peekable<std::str::Chars>) {
  for s in "vertex".chars() {
    match chars.next() {
      Some(x) if x == s => continue,
      _ => panic!("ended in vertex")
    };
  }
}

fn consume_endloop(chars: &mut std::iter::Peekable<std::str::Chars>) {
  for s in "endloop".chars() {
    match chars.next() {
      Some(x) if x == s => continue,
      _ => panic!("ended in endloop")
    };
  }
}

fn consume_endfacet(chars: &mut std::iter::Peekable<std::str::Chars>) {
  for s in "endfacet".chars() {
    match chars.next() {
      Some(x) if x == s => continue,
      _ => panic!("ended in endfacet")
    };
  }
}

fn consume_normal_vector(chars: &mut std::iter::Peekable<std::str::Chars>) -> datatypes::Vertex {
  consume_normal(chars);
  consume_whitespace(chars);
  let x = consume_number(chars);
  consume_whitespace(chars);
  let y = consume_number(chars);
  consume_whitespace(chars);
  let z = consume_number(chars);
  datatypes::Vertex {
    x, y, z,
  }
}

fn consume_vertex_vector(chars: &mut std::iter::Peekable<std::str::Chars>) -> datatypes::Vertex {
  consume_vertex(chars);
  consume_whitespace(chars);
  let x = consume_number(chars);
  consume_whitespace(chars);
  let y = consume_number(chars);
  consume_whitespace(chars);
  let z = consume_number(chars);
  datatypes::Vertex {
    x, y, z,
  }
}

fn consume_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> f32 {
  let mut number = String::from("");
  let number = loop {
    match chars.peek() {
      Some(x) if x.is_whitespace() => break number,
      Some(x) => number.push(*x),
      None => break number
    };
    chars.next();
  };

  number.parse().expect("expecting an e mantissa number!")
}

fn consume_whitespace(chars: &mut std::iter::Peekable<std::str::Chars>) {
  loop {
    match chars.peek() {
      Some(x) if x.is_whitespace() => chars.next(),
      _ => break
    };
  }
}

fn consume_title(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
  let mut title = String::from("");
  loop {
    match chars.peek() {
      Some(x) if x.is_whitespace() => return title,
      Some(x) => title.push(*x),
      None => return title
    };
    chars.next();
  }
}

fn peek_endsolid(chars: &mut std::iter::Peekable<std::str::Chars>) -> bool {
  match chars.peek() {
    Some(x) if *x == 'e' => {
      println!("ending!");
      true
    },
    _ => false,
  }
}