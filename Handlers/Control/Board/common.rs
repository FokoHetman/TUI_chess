#[derive(Clone,Debug,PartialEq)]
pub enum Colors {
  White,
  Black,
}
#[derive(Clone,Debug)]
pub struct Empty {icon:String}

#[derive(Clone,Debug)]
pub struct Pawn{value: i32, color: Colors, icon:String}

#[derive(Clone,Debug)]
pub struct Rook{value: i32, color: Colors, icon:String}

#[derive(Clone, Debug)]
pub struct Knight{value: i32, color: Colors, icon:String}

#[derive(Clone, Debug)]
pub struct Bishop{value: i32, color: Colors, icon:String}

#[derive(Clone, Debug)]
pub struct Queen{value: i32, color: Colors, icon:String}

#[derive(Clone, Debug)]
pub struct King{value: i32, color: Colors, icon:String}

#[derive(Clone,Debug)]
pub enum Pieces {
  Empty(Empty),
  Pawn(Pawn),
  Rook(Rook),
  Knight(Knight),
  Bishop(Bishop),
  Queen(Queen),
  King(King),
}


pub enum movestate {
  Legal,
  Illegal,
  EnPassant,
}
impl Pawn {
  pub fn new(color: Colors) -> Pieces {
    return Pieces::Pawn(Pawn {value:1,color,icon:String::from("P")});
  }
  pub fn check_move(&mut self, code: Vec<&str>, entity: Pieces, lmove: Vec<String>) -> movestate {
    let o_pos = code[0].to_string().chars().collect::<Vec<char>>();
    let n_pos = code[1].to_string().chars().collect::<Vec<char>>();

    let opos = o_pos[1].to_string().parse::<i32>().unwrap();
    let npos = n_pos[1].to_string().parse::<i32>().unwrap();
    let oindex = Into::<u32>::into(o_pos[0].to_string().to_lowercase().chars().next().unwrap()) -96;
    let nindex = Into::<u32>::into(n_pos[0].to_string().to_lowercase().chars().next().unwrap()) -96;
    match entity.clone() {
      Pieces::Pawn(e) => {if e.color==self.color {return movestate::Illegal}},
      Pieces::Knight(e) => {if e.color==self.color {return movestate::Illegal}},
      Pieces::Bishop(e) => {if e.color==self.color {return movestate::Illegal}},
      Pieces::Queen(e) => {if e.color==self.color {return movestate::Illegal}},
      Pieces::King(e) => {if e.color==self.color {return movestate::Illegal}},
      Pieces::Rook(e) => {if e.color==self.color {return movestate::Illegal}},
      _ => {}
    }
    return match entity {
      Pieces::Empty(Empty) => {
        if oindex!=nindex {
          if oindex!=nindex+1 && oindex!=nindex-1 {
            return movestate::Illegal;
          }
          let mv0 = lmove[1].clone();
          
          let o_pos = mv0.chars().collect::<Vec<char>>();
          let lindex = Into::<u32>::into(o_pos[0].to_string().to_lowercase().chars().next().unwrap()) -97;


          let n_pos = lmove[2].clone().chars().collect::<Vec<char>>();
          let lnindex = Into::<u32>::into(n_pos[0].to_string().to_lowercase().chars().next().unwrap())-97;

          if n_pos[1].to_string().parse::<i32>().unwrap()!=opos {
            return movestate::Illegal
          }
          if lnindex !=nindex+1 && lnindex !=nindex-1 {
            return movestate::Illegal
          }
          return movestate::EnPassant

        }
        match self.color {
          Colors::White => {
            if (npos-opos>2 || npos-opos<1) || (npos-opos==2 && opos!=2) {
              return movestate::Illegal
            }
          },
          Colors::Black => {
            if (opos-npos>2 || opos-npos<1) || (opos-npos==2 && opos!=7) {
              return movestate::Illegal
            }
          }
        }
        return movestate::Legal

      }
      _ => {
        if oindex!=nindex+1 && oindex!=nindex-1 {
          return movestate::Illegal
        }
        match self.color {
          Colors::White => {
            if npos-opos!=1 {
              return movestate::Illegal
            }
          }
          Colors::Black => {
            if opos-npos!=1 {
              return movestate::Illegal
            }
          }

        }
        return movestate::Legal
      }
    }
  }
}

