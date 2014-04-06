use rand::random;
use std::vec::Vec;
use std::char::is_alphabetic;
use solve::Placement;
use pentomino::{Pentomino, System};


/// Generates `n` random numbers
fn generateNRandomAscii(n: uint, used: &mut Vec<char>) {
  if n == 0 { return }

  let x = random::<char>().to_uppercase();
  let mut present = false;

  for y in used.iter() { 
    if x == *y { present = true; break }
  }

  if present || !is_alphabetic(x) || !x.is_ascii() {
    generateNRandomAscii(n, used);
  } else {
    used.push(x);
    generateNRandomAscii(n - 1, used);
  }
}


/// Takes solutions in the form of a vector of 
/// indices, and converts them each into a 
/// Pentomino
pub fn convertSolutions(board: &Pentomino,
                        solutions: &Vec<Vec<uint>>, 
                        placements: &Vec<Placement>, 
                        offset: uint) -> Vec<Pentomino> {
  let mut boards = Vec::with_capacity(solutions.len());
  let mut symbols = Vec::with_capacity(offset);

  generateNRandomAscii(offset, &mut symbols);

  for s in solutions.iter() {
    let mut system: System = Vec::new();

    for n in s.iter() {
      let row = placements.get(*n); 
      let pieceNum = *row.filled().get(0);

      assert!(pieceNum < offset)

      for row0 in row.filled().slice_from(1).iter() {
        let (x, y) = board.getCoordinates(*row0 - offset);
        system.push((x, y, symbols.get(pieceNum).to_ascii()));
      }
    }

    boards.push(Pentomino::newFromSystem(system));
  }

  boards
}


/// Takes a vector of Pentominos, and 
/// removes any isometric ones
pub fn removeIsometric(boards: &mut Vec<Pentomino>) {
  for i in range(0, boards.len()).rev() {
    match boards.remove(i) {
      Some(b) => {
        let mut present = false;

        for rotation in b.rotations() {
          for reflection in rotation.reflections() {
            for b0 in boards.iter() {
              if *b0 == reflection { present = true; break }
            }
            if present { break }
          }
          if present { break }
        }

        if !present { boards.push(b); } 
      }
      None => break
    }
  }
}
