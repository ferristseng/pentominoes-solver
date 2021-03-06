use std::str;
use std::iter;
use std::num::abs;
use std::slice::Items;
use std::vec::{Vec, MoveItems};
use std::fmt::{Show, Formatter, Result};



pub type Point = (uint, uint, Ascii);
pub type System = Vec<Point>;


/// Possible representations of a Square (a 
/// part of an entire piece) in a Pentomino
#[deriving(Clone, Eq)]
pub enum Square {
  Filled(Ascii),
  Empty
}

impl AsciiCast<Ascii> for Square {
  unsafe fn to_ascii_nocheck(&self) -> Ascii {
    match *self {
      Filled(c) => c,
      Empty => ' '.to_ascii()
    }
  }
  fn is_ascii(&self) -> bool { true }
}

impl Show for Square {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f.buf, "{:s}", self.to_ascii().to_str())
  }
}


/// A Pentomino
///
///   * `dimX` - x dimension of the piece
///   * `dimY` - y dimension of the piece
///   * `size` - number of none empty / marked Squares in 
///              the piece
///   * `squares` - underlying structure of the Pentomino
#[deriving(Clone)]
pub struct Pentomino<'a> {
  dimX: uint,
  dimY: uint,
  size: uint,
  squares: Vec<Square>
}


impl<'a> Pentomino<'a> {
  /// Create a new Pentomino from a System
  pub fn newFromSystem(system: System) -> Pentomino {
    let mut maxX = 0;
    let mut maxY = 0;
    let mut system = system;

    // Generate max / min to normalize points
    for point in system.iter() {
      let (coorX, _, _) = *point;
      if coorX > maxX { maxX = coorX }
    }

    let mut minX = maxX;

    for point in system.iter() {
      let (coorX, _, _) = *point;
      if coorX < minX { minX = coorX }
    }

    for point in system.iter() {
      let (_, coorY, _) = *point;
      if coorY > maxY { maxY = coorY }
    }

    let mut minY = maxY;

    for point in system.iter() {
      let (_, coorY, _) = *point;
      if coorY < minY { minY = coorY }
    }

    let dimX = maxX - minX + 1;
    let dimY = maxY - minY + 1;

    // Normalize Points
    for point in system.mut_iter() {
      let (coorX, coorY, c) = *point;
      *point = (coorX - minX, coorY - minY, c);
    }

    let mut squares = Vec::from_elem(dimX * dimY, Empty);

    for point in system.iter() {
      let (coorX, coorY, c) = *point;
      *squares.get_mut(coorY * dimX + coorX) = Filled(c);
    }

    Pentomino {
      dimX: dimX,
      dimY: dimY,
      size: system.len(),
      squares: squares
    }
  }
}


