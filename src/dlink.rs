use std::uint;
use std::vec;


#[deriving(Clone)]
pub enum DancingNode {
  Root(LinkedNode),
  InnerNode(LinkedNode, uint),
  ColumnNode(LinkedNode, uint),
  EmptyNode,
}


impl Eq for DancingNode {
  fn eq(&self, other: &DancingNode) -> bool {
    match *self {
      Root(_) => match *other {
        Root(_) => true,
        _ => false
      },
      InnerNode(_, col0) => match *other {
        InnerNode(_, col1) => col0 == col1,
        _ => false
      },
      ColumnNode(_, col0) => match *other {
        ColumnNode(_, col1) => col0 == col1,
        _ => false
      },
      EmptyNode => match *other {
        EmptyNode => true,
        _ => false
      }
    }
  }
}


// Getters
impl DancingNode {
  pub fn right(&self) -> uint {
    match *self {
      Root(n) | InnerNode(n, _) | ColumnNode(n, _) => n.right,
      _ => 0
    }
  }
  pub fn left(&self) -> uint {
    match *self {
      Root(n) | InnerNode(n, _) | ColumnNode(n, _) => n.left,
      _ => 0
    }
  }
  pub fn up(&self) -> uint {
    match *self {
      Root(n) | InnerNode(n, _) | ColumnNode(n, _) => n.up,
      _ => 0
    }
  }
  pub fn down(&self) -> uint {
    match *self {
      Root(n) | InnerNode(n, _) | ColumnNode(n, _) => n.down,
      _ => 0
    }
  }
}


// Setters
impl DancingNode {
  fn setRight(&mut self, i: uint) {
    match *self {
      Root(ref mut n) |
      InnerNode(ref mut n, _) | 
      ColumnNode(ref mut n, _) => {
        n.right = i
      }
      _ => fail!("invalid set!") 
    }
  }
  fn setLeft(&mut self, i: uint) {
    match *self {
      Root(ref mut n) |
      InnerNode(ref mut n, _) | 
      ColumnNode(ref mut n, _) => {
        n.left = i
      }
      _ => fail!("invalid set!") 
    }
  }
  fn setUp(&mut self, i: uint) {
    match *self {
      Root(ref mut n) |
      InnerNode(ref mut n, _) |
      ColumnNode(ref mut n, _) => {
        n.up = i
      }
      _ => fail!("invalid set!") 
    }
  }
  fn setDown(&mut self, i: uint) {
    match *self {
      Root(ref mut n) |
      InnerNode(ref mut n, _) | 
      ColumnNode(ref mut n, _) => {
        n.down = i
      }
      _ => fail!("invalid set!") 
    }
  }
}


impl DancingNode {
  pub fn incr(&mut self) {
    match *self {
      ColumnNode(_, ref mut x) => *x += 1,
      Root(_) => (),
      _ => fail!("incr applied on invalid node!")
    }
  }
}


impl Container for DancingNode {
  fn len(&self) -> uint {
    match *self {
      Root(_) => uint::max_value, 
      InnerNode(_, _) => 0,
      ColumnNode(_, l) => l,
      EmptyNode => 0
    }
  }
}


impl ToStr for DancingNode {
  fn to_str(&self) -> ~str {
    match *self {
      Root(_) => ~"R",
      ColumnNode(_, _) => ~"C",
      InnerNode(_, _) => ~"1",
      EmptyNode => ~"0",
    } 
  }
}


#[deriving(Clone, ToStr)]
pub struct LinkedNode {
  up: uint,
  down: uint,
  left: uint,
  right: uint
}


impl LinkedNode {
  pub fn new(up: uint, down: uint, right: uint, left: uint) -> LinkedNode {
    LinkedNode { up: up, down: down, right: right, left: left }
  }
}


impl Default for LinkedNode {
  fn default() -> LinkedNode {
    LinkedNode { up: 0, down: 0, left: 0, right: 0 }
  }
}


pub struct DancingMatrix {
  cols: uint, 
  inner: ~[~[DancingNode]]
}


