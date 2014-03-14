use std::{vec, str};
use std::num::abs;
use std::cmp::{Greater, Less};

pub enum Square {
  Unmarked(char),
  Marked(char),
  Empty
}

pub type Point = (uint, uint, char);
pub type System = ~[Point];
pub type Piece = ~[Square];

#[deriving(Clone)]
pub struct Pentomino<'a> {
  dimX: uint,
  dimY: uint,
  size: uint,
  piece: Piece
}

impl<'a> Pentomino<'a> {
  /// Create a new Pentomino from a System
  pub fn newFromSystem(system: System) -> Pentomino {
    let mut maxX = 0;
    let mut maxY = 0;
    let mut newSystem = ~[];

    // Generate max / min to normalize points
    for point in system.iter() {
      let (coorX, _, _) = *point;
      if (coorX > maxX) { maxX = coorX }
    }

    let mut minX = maxX;

    for point in system.iter() {
      let (coorX, _, _) = *point;
      if (coorX < minX) { minX = coorX }
    }

    for point in system.iter() {
      let (_, coorY, _) = *point;
      if (coorY > maxY) { maxY = coorY }
    }

    let mut minY = maxY;

    for point in system.iter() {
      let (_, coorY, _) = *point;
      if (coorY < minY) { minY = coorY }
    }

    let dimX = maxX - minX + 1;
    let dimY = maxY - minY + 1;

    // Normalize Points
    for point in system.iter() {
      let (coorX, coorY, c) = *point;
      newSystem.push((coorX - minX, coorY - minY, c));
    }

    Pentomino::new(newSystem, dimX, dimY)
  }
  /// System in Pentomino is sorted by (x, y)
  pub fn new(system: System, dimX: uint, dimY: uint) -> Pentomino {
    let mut system = system;

    system.sort_by(|a, b| {
      let (coorX, coorY, _) = *a;
      let (coorX0, coorY0, _) = *b;

      if (coorY < coorY0) {
        Less
      } else if (coorY == coorY0) {
        coorX.cmp(&coorX0) 
      } else {
        Greater
      }
    });

    Pentomino {
      dimX: dimX,
      dimY: dimY,
      system: system
    }
  }
}

impl<'a> Pentomino<'a> {
  /// Calculate the area of a Pentomino
  pub fn area(&self) -> uint {
    self.dimX * self.dimY
  }
  /// Size of a Pentomino is the number of points in 
  /// the System
  pub fn size(&self) -> uint {
    self.system.len()
  }
  /// Points in the Pentomino
  pub fn points(&'a self) -> &'a System {
    &self.system
  }
  /// Iterator over points in Pentomino
  pub fn iter(&'a self) -> vec::VecIterator<'a, Point> {
    self.points().iter()
  }
  /// First point in the system
  pub fn first(&'a self) -> &'a Point {
    self.points().head()
  }
  /// Get a point in the system
  pub fn get_opt(&'a self, i: uint) -> Option<&'a Point> {
    self.points().get_opt(i)
  }
}

impl<'a> Pentomino<'a> {
  /// Reflect the Pentomino over the X axis
  pub fn reflectX(&mut self) {
    for point in self.system.mut_iter() {
      let (coorX, coorY, c) = *point;
      *point = ((abs(coorX as int - self.dimX as int) - 1) as uint, coorY, c);
    }
  }
  /// Reflect the Pentomino over the Y axis
  pub fn reflectY(&mut self) {
    for point in self.system.mut_iter() {
      let (coorX, coorY, c) = *point;
      *point = (coorX, (abs(coorY as int - self.dimY as int) - 1) as uint, c);
    }
  }
  /// Rotates the current Pentomino 90 degrees
  /// to the left
  pub fn rotateLeft(&mut self) {
    let tmp = self.dimX;
    self.dimX = self.dimY;
    self.dimY = tmp;

    for point in self.system.mut_iter() {
      let (coorX, coorY, c) = *point;
      let coorX0 = coorX;
      *point = (coorY, self.dimY - coorX0 - 1, c);
    }
  }
  /// Rotates the current Pentomino 90
  /// degrees to the right
  pub fn rotateRight(&mut self) {
    let tmp = self.dimX;
    self.dimX = self.dimY;
    self.dimY = tmp;

    for point in self.system.mut_iter() {
      let (coorX, coorY, c) = *point;
      let coorX0 = coorX;
      *point = (self.dimX - coorY - 1, coorX0, c);
    }
  }
  /// Attempts to overlay a Pentomino
  /// on the current Pentomino at a specified 
  /// offset
  pub fn tryPlacement(&mut self, piece: &Pentomino,
                      offsetX: int, offsetY: int) -> bool {
    let mut indices = ~[];

    for point in piece.iter() {
      let (coorX, coorY, c) = *point;

      for (i, point0) in self.iter().enumerate() {
        let (coorX0, coorY0, c0) = *point0;

        if (coorX0 as int == coorX as int + offsetX &&
            coorY0 as int == coorY as int + offsetY &&
            c == c0) {
          indices.push(i);
          break;
        }
      }
    }

    if (indices.len() == piece.size()) {
      indices.sort_by(|a, b| b.cmp(a));
      for i in indices.iter() { self.system.remove(*i); }
      true
    } else {
      false
    }
  }
}


impl<'a> ToStr for Pentomino<'a> {
  fn to_str(&self) -> ~str {
    let mut buf = vec::from_elem((self.dimX + 1) * self.dimY, ' ');

    for point in self.system.iter() {
      let (coorX, coorY, c) = *point;
      buf[(self.dimX + 1) * coorY + coorX] = c;
    }

    for i in range(1, self.dimY) {
      buf[self.dimX * i + (i - 1)] = '\n';
    }

    str::from_chars(buf) 
  }
}