impl Empty {
  pub fn new() -> Pieces {
    return Pieces::Empty(Empty {icon:String::from(" ")});
  }
}

impl Rook {
  pub fn new(color: Colors) -> Pieces {
    return Pieces::Rook(Rook {value: 5,color,icon:String::from("R")});
  }
  pub fn check_move(&mut self, code: Vec<&str>, entity: Pieces) -> movestate {
    return movestate::Legal
  }
}
impl Knight {
  pub fn new(color: Colors) -> Pieces {
    return Pieces::Knight(Knight {value: 3,color,icon:String::from("K")});
  }
  pub fn check_move(&mut self, code: Vec<&str>, entity: Pieces) -> movestate {
    let o_pos = code[0].to_string().chars().collect::<Vec<char>>();
    let n_pos = code[1].to_string().chars().collect::<Vec<char>>();

    let opos = o_pos[1].to_string().parse::<i32>().unwrap();
    let npos = n_pos[1].to_string().parse::<i32>().unwrap();
    let oindex = (Into::<u32>::into(o_pos[0].to_string().to_lowercase().chars().next().unwrap()) -96) as i32;
    let nindex = (Into::<u32>::into(n_pos[0].to_string().to_lowercase().chars().next().unwrap()) -96) as i32;
    match entity.clone() {
      Pieces::Pawn(e) => {if e.color==self.color {return movestate::Illegal}},
      Pieces::Knight(e) => {if e.color==self.color {return movestate::Illegal}},
      Pieces::Bishop(e) => {if e.color==self.color {return movestate::Illegal}},
      Pieces::Queen(e) => {if e.color==self.color {return movestate::Illegal}},
      Pieces::King(e) => {if e.color==self.color {return movestate::Illegal}},
      Pieces::Rook(e) => {if e.color==self.color {return movestate::Illegal}},
      _ => {}
    }
    if (npos-opos).abs()+(nindex-oindex).abs()!=3 {
      return movestate::Illegal
    }
    if npos==opos || oindex==nindex {
      return movestate::Illegal
    }

    return movestate::Legal
  }
}
impl Bishop {
  pub fn new(color: Colors) -> Pieces {
    return Pieces::Bishop(Bishop {value: 3,color,icon:String::from("B")});
  }
  pub fn check_move(&mut self, code: Vec<&str>, entity: Pieces) -> movestate {
    return movestate::Legal
  }
}
impl Queen {
  pub fn new(color: Colors) -> Pieces {
    return Pieces::Queen(Queen {value: 8,color,icon:String::from("Q")});
  }
  pub fn check_move(&mut self, code: Vec<&str>, entity: Pieces) -> movestate {
    return movestate::Legal
  }
}
impl King {
  pub fn new(color: Colors) -> Pieces {
    return Pieces::King(King {value:0,color,icon:String::from("✝")});
  }
  pub fn check_move(&mut self, code: Vec<&str>, entity: Pieces) -> movestate {
    return movestate::Legal
  }
}


impl Pieces {
  pub fn getIcon(&mut self) -> String {
    return match self {
      Pieces::Pawn(Pawn) => Pawn.icon.clone(),
      Pieces::Rook(Rook) => Rook.icon.clone(),
      Pieces::Knight(Knight) => Knight.icon.clone(),
      Pieces::Bishop(Bishop) => Bishop.icon.clone(),
      Pieces::Queen(Queen) => Queen.icon.clone(),
      Pieces::King(King) => King.icon.clone(),
      Pieces::Empty(Empty) => Empty.icon.clone(),
      _ => String::from("?"),
    }
  }
  pub fn check_move(&mut self, code:Vec<&str>, entity: Pieces, lmove: Vec<String>) -> movestate {
    return match self {
      Pieces::Pawn(Pawn) => Pawn.check_move(code, entity, lmove),
      Pieces::Rook(Rook) => Rook.check_move(code, entity),
      Pieces::Knight(Knight) => Knight.check_move(code, entity),
      Pieces::Bishop(Bishop) => Bishop.check_move(code, entity),
      Pieces::Queen(Queen) => Queen.check_move(code, entity),
      Pieces::King(King) => King.check_move(code, entity),
      _ => movestate::Illegal,
    }
  }
}
