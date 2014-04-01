use std::vec;
use pentomino::Pentomino;
use dlink::{DancingMatrix, LinkedNode, EmptyNode,
            InnerNode};

pub fn generateSolutionMatrix(board: &Pentomino, 
                              pentominoes: &~[Pentomino]) -> DancingMatrix {
  let offset = pentominoes.len();
  let cols = offset + board.area();

  let mut removed: uint = 0;
  let mut solutionMatrix = DancingMatrix::new(cols);

  for (i, piece) in pentominoes.iter().enumerate() {
    let mut count: uint = 0;

    for (x, y, _) in board.coordinates() {
      for rotation in piece.rotations() {
        for permutation in rotation.reflections() {
          if (board.canPlace(&permutation, x, y)) {
            let num = i + 1;
            let mut row = vec::from_elem(cols + 1, EmptyNode);

            row[0] = InnerNode(Default::default(), 0);
            row[num] = InnerNode(LinkedNode::new(0, 0, num, num), num);

            for (x0, y0, _) in permutation.filled() {
              let j = offset + 1 + board.getIndex(x + x0, y + y0);
              row[j] = InnerNode(LinkedNode::new(0, 0, j, j), j);
            }

            if (solutionMatrix.insert(row)) {
              count += 1;
            }
          }
        }
      }
    }

    debug!("--");
    debug!("{:s}", piece.to_str());
    debug!("{:u} placements", count);
  }

  for i in range(1, solutionMatrix.cols()) {
    if (solutionMatrix.header()[i].len() == 0) {
      solutionMatrix.deleteCol(i);
      removed += 1;
    }
  }

  debug!("Removed {:u} empty columns", removed);

  solutionMatrix
}