// Creation
impl DancingMatrix {
  pub fn new(cols: uint) -> DancingMatrix {
    let mut header = vec::with_capacity(cols + 1);
    
    header.push(Root(Default::default()));

    let mut r = DancingMatrix { cols: cols + 1, inner: ~[header] };

    for i in range(1, cols + 1) {
      r.pushColumn(i);
    }

    r
  }
  /// Returns the first row (aka the header row)
  pub fn header<'a>(&'a self) -> &'a ~[DancingNode] {
    &self.inner[0]
  }
  /// Returns the root node
  pub fn root<'a>(&'a self) -> &'a DancingNode {
    &self.inner[0][0]
  }
  /// Returns the number of columns
  pub fn cols(&self) -> uint {
    self.cols
  }
  /// Returns a node at (x, y)
  pub fn get<'a>(&'a self, x: uint, y: uint) -> &'a DancingNode {
    &self.inner[x][y]
  }
  /// Pushes a column into the header row
  fn pushColumn(&mut self, col: uint) {
    let last = self.inner[0].len() - 1;
    self.inner[0][0].setLeft(col);
    self.inner[0][last].setRight(col);
    self.inner[0].push(ColumnNode(LinkedNode::new(0, 0, 0, last), 0));
  }
  /// Slow O(n) where n is the length of the inner array
  fn exists<'a>(&'a mut self, v: &'a ~[DancingNode]) -> bool {
    let mut present = false;

    for r in self.inner.iter() {
      if (r == v) { present = true }
    }

    present
  }
  /// Inserts a row in if it does not exists, and sets up links
  /// between the nodes
  pub fn insert(&mut self, mut v: ~[DancingNode]) -> bool {
    if (v.len() != self.header().len()) { 
      fail!("inappropriately sized row!")
    }

    if (!self.exists(&v)) {
      let row = self.inner.len();

      for col0 in range(0, v.len()) {
        let last = v[0].left();
        let mut inner = false;

        match v[col0] { 
          // Link each node with the upper tail
          // and right tail
          InnerNode(ref mut n, col) => {
            let prev = self.inner[0][col].up();
            n.up = prev;
            n.down = 0;
            n.right = 0;
            n.left = last;

            self.inner[prev][col].setDown(row);
            self.inner[0][col].setUp(row);

            self.inner[0][col].incr();

            inner = true;
          }
          _ => () 
        }

        if (inner) {
          v[last].setRight(col0);
          v[0].setLeft(col0);
        }
      }
      
      self.inner.push(v);

      true
    } else { false }
  }
}


// Node deletion and undeletion
impl DancingMatrix {
  /// Deletes a col
  fn _deleteCol(&mut self, i: uint) {
    let left = self.header()[i].left();
    let right = self.header()[i].right();

    self.inner[0][left].setRight(right);
    self.inner[0][right].setLeft(left);
  }
  /// Deletes a node
  fn _deleteNode(&mut self, row: uint, col: uint) {
    let up = self.inner[row][col].up();
    let down = self.inner[row][col].down();

    self.inner[up][col].setDown(down);
    self.inner[down][col].setUp(up);
  }
  /// Undelete col
  fn _undeleteCol(&mut self, i: uint) {
    let left = self.header()[i].left();
    let right = self.header()[i].right();

    self.inner[0][left].setRight(i);
    self.inner[0][right].setLeft(i);
  }
  /// Undelete a node
  fn _undeleteNode(&mut self, row: uint, col: uint) {
    let up = self.inner[row][col].up();
    let down = self.inner[row][col].down();

    self.inner[up][col].setDown(row);
    self.inner[down][col].setUp(row);
  }
  /// Deletes a column in the inner matrix
  /// and all rows in that column
  pub fn deleteCol(&mut self, i: uint) {
    if (!(i < self.len())) { fail!("out of index column!") } 

    self._deleteCol(i);

    let mut currentCol;
    let mut currentRow = self.inner[0][i].down();

    while (currentRow != 0) {
      currentCol = self.inner[currentRow][i].right(); 

      while (currentCol != i) {
        self._deleteNode(currentRow, currentCol);

        currentCol = self.inner[currentRow][currentCol].right();
      }

      currentRow = self.inner[currentRow][i].down();
    }
  }
  /// Undeletes a column in the inner matrix
  /// and all rows in that column
  pub fn undeleteCol(&mut self, i: uint) {
    if (!(i < self.len())) { fail!("out of index column!") }

    self._undeleteCol(i);

    let mut currentCol;
    let mut currentRow = self.inner[0][i].up();

    while (currentRow != 0) {
      currentCol = self.inner[currentRow][i].left();

      while (currentCol != i) {
        self._undeleteNode(currentRow, currentCol);

        currentCol = self.inner[currentRow][currentCol].left();
      }

      currentRow = self.inner[currentRow][i].up();
    }
  }
}


// Iterators
impl DancingMatrix {
  /// An iterator for the header row
  pub fn iterHeader<'a>(&'a self) -> DancingMatrixGenericIterator<'a> {
    DancingMatrixGenericIterator { 
      inner: self.header(), start: 0, current: 0, 
      next_fn: |d: &DancingNode| -> uint { d.right() } 
    }
  }
}


impl ToStr for DancingMatrix {
  fn to_str(&self) -> ~str {
    let mut buf = ~"";
    let mut cols = ~[];

    for (col, _) in self.iterHeader() {
      cols.push(col);
    }

    for row in self.inner.slice_from(1).iter() {
      for col in cols.iter() {
        buf.push_str(row[*col].to_str() + ", ");
      }
      buf.push_char('\n');
    }

    buf
  }
}


impl Container for DancingMatrix {
  fn len(&self) -> uint {
    self.inner.len() - 1
  }
}


struct DancingMatrixGenericIterator<'a> {
  inner: &'a ~[DancingNode],
  start: uint,
  current: uint,
  next_fn: 'a |&'a DancingNode| -> uint 
}


impl<'a> Iterator<(uint, &'a DancingNode)> for DancingMatrixGenericIterator<'a> {
  fn next(&mut self) -> Option<(uint, &'a DancingNode)> {
    if (self.current == self.start) { return None }
    self.current = self.next_fn(self.inner[self.current]);
    Some((self.current, &self.inner[self.current]))
  }
}
