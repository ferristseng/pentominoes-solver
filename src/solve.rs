use std::hashmap::HashSet;
use pentomino::Pentomino;

/// Brute force method for board placement
/// 
/// Returns number of solutions
pub fn bruteForce(board: &mut Pentomino, permutations: &mut ~[HashSet<Pentomino>], 
                  depth: uint) -> uint {
  if (permutations.len() == 0) { println("Solved!"); return 1 }

  let mut solutions = 0;

  for _ in range(0, permutations.len()) {
    let reprs = permutations.pop();

    for pentomino in reprs.iter() {
      match board.getFirst() {
        Some((x, y)) => { 
          for i in range(0, pentomino.dimX) {
            if (board.tryPlacement(pentomino, x as int - i as int, y as int)) { 
              solutions = solutions + bruteForce(board, permutations, depth + 1);
              board.removePlacement(pentomino, x as int - i as int, y as int);
            }
          }
        },
        None => unreachable!()
      }
    }

    permutations.unshift(reprs); 
  }

  solutions
}
