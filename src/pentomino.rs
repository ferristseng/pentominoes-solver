use std::{vec, str};
use std::num::abs;


pub type Point = (uint, uint, Ascii);
pub type System = ~[Point];


/// Possible representations of a Square (a 
/// part of an entire piece) in a Pentomino
#[deriving(Clone)]
pub enum Square {
  Unmarked(Ascii),
  Marked(Ascii),
  Empty
}

impl Square {
  /// Unmarked -> Marked
  fn mark(&mut self) {
    match *self {
      Unmarked(c) => *self = Marked(c),
      _ => ()
    }
  }
  /// Marked -> Unmarked
  fn unmark(&mut self) {
    match *self {
      Marked(c) => *self = Unmarked(c),
      _ => ()
    }
  }
  /// Checks if unmarked
  fn isUnmarked(&self) -> bool {
    match *self { Unmarked(_) => true, _ => false }
  }
  /// Checks if marked
  fn isMarked(&self) -> bool {
    !self.isUnmarked()
  }
}

impl Eq for Square {
  fn eq(&self, other: &Square) -> bool {
    match *self {
      Unmarked(c) => match *other { Unmarked(c1) => c == c1, _ => false },
      _ => false
    }
  }
}

impl AsciiCast<Ascii> for Square {
  unsafe fn to_ascii_nocheck(&self) -> Ascii {
    match *self {
      Unmarked(c) => c,
      Marked(_) | 
      Empty => ' '.to_ascii()
    }
  }
  fn is_ascii(&self) -> bool { true }
}

impl ToStr for Square {
  fn to_str(&self) -> ~str {
    self.to_ascii().to_str()
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
  priv size: uint,
  priv squares: ~[Square]
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
    for point in system.mut_iter() {
      let (coorX, coorY, c) = *point;
      *point = (coorX - minX, coorY - minY, c);
    }

    let mut squares = vec::from_elem(dimX * dimY, Empty);

    for point in system.iter() {
      let (coorX, coorY, c) = *point;
      squares[coorY * dimX + coorX] = Unmarked(c);
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
  pub fn squares(&'a self) -> &'a ~[Square] {
    &self.squares
  }
  /// Iterator over squares in Pentomino
  pub fn iter(&'a self) -> vec::VecIterator<'a, Square> {
    self.squares().iter()
  }
  /// Mutable Iterator over squares in Pentomino
  pub fn mut_iter(&'a mut self) -> vec::VecMutIterator<'a, Square> {
    self.squares.mut_iter()
  }
  /// Get a square at coordinate (x, y) in the Pentomino
  pub fn get_opt(&'a self, x: uint, y: uint) -> Option<&'a Square> {
    if (x < self.dimX && y < self.dimX) {
      self.squares().get_opt(self.getIndex(x, y))
    } else {
      None
    }
  }
  /// Get a square at coordinate (x, y) in the Pentomino
  pub fn get(&'a self, x: uint, y: uint) -> &'a Square {
    &self.squares()[self.getIndex(x, y)]
  }
}


impl<'a> Pentomino<'a> {
  /// Gets the coordinates of the first 
  /// open (Unmarked) square
  pub fn getFirst(&self) -> Option<(uint, uint)> {
    for i in range(0, self.area()) { 
      match self.squares[i] {
        Unmarked(_) => return Some(self.getCoordinates(i)),
        _ => () 
      }
    }
    
    None
  }
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


impl <'a> Pentomino<'a> {
  /// Generalized transform 
  /// O(n), n = number of squares 
  fn doTransformation(&self, dimX: uint, dimY: uint,
                      fun: |x: uint, y: uint| -> uint) -> Pentomino {
    let mut squares = vec::from_elem(self.dimX * self.dimY, Empty);

    for i in range(0, self.area()) {
      let (coorX, coorY) = self.getCoordinates(i);
      squares[fun(coorX, coorY)] = self.squares[i];
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
  /// Generalized Placement
  /// O(n)
  fn generalizedPlacement(&mut self, piece: &Pentomino,
                          offsetX: int, offsetY: int, 
                          fun: |square: &mut Square|) {
    for i in range(0, piece.area()) {
      let (coorX, coorY) = piece.getCoordinates(i);
      let (shiftedX, shiftedY) = ((coorX as int + offsetX) as uint,
        (coorY as int + offsetY) as uint);

      if (shiftedX < self.dimX && shiftedY < self.dimY && 
          piece.get(coorX, coorY).isUnmarked()) {
        fun(&mut self.squares[self.getIndex(shiftedX, shiftedY)]);
      }
    }
  }
  /// Makes a placement, marking all pieces in the Pentomino
  /// that match with the inputted piece
  pub fn doPlacement(&mut self, piece: &Pentomino,
                 offsetX: int, offsetY: int) {
    self.generalizedPlacement(piece, offsetX, offsetY, 
      |square| { square.mark() });
    self.size = self.size - piece.size()
  }
  /// Removes a placement, unmarked all pieces in the Pentomino
  /// that match with the inputted piece
  pub fn removePlacement(&mut self, piece: &Pentomino,
                     offsetX: int, offsetY: int) {
    self.generalizedPlacement(piece, offsetX, offsetY,
      |square| { square.unmark() });
    self.size = self.size + piece.size()
  }
  /// Attempts to overlay a Pentomino
  /// on the current Pentomino at a specified 
  /// offset
  pub fn tryPlacement(&mut self, piece: &Pentomino,
                      offsetX: int, offsetY: int) -> bool {
    let mut count = 0;

    for i in range(0, piece.area()) {
      let (coorX, coorY) = piece.getCoordinates(i);

      match self.get_opt((coorX as int + offsetX) as uint, 
                         (coorY as int + offsetY) as uint) {
        Some(square) => if (square == piece.get(coorX, coorY)) {
          count = count + 1
        },
        None => () 
      }
    }

    if (count == piece.size()) {
      self.doPlacement(piece, offsetX, offsetY);
      true
    } else {
      false
    }
  }
}


impl<'a> ToStr for Pentomino<'a> {
  fn to_str(&self) -> ~str {
    let mut buf = vec::from_elem((self.dimX + 1) * self.dimY, ' ');

    for y in range(0, self.dimY) {
      for x in range(0, self.dimX) {
        buf[y * (self.dimX + 1) + x] = self.get(x, y).to_ascii().to_char();
      }
    }

    for i in range(1, self.dimY) {
      buf[self.dimX * i + (i - 1)] = '\n';
    }

    str::from_chars(buf) 
  }
}
