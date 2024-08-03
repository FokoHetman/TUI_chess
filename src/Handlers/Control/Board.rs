pub mod common;
use self::common::{Pieces, Colors};
use self::common::{Empty, Pawn, Rook, Knight, Bishop, Queen, King};
#[derive(Clone,Debug,PartialEq)]
pub struct Cell {
  pub locator: Pieces,
}

#[derive(Clone,Debug)]
pub struct Row {
  a: Cell,
  b: Cell,
  c: Cell,
  d: Cell,
  e: Cell,
  f: Cell,
  g: Cell,
  h: Cell
}
#[derive(Clone)]
pub struct Board {
  a: Row,
  b: Row,
  c: Row,
  d: Row,
  e: Row,
  f: Row,
  g: Row,
  h: Row,
}

impl Cell {
  fn empty() -> Self {
    return Cell {locator: Empty::new()};
  }
  fn new(locator: Pieces) -> Self {
    return Cell {locator: locator};
  }
}
impl Row {
  fn new(a:Cell,b:Cell,c:Cell,d:Cell,e:Cell,f:Cell,g:Cell,h:Cell) -> Self {
    return Row {a,b,c,d,e,f,g,h};
  }
  fn newOne(a:Cell) -> Self {
    return Row{a: a.clone(), b:a.clone(), c:a.clone(), d:a.clone(), e:a.clone(), f:a.clone(), g:a.clone(), h:a.clone()};
  }
  pub fn as_array(&mut self) -> [Cell; 8] {
    [self.a.clone(),self.b.clone(),self.c.clone(),self.d.clone(),self.e.clone(),self.f.clone(),self.g.clone(),self.h.clone()]
  }
  pub fn dump_array(&mut self, array: [Cell; 8]) {
    *self = Row {a: array[0].clone(), b: array[1].clone(), c: array[2].clone(), d: array[3].clone(), e: array[4].clone(), f: array[5].clone(), g: array[6].clone(), h: array[7].clone()};
  }
}
impl Board {
  fn new() -> Self {
    let mut erow = Row::newOne(Cell::empty());
    return Board {a: erow.clone(), b: erow.clone(), c: erow.clone(), d: erow.clone(), e: erow.clone(), f: erow.clone(), g: erow.clone(), h: erow.clone()};
  }
  fn default() -> Self {
    let mut erow=Row::newOne(Cell::empty());

    let mut white_pawns = Row::newOne(Cell::new(Pawn::new(Colors::White)));
    let mut black_pawns = Row::newOne(Cell::new(Pawn::new(Colors::Black)));

    let mut white_row = Row::new(  Cell::new(Rook::new(Colors::White)), Cell::new(Knight::new(Colors::White)), Cell::new(Bishop::new(Colors::White)), Cell::new(Queen::new(Colors::White)), Cell::new(King::new(Colors::White)), Cell::new(Bishop::new(Colors::White)), Cell::new(Knight::new(Colors::White)), Cell::new(Rook::new(Colors::White)) );
    let mut black_row = Row::new(  Cell::new(Rook::new(Colors::Black)), Cell::new(Knight::new(Colors::Black)), Cell::new(Bishop::new(Colors::Black)), Cell::new(Queen::new(Colors::Black)), Cell::new(King::new(Colors::Black)), Cell::new(Bishop::new(Colors::Black)), Cell::new(Knight::new(Colors::Black)), Cell::new(Rook::new(Colors::Black)) );

    return Board {a:white_row, b: white_pawns, c: erow.clone(), d: erow.clone(), e:erow.clone(), f:erow.clone(), g: black_pawns, h: black_row};
  }
  pub fn show(&mut self) -> String {
    let mut res:String = String::new();
    let mut r_index = 1;
    for mut row in self.as_array() {
      
      for mut cell in row.as_array() {
        /*res += &match cell.locator.clone() {
          Pieces::Pawn(e) => { match e.color {Colors::White =>   cell.locator.clone().getIcon(),  Colors::Black =>  cell.locator.clone().getIcon().to_lowercase()  }},
          Pieces::Rook(e) => { match e.color {Colors::White =>   cell.locator.clone().getIcon(),  Colors::Black =>  cell.locator.clone().getIcon().to_lowercase()  }},
          Pieces::Knight(e) => { match e.color {Colors::White =>   cell.locator.clone().getIcon(),  Colors::Black =>  cell.locator.clone().getIcon().to_lowercase()  }},
          Pieces::Bishop(e) => { match e.color {Colors::White =>   cell.locator.clone().getIcon(),  Colors::Black =>  cell.locator.clone().getIcon().to_lowercase()  }},
          Pieces::King(e) => { match e.color {Colors::White =>   cell.locator.clone().getIcon(),  Colors::Black =>  cell.locator.clone().getIcon().to_lowercase()  }},
          Pieces::Queen(e) => { match e.color {Colors::White =>   cell.locator.clone().getIcon(),  Colors::Black =>  cell.locator.clone().getIcon().to_lowercase()  }},
          _ => "".to_string()
        };*/
        res += &cell.locator.getIcon()
      }
      res += "  ";      // DEBATABLE
      res += &r_index.to_string();   // DEBATABLE
      res += "\n";
      r_index += 1;
    }
    res += "\nabcdefgh"; // DEBATABLE
    return res
  }
  pub fn as_array(&mut self) -> [Row; 8] {
    [self.a.clone(),self.b.clone(),self.c.clone(),self.d.clone(),self.e.clone(),self.f.clone(),self.g.clone(),self.h.clone()]
  }
  pub fn dump_array(&mut self, array: [Row; 8]) {
    *self = Board {a:array[0].clone(),b:array[1].clone(),c:array[2].clone(),d:array[3].clone(),e:array[4].clone(),f:array[5].clone(),g:array[6].clone(),h:array[7].clone() };
  }
}
pub fn build() -> Board {
  let mut board = Board::default();

  return board;
}
