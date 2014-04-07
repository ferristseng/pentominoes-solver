use std::vec::Vec;
use std::num::abs;
use std::io::File;
use pentomino::{Pentomino, System};


/// Discovers all adjacent points
/// in a System from a given coordinate
#[inline]
fn discoverPoints(startX: uint, startY: uint, system: &mut System, 
                  newSystem: &mut System) {
  for _ in range(0, system.len()) {
    match system.pop() {
      Some(point) => {
        let (coorX, coorY, _) = point;

        if abs(coorX as int - startX as int) <= 1 &&
           abs(coorY as int - startY as int) <= 1 {
          newSystem.push(point);
          discoverPoints(coorX, coorY, system, newSystem);
        } else {
          system.unshift(point);
        }
      }
      None => break
    }
  }
}


/// Takes in a path, and parses
/// a file at the path, finding all 
/// valid Pentominoes in the file.
pub fn parseFile(path: &Path) -> Vec<Pentomino> {
  assert!(path.exists())

  let mut file = File::open(path);

  let mut x = 0;
  let mut y = 0;
  let mut points: System = Vec::new();
  let mut pentominoes: Vec<Pentomino> = Vec::new();

  loop {
    match file.read_byte() {
      Ok(b) => {
        match b as char {
          '\n' => {
            x = 0;
            y = y + 1;
          }
          c => {
            if c != ' ' {
              points.push((x, y, c.to_ascii()));
            }

            x = x + 1;
          }
        }
      }
      // EOF
      Err(_) => break 
    }
  }

  loop {
    match points.pop() {
      Some(point) => {
        let (coorX, coorY, _) = point;
        let mut pentomino = Vec::from_elem(1, point);
        discoverPoints(coorX, coorY, &mut points, &mut pentomino);
        pentominoes.push(Pentomino::newFromSystem(pentomino));
      }
      None => break
    }
  }

  pentominoes
}
