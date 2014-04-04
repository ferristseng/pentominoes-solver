use std::uint;
use std::vec::Vec;
use collections::dlist::DList;
use pentomino::Pentomino;


/// Placements are represented in two ways.
///
/// First, as an array of integer values of
/// the filled columns.
/// Second, as an array of booleans representing
/// which pieces on the board are filled 
/// by the piece.
#[deriving(Show)]
struct Placement {
  filled: Vec<uint>,
  inner: Vec<bool>,
  pieceNum: uint
}


impl Placement {
  fn new(filled: Vec<uint>, inner: Vec<bool>, n: uint) -> Placement {
    Placement { filled: filled, inner: inner, pieceNum: n }
  }
}


impl Eq for Placement {
  fn eq(&self, other: &Placement) -> bool {
    self.filled == other.filled
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
  let mut placements = Vec::new();
  let mut columns = Vec::from_elem(board.area(), (true, 0 as uint));

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
          let mut filled = Vec::with_capacity(permutation.size());;
          let mut inner = Vec::from_elem(board.area(), false);

          for (x0, y0, _) in permutation.filled() {
            let j = board.getIndex(x + x0, y + y0);
            *inner.get_mut(j) = true;
            filled.push(j);
          }

          let newPlacement = Placement::new(filled, inner, i);

          if !permutationExists(&newPlacement, &placements) {
            for (i, b) in newPlacement.inner.iter().enumerate() {
              if *b { columns.get_mut(i).incr() }
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

  (columns, placements) 
}


pub fn solve(placements: &mut Vec<Placement>, columns: &mut Vec<MatrixColumn>,
             rows: &mut Vec<bool>, solutions: &mut uint, depth: uint, 
             maxDepth: uint) {
  if depth == maxDepth { 
    println!("Solution Found");
    *solutions += 1;
    return;
  }
  
  let mut min = uint::MAX;

  for (i, c) in columns.iter().enumerate() {
    if c.status() {
      if min == uint::MAX { min = i }
      if c.len() == 0 { return }
      if c.len() > columns.get(min).len() { min = i; }
    }
  }

  columns.get_mut(min).toggle(false);

  // for row in column
  // toggle the row in rows
  // delete each column in the row
  //  and each row in those columns
  // recurse
  // restore the rows and columns
  for row in range(0, rows.len()) {
    if *rows.get(row) && *placements.get(row).inner.get(min) {
      let mut toggledRows: DList<uint> = DList::new();
      let mut toggledCols: DList<uint> = DList::new();

      for c in placements.get(row).filled.iter() {
        columns.get_mut(*c).toggle(false);

        for row0 in range(0, rows.len()) {
          if *rows.get(row0) && *placements.get(row0).inner.get(*c) {
            *rows.get_mut(row0) = false;
          }
        }
      }

      solve(placements, columns, rows, solutions, depth + 1, maxDepth);
    }
  }

  columns.get_mut(min).toggle(true);
}
