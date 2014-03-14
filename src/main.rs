#[allow(dead_code)];

use std::vec;
use parse::parseFile;
use solve::bruteForce;
use pentomino::Pentomino;

mod solve;
mod parse;
mod pentomino;

/// Finds the Board in a vector of pentominoes, and removes it
/// and returns it.
#[inline]
pub fn discoverBoard(pentominoes: &mut ~[Pentomino]) -> Pentomino {
  let mut index = 0;
  let mut max = 0;
  
  for (i, pentomino) in pentominoes.iter().enumerate() {
    if (pentomino.area() > max) { 
      max = pentomino.area();
      index = i; 
    }
  }

  pentominoes.remove(index)
}

/// Generates permutations of all Pentominoes, 
/// destroying the initial vector
#[inline]
fn generatePermutations(pentominoes: &mut ~[Pentomino], 
                        doRotations: bool,
                        doReflections: bool) -> ~[~[Pentomino]] {
  let mut permutations= vec::with_capacity(pentominoes.len());;

  // Compute all permutations of each pentomino
  loop {
    match pentominoes.pop_opt() {
      Some(p) => {
        let mut reprs = vec::with_capacity(8);

        if (doReflections) {
          reprs.push(p.reflectX());
        }

        reprs.push(p);

        if (doRotations) {
          for i in range(0, reprs.len()) {
            let mut tmp = i;

            for _ in range(0, 3) {
              let r = reprs[tmp].rotateRight();
              reprs.push(r);
              tmp = reprs.len() - 1;
            }
          }
        }

        permutations.push(reprs);
      }
      None => break
    }
  }

  permutations 
}

fn main() {
  let path = Path::new("test/pentominoes6x10.txt");
  //let path = Path::new("test/trivial.txt");
  //let path = Path::new("test/pentominoes3x20.txt");
  let mut pentominoes = parseFile(&path);
  let mut board = discoverBoard(&mut pentominoes);
  let mut permutations = generatePermutations(&mut pentominoes, true, false);

  println(bruteForce(&mut board, &mut permutations, 0).to_str());
}
