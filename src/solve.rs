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


pub fn solve(solutionMatrix: &mut DancingMatrix, depth: uint, solutions: &mut uint) {
  if (solutionMatrix.root().right() == 0) {
    *solutions += 1;
    return;
  }

  // Find the column with the minimum number of 1's 
  let mut minCol = 0;
  
  for (col, n) in solutionMatrix.iterHeader() {
    if (minCol == 0) { minCol = col }
    if (n.len() == 0) { return }
    if (n.len() < solutionMatrix.get(0, minCol).len() && col != 0) { minCol = col }
  }

  let mut currentCol;
  let mut currentRow = solutionMatrix.get(0, minCol).down();

  //debug!("minCol=({:u}, {:u})", minCol, solutionMatrix.get(0, minCol).len());

  solutionMatrix.coverCol(minCol);

  while (currentRow != 0) {
    currentCol = solutionMatrix.get(currentRow, minCol).right(); 

    // Cover all columns in the current row
    while (currentCol != minCol) {
      solutionMatrix.coverCol(currentCol);
      
      currentCol = solutionMatrix.get(currentRow, currentCol).right();
    }

    // Recursively solve
    solve(solutionMatrix, depth + 1, solutions);

    currentCol = solutionMatrix.get(currentRow, minCol).left();

    // Uncover all columns in the current row (iterate in reverse)
    while (currentCol != minCol) {
      solutionMatrix.uncoverCol(currentCol);

      currentCol = solutionMatrix.get(currentRow, currentCol).left();
    }

    currentRow = solutionMatrix.get(currentRow, minCol).down();
  }

  solutionMatrix.uncoverCol(minCol);
}


