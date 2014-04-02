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
  pub fn decr(&mut self) {
    match *self {
      ColumnNode(_, ref mut x) => *x -= 1,
      Root(_) => (),
      _ => fail!("decr applied on invalid node!")
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
  pub fn deleteCol(&mut self, i: uint) {
    assert!(i < self.cols());

    if (i != 0) {
      debug!("Deleting {:u}...", i);

      let left = self.header()[i].left();
      let right = self.header()[i].right();

      self.inner[0][left].setRight(right);
      self.inner[0][right].setLeft(left);
    }
  }
  /// Deletes a node
  pub fn deleteNode(&mut self, row: uint, col: uint) {
    assert!(row < self.len() + 1);
    assert!(col < self.cols());

    debug!("Deleting ({:u}, {:u})...", row, col);

    let up = self.inner[row][col].up();
    let down = self.inner[row][col].down();

    self.inner[up][col].setDown(down);
    self.inner[down][col].setUp(up);

    self.inner[0][col].decr();
  }
  /// Undelete col
  pub fn undeleteCol(&mut self, i: uint) {
    assert!(i < self.cols());

    if (i != 0) {
      debug!("Undeleting {:u}...", i);

      let left = self.header()[i].left();
      let right = self.header()[i].right();

      self.inner[0][left].setRight(i);
      self.inner[0][right].setLeft(i);
    }
  }
  /// Undelete a node
  pub fn undeleteNode(&mut self, row: uint, col: uint) {
    assert!(row < self.len() + 1);
    assert!(col < self.cols());

    debug!("Undeleting ({:u}, {:u})...", row, col);

    let up = self.inner[row][col].up();
    let down = self.inner[row][col].down();

    self.inner[up][col].setDown(row);
    self.inner[down][col].setUp(row);

    self.inner[0][col].incr();
  }
  /// Covers a column in the inner 
  /// matrix
  pub fn coverCol(&mut self, i: uint) {
    assert!(i < self.cols());

    if (i == 0) { return }

    self.deleteCol(i);

    let mut currentCol;
    let mut currentRow = self.inner[0][i].down();

    while (currentRow != 0) {
      currentCol = self.inner[currentRow][i].right();

      while (currentCol != i) {
        self.deleteNode(currentRow, currentCol);

        currentCol = self.inner[currentRow][currentCol].right();
      }

      currentRow = self.inner[currentRow][i].down();
    }
  }
  /// Uncovers a column in the inner matrix
  pub fn uncoverCol(&mut self, i: uint) {
    assert!(i < self.cols());

    if (i == 0) { return; }

    self.undeleteCol(i);

    let mut currentCol;
    let mut currentRow = self.inner[0][i].up();

    while (currentRow != 0) {
      currentCol = self.inner[currentRow][i].left();

      while (currentCol != i) {
        self.undeleteNode(currentRow, currentCol);

        currentCol = self.inner[currentRow][currentCol].left();
      }

      currentRow = self.inner[currentRow][i].up();
    }
  }
}


// Iterators
impl DancingMatrix {
  /// An iterator for the header row
  pub fn iterHeader<'a>(&'a self) -> DancingMatrixHeaderIterator<'a> {
    DancingMatrixHeaderIterator { inner: self.header(), current: self.root().right() } 
  }
  /// An iterator across all the rows
  pub fn iterRows<'a>(&'a self) -> DancingMatrixRowIterator<'a> {
    DancingMatrixRowIterator { inner: self, current: self.root().down() }
  }
}


impl ToStr for DancingMatrix {
  fn to_str(&self) -> ~str {
    let mut buf = ~"";
    let mut cols = ~[];

    for (col, _) in self.iterHeader() {
      cols.push(col);
    }

    for (_, row) in self.iterRows() {
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


struct DancingMatrixHeaderIterator<'a> {
  inner: &'a ~[DancingNode],
  current: uint
}


impl<'a> Iterator<(uint, &'a DancingNode)> for DancingMatrixHeaderIterator<'a> {
  /// Gives a tuple (Column, Node)
  fn next(&mut self) -> Option<(uint, &'a DancingNode)> {
    if (self.current == 0) { return None }
    let tmp = self.current;
    self.current = self.inner[self.current].right();
    Some((tmp, &self.inner[tmp]))
  }
}


struct DancingMatrixRowIterator<'a> {
  inner: &'a DancingMatrix,
  current: uint
}


impl<'a> Iterator<(uint, &'a ~[DancingNode])> for DancingMatrixRowIterator<'a> {
  /// Gives a tuple (Row_Num, Row)
  fn next(&mut self) -> Option<(uint, &'a ~[DancingNode])> {
    if (self.current == 0) { return None }
    let tmp = self.current;
    self.current = self.inner.get(self.current, 0).down();
    Some((tmp, &self.inner.inner[tmp]))
  }
}