impl<'a> Pentomino<'a> {
  /// Calculate the area of a Pentomino
  pub fn area(&self) -> uint {
    self.dimX * self.dimY
  }
  /// Size of a Pentomino is the number of Unmarked 
  /// squares it has
  pub fn size(&self) -> uint {
    self.size
  }
  /// Squares in the Pentomino
  pub fn squares(&'a self) -> &'a Vec<Square> {
    &self.squares
  }
  /// Get a square at coordinate (x, y) in the Pentomino
  pub fn get_opt(&'a self, x: uint, y: uint) -> Option<&'a Square> {
    if x < self.dimX && y < self.dimY {
      Some(self.squares().get(self.getIndex(x, y)))
    } else {
      None
    }
  }
  /// Get a square at coordinate (x, y) in the Pentomino
  pub fn get(&'a self, x: uint, y: uint) -> &'a Square {
    self.squares().get(self.getIndex(x, y))
  }
}


impl<'a> Pentomino<'a> {
  /// Iterator over squares in Pentomino
  pub fn iter(&'a self) -> Items<'a, Square> { 
    self.squares().iter()
  }
  /// Range to the area
  pub fn range(&'a self) -> iter::Range<uint> {
    range(0, self.area())
  }
  /// Returns an iterator over all coordinates
  pub fn coordinates(&self) -> MoveItems<Point> {
    Vec::from_fn(self.area(), |i| { 
      let (x, y) = self.getCoordinates(i);

      match self.get_opt(x, y) {
        Some(&Filled(sq)) => (x, y, sq), 

        _ => (x, y, ' '.to_ascii())
      }
    }).move_iter()
  }
  /// Returns an iterator over all non-empty coordinates
  pub fn filled(&self) -> MoveItems<Point> { 
    let mut coords = Vec::with_capacity(self.size());

    for i in self.range() {
      let (x, y) = self.getCoordinates(i);

      match self.get_opt(x, y) {
        Some(&Filled(sq)) => coords.push((x, y, sq)),
        _ => () 
      }
    }

    coords.move_iter()
  }
  /// Returns an iterator across all rotated varients of
  /// the Pentomino
  pub fn rotations(&self) -> MoveItems<Pentomino> {
    let mut rotations = Vec::with_capacity(4);

    rotations.push(self.rotateRight());

    for _ in range(0, 4) {
      let new = match rotations.last() {
        Some(piece) => piece.rotateRight(),
        None => fail!("expected a piece to rotate")
      };
      rotations.push(new);
    }

    rotations.move_iter()
  }
  /// Returns an iterator across all reflected varients of the
  /// Pentomino
  pub fn reflections(&self) -> MoveItems<Pentomino> {
    let mut reflections = Vec::with_capacity(2);

    reflections.push(self.clone());
    reflections.push(self.reflectX());

    reflections.move_iter()
  }
}


impl<'a> Pentomino<'a> {
  /// Gets coordinates that are represented 
  /// by an index
  pub fn getCoordinates(&self, i: uint) -> (uint, uint) {
    let y = i / self.dimX;
    (i - (y * self.dimX), y)
  }
  /// Gets an index from coordinates
  pub fn getIndex(&self, x: uint, y: uint) -> uint {
    y * self.dimX + x
  }
}


impl<'a> Pentomino<'a> {
  /// Generalized transform 
  /// O(n), n = number of squares 
  fn doTransformation(&self, dimX: uint, dimY: uint,
                      fun: |x: uint, y: uint| -> uint) -> Pentomino {
    let mut squares = Vec::from_elem(self.dimX * self.dimY, Empty);

    for i in self.range() {
      let (coorX, coorY) = self.getCoordinates(i);
      *squares.get_mut(fun(coorX, coorY)) = *self.squares.get(i);
    }

    Pentomino {
      dimX: dimX,
      dimY: dimY, 
      size: self.size,
      squares: squares
    }
  }
  /// Returns a new Pentomino which is the
  /// current one reflected over the X axis
  pub fn reflectX(&self) -> Pentomino {
    self.doTransformation(self.dimX, self.dimY, 
      |x, y| { (self.dimX * y) + (abs(x as int - self.dimX as int) - 1) as uint })
  }
  /// Returns a new Pentomino which is the
  /// current one reflected over the Y axis
  pub fn reflectY(&self) -> Pentomino {
    self.doTransformation(self.dimX, self.dimY,
      |x, y| { ((abs(y as int - self.dimY as int) - 1) as uint * self.dimX + x) })
  }
  /// Returns a new Pentomino which is the 
  /// current one rotated left 90 degrees
  pub fn rotateLeft(&self) -> Pentomino {
    self.doTransformation(self.dimY, self.dimX, 
      |x, y| { (self.dimX - x - 1) * self.dimY + y })
  }
  /// Returns a new Pentomino which is the 
  /// current one rotated right 90 degrees
  pub fn rotateRight(&self) -> Pentomino {
    self.doTransformation(self.dimY, self.dimX,
      |x, y| { (x * self.dimY) + self.dimY - y - 1 })
  }
}


impl<'a> Pentomino<'a> {
  /// Returns whether or not a piece can be placed on 
  /// the piece
  pub fn canPlace(&self, p: &Pentomino, offsetX: uint, 
                  offsetY: uint) -> bool {
    let mut placements: uint = 0;

    for (x, y, c) in p.filled() {
      match self.get_opt(x + offsetX, y + offsetY) {
        Some(sq) => if sq.to_ascii() == c { placements += 1 },
        None => ()
      }
    }

    placements == p.size()
  }
}


impl<'a> Eq for Pentomino<'a> {
  fn eq(&self, other: &Pentomino) -> bool {
    if self.size() != other.size() ||
       self.dimX != other.dimX ||
       self.dimY != other.dimY { 
      return false 
    }

    let mut equalEl = 0;

    for i in self.range() { 
      if *other.squares().get(i) == *self.squares().get(i) {
        equalEl += 1
      } else {
        return false
      }
    }

    self.area() == equalEl
  }
}


impl<'a> Show for Pentomino<'a> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let mut buf = Vec::from_elem((self.dimX + 1) * self.dimY, ' ');

    for y in range(0, self.dimY) {
      for x in range(0, self.dimX) {
        *buf.get_mut(y * (self.dimX + 1) + x) = self.get(x, y).to_ascii().to_char();
      }
    }

    for i in range(1, self.dimY) {
      *buf.get_mut(self.dimX * i + (i - 1)) = '\n';
    }

    write!(f.buf, "{:s}", str::from_chars(buf.as_slice()))
  }
}
