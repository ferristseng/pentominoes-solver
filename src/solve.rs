use pentomino::Pentomino;

/// Brute force method for board placement
/// 
/// Returns number of solutions
pub fn bruteForce(board: &Pentomino, pieces: &mut ~[Pentomino]) -> uint {
  if (pieces.len() == 0 || board.size() == 0) { return 1 }

  let mut solutions = 0;
  let doTest = |board: &Pentomino, piece: &Pentomino, 
                offsetX: int, offsetY: int| -> uint {
    let mut boardCopy = board.clone();

    if (boardCopy.tryPlacement(piece, offsetX, offsetY)) {
      return bruteForce(&boardCopy, pieces)
    }

    0 
  };

  for _ in range(0, pieces.len()) {
    match pieces.pop_opt() {
      Some(piece) => {
        let mut piece = piece;
        let (offsetX, offsetY, _) = *board.first();

        // Try all possible rotations
        // and permutations of rotations
        for _ in range(0, 4) {
          for i in range(0, piece.dimX) {
            solutions = solutions + 
              doTest(board, &piece, offsetX as int - i as int, offsetY as int); 
          }

          piece.rotateRight();
        }

        pieces.unshift(piece);
      }
      None => unreachable!()
    }
  }

  solutions
}
