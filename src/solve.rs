use std::uint;
use std::bool;
use std::vec::Vec;
use std::fmt::{Show, Formatter, Result};
use pentomino::Pentomino;
use collections::dlist::DList;
use collections::deque::Deque;


/// Placements are represented in two ways.
///
/// First, as an array of integer values of
/// the filled columns.
/// Second, as an array of booleans representing
/// which pieces on the board are filled 
/// by the piece.
pub struct Placement {
  filled: Vec<uint>,
  inner: Vec<bool>
}


impl Placement {
  fn new(filled: Vec<uint>, inner: Vec<bool>) -> Placement {
    Placement { filled: filled, inner: inner }
  }
  pub fn filled<'a>(&'a self) -> &'a Vec<uint> { &self.filled }
  pub fn inner<'a>(&'a self) -> &'a Vec<bool> { &self.inner }
}


impl Eq for Placement {
  fn eq(&self, other: &Placement) -> bool {
    self.filled == other.filled
  }
}


impl Show for Placement {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let mut buf = ~"";

    for b in self.inner.iter() {
      buf.push_str(bool::to_bit::<uint>(*b).to_str() + ", ");
    }

    write!(f.buf, "{:s}", buf) 
  }
}


/// Checks if a placement exists in a given array of Placements 
fn permutationExists(p: &Placement, all: &Vec<Placement>) -> bool {
  let mut present = false;

  for o in all.iter() { if p == o { present = true; } }

  present
}


type MatrixColumn = (bool, uint);


trait Column {
  fn len(&self) -> uint;
  fn status(&self) -> bool;
  fn incr(&mut self);
  fn decr(&mut self);
  fn toggle(&mut self, b: bool);
}


impl Column for MatrixColumn {
  fn status(&self) -> bool {
    self.val0()
  }
  fn len(&self) -> uint {
    self.val1()
  }
  fn incr(&mut self) {
    let len = self.mut1();
    *len = *len + 1;
  }
  fn decr(&mut self) {
    let len = self.mut1();
    *len = *len - 1; 
  }
  fn toggle(&mut self, b: bool) {
    let status = self.mut0();
    *status = b; 
  }
}


/// Finds all placements of all pieces (does not add equivalent 
/// placements) in all positions on the board.
pub fn generatePlacements(board: &Pentomino, 
                          pentominoes: &Vec<Pentomino>,
                          useRotations: bool,
                          useReflections: bool) -> (Vec<MatrixColumn>, Vec<Placement>) {
  let offset = pentominoes.len();
  let cols = board.area() + offset; 
  let mut placements = Vec::new();
  let mut columns = Vec::from_elem(cols, (true, 0 as uint));

  for (i, piece) in pentominoes.iter().enumerate() {
    let mut count: uint = 0;
    let mut permutations = Vec::with_capacity(8);

    // Add rotations
    if useRotations {
      for rotation in piece.rotations() { permutations.push(rotation); }
    } else {
      permutations.push(piece.clone());
    }

    // Add reflections
    if useReflections {
      let mut reflections = Vec::new();
      for piece in permutations.iter() { reflections.push(piece.reflectX()); }
      permutations.push_all_move(reflections);
    }

    // Generate all placements for each piece
    for (x, y, _) in board.coordinates() {
      for permutation in permutations.iter() {
        if board.canPlace(permutation, x, y) {
          let mut filled = Vec::with_capacity(permutation.size() + 1);;
          let mut inner = Vec::from_elem(cols, false);
          
          *inner.get_mut(i) = true;
          filled.push(i);

          for (x0, y0, _) in permutation.filled() {
            let j = board.getIndex(x + x0, y + y0) + offset;
            *inner.get_mut(j) = true;
            filled.push(j);
          }

          let newPlacement = Placement::new(filled, inner);

          if !permutationExists(&newPlacement, &placements) {
            for i in newPlacement.filled.iter() {
              columns.get_mut(*i).incr();
            }
            placements.push(newPlacement);
            count += 1;
          }
        }
      }
    }

    debug!("--");
    debug!("{:s}", piece.to_str());
    debug!("{:u} placements", count);
  }

  for c in columns.mut_iter() {
    if c.len() == 0 { c.toggle(false); }
  }

  (columns, placements) 
}


pub fn solve(placements: &mut Vec<Placement>, columns: &mut Vec<MatrixColumn>,
             rows: &mut Vec<bool>, solutions: &mut uint, depth: uint, 
             current: &mut Vec<uint>, maxSolutions: uint, 
             success: &|&Vec<uint>| -> ()) {
  if *solutions == maxSolutions && maxSolutions != 0 { return }

  let mut min = uint::MAX;

  for (i, c) in columns.iter().enumerate() {
    if c.status() {
      if min == uint::MAX { min = i }
      if c.len() == 0 { return }
      if c.len() < columns.get(min).len() { min = i; }
    }
  }

  if min == uint::MAX {
    *solutions += 1;
    (*success)(current);
    return;
  }

  columns.get_mut(min).toggle(false);

  for row in range(0, rows.len()) {
    if *rows.get(row) && *placements.get(row).inner().get(min) {
      let mut toggledRows: DList<uint> = DList::new();
      let mut toggledCols: DList<uint> = DList::new();

      for c in placements.get(row).filled().iter() {
        columns.get_mut(*c).toggle(false);

        toggledCols.push_back(*c);

        for row0 in range(0, rows.len()) {
          if *rows.get(row0) && *placements.get(row0).inner().get(*c) {
            *rows.get_mut(row0) = false;
            for col0 in placements.get(row0).filled().iter() { 
              columns.get_mut(*col0).decr(); 
            }
            toggledRows.push_back(row0);
          }
        }
      }

      current.push(row);

      solve(placements, columns, rows, solutions, depth + 1, 
            current, maxSolutions, success);

      current.pop();

      for row0 in toggledRows.iter() { 
        *rows.get_mut(*row0) = true; 
        for col0 in placements.get(*row0).filled().iter() { 
          columns.get_mut(*col0).incr(); 
        }
      }
      for col in toggledCols.iter() { columns.get_mut(*col).toggle(true); }
    }
  }

  columns.get_mut(min).toggle(true);
}

fn printMatrix(placements: &Vec<Placement>, columns: &Vec<MatrixColumn>,
               filled: &Vec<bool>) {
  for (i, p) in placements.iter().enumerate() {
    if *filled.get(i) {
      for (j, b) in p.inner.iter().enumerate() { 
        if columns.get(j).status() { print!("{:u}, ", bool::to_bit::<uint>(*b)) }
      }
      println!("")
    }
  }
}
