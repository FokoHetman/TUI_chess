pub mod Board;

pub struct Controller {pub board: Board::Board, pub lmove: Vec<String>}


impl Controller {
  pub fn new(board: Board::Board) -> Self {
    return Controller {board, lmove: [String::from("e"), String::from("a4"), String::from("c4")].to_vec()};
  }
  pub fn chk_move(&mut self, code: String) -> Board::common::movestate { // lmove 0->figure 1->opos 2->npos
    let moves = code.split(" ").collect::<Vec<&str>>();

    let mv0 = moves[0].to_string();
    let o_pos = mv0.chars().collect::<Vec<char>>();


    let index = Into::<u32>::into(o_pos[0].to_string().to_lowercase().chars().next().unwrap()) -97;
    let mut o_row: Board::Row = self.board.as_array()[o_pos[1].to_string().parse::<i32>().unwrap() as usize-1].clone();
    let mut o_cell = o_row.as_array()[index as usize].clone();
    if match o_cell.locator {
      Board::common::Pieces::Pawn(_) => self.linear_hitscan(o_pos, moves[1].to_string().chars().collect::<Vec<char>>()),
      Board::common::Pieces::Rook(_) => self.linear_hitscan(o_pos, moves[1].to_string().chars().collect::<Vec<char>>()),
      _ => false
    } {
      return Board::common::movestate::Illegal;
    }

    let n_pos = moves[1].to_string().chars().collect::<Vec<char>>();
    let nindex = Into::<u32>::into(n_pos[0].to_string().to_lowercase().chars().next().unwrap())-97;
    let mut n_row: Board::Row = self.board.as_array()[n_pos[1].to_string().parse::<i32>().unwrap() as usize-1].clone();
    let mut n_cell = n_row.as_array()[nindex as usize].clone();

//    println!("{:#?} => {:#?}", o_cell, n_cell);
    return o_cell.locator.check_move(moves, n_cell.locator, self.lmove.clone());
  }
  pub fn mk_move(&mut self, code: String) -> bool {
    match self.chk_move(code.clone()) {

      Board::common::movestate::Legal => {
        let moves = code.split(" ").collect::<Vec<&str>>();
        let mv0 = moves[0].to_string();
      
        let o_pos = mv0.chars().collect::<Vec<char>>();
        let index = Into::<u32>::into(o_pos[0].to_string().to_lowercase().chars().next().unwrap()) -97;
        let mut o_row: Board::Row = self.board.as_array()[o_pos[1].to_string().parse::<i32>().unwrap() as usize-1].clone();
        let mut o_cell = o_row.as_array()[index as usize].clone();

        let n_pos = moves[1].to_string().chars().collect::<Vec<char>>();
        let nindex = Into::<u32>::into(n_pos[0].to_string().to_lowercase().chars().next().unwrap())-97;
        let mut arn_board = self.board.as_array();
        let mut n_cell: Board::Cell;
        if o_pos[1]==n_pos[1] {
          n_cell = o_row.as_array()[nindex as usize].clone();
          n_cell.locator = o_cell.locator;
          o_cell.locator = Board::common::Empty::new();




          let mut aro_row = o_row.as_array();
          aro_row[index as usize] = o_cell;
          aro_row[nindex as usize] = n_cell.clone();
          o_row.dump_array(aro_row);
          arn_board[o_pos[1].to_string().parse::<i32>().unwrap() as usize-1] = o_row;
        } else {
          let mut n_row: Board::Row = self.board.as_array()[n_pos[1].to_string().parse::<i32>().unwrap() as usize-1].clone();
          n_cell = n_row.as_array()[nindex as usize].clone();
          n_cell.locator = o_cell.locator;
          o_cell.locator = Board::common::Empty::new();



          let mut arn_row = n_row.as_array();
          let mut aro_row = o_row.as_array();
      
//        println!("{:#?} and {:#?}", arn_row, aro_row);
          arn_row[nindex as usize] = n_cell.clone();
          aro_row[index as usize] = o_cell;

          o_row.dump_array(aro_row);
          n_row.dump_array(arn_row);

          arn_board[o_pos[1].to_string().parse::<i32>().unwrap() as usize-1] = o_row;
          arn_board[n_pos[1].to_string().parse::<i32>().unwrap() as usize-1] = n_row;
        }
        
        self.board.dump_array(arn_board);
        
        let id = n_cell.locator.getIcon();
        self.lmove = [id, moves[0].to_string(), moves[1].to_string()].to_vec();
      },
      Board::common::movestate::EnPassant => {
        let moves = code.split(" ").collect::<Vec<&str>>();
        let mv0 = moves[0].to_string();

        let o_pos = mv0.chars().collect::<Vec<char>>();
        let index = Into::<u32>::into(o_pos[0].to_string().to_lowercase().chars().next().unwrap()) -97;
        let mut o_row: Board::Row = self.board.as_array()[o_pos[1].to_string().parse::<i32>().unwrap() as usize-1].clone();
        let mut o_cell = o_row.as_array()[index as usize].clone();

        let n_pos = moves[1].to_string().chars().collect::<Vec<char>>();
        let nindex = Into::<u32>::into(n_pos[0].to_string().to_lowercase().chars().next().unwrap())-97;
        if o_pos[1]==n_pos[1] {
          let mut n_cell = o_row.as_array()[nindex as usize].clone();
          n_cell.locator = o_cell.locator;
          o_cell.locator = Board::common::Empty::new();

          let id = n_cell.locator.getIcon();

          self.lmove = [id, moves[0].to_string(), moves[1].to_string()].to_vec();
          let mut arn_board = self.board.as_array();
          let mut aro_row = o_row.as_array();
        } else {
          let mut n_row: Board::Row = self.board.as_array()[n_pos[1].to_string().parse::<i32>().unwrap() as usize-1].clone();
          let mut n_cell = n_row.as_array()[nindex as usize].clone();
          n_cell.locator = o_cell.locator;
          o_cell.locator = Board::common::Empty::new();

          let id = n_cell.locator.getIcon();

          self.lmove = [id, moves[0].to_string(), moves[1].to_string()].to_vec();
          let mut arn_board = self.board.as_array();
          let mut arn_row = n_row.as_array();
          let mut aro_row = o_row.as_array();
  
          let ln_pos = self.lmove[2].clone().chars().collect::<Vec<char>>();
          let lnindex = Into::<u32>::into(ln_pos[0].to_string().to_lowercase().chars().next().unwrap())-97;
          let mut ln_cell = o_row.as_array()[lnindex as usize].clone();
          ln_cell.locator = Board::common::Empty::new();

          aro_row[index as usize] = o_cell;
          aro_row[lnindex as usize] = ln_cell;
          arn_row[nindex as usize] = n_cell;
          o_row.dump_array(aro_row);
          n_row.dump_array(arn_row); 

          arn_board[o_pos[1].to_string().parse::<i32>().unwrap() as usize-1] = o_row;
          arn_board[n_pos[1].to_string().parse::<i32>().unwrap() as usize-1] = n_row;

          self.board.dump_array(arn_board);
        }
      },
      Board::common::movestate::Illegal => {
        println!("Invalid move!");
      }
    }
    return true
  }
  fn cursive_hitscan(&mut self, start:Vec<char>, end:Vec<char>) -> bool {
/*    let s_pos = start;
    println!("{:#?}", s_pos);
    let s_index = (Into::<u32>::into(s_pos[0].to_string().to_lowercase().chars().next().unwrap())-97) as i32;
    let s_num = s_pos[1].to_string().parse::<i32>().unwrap();

    let n_pos = end;
    let n_index = (Into::<u32>::into(n_pos[0].to_string().to_lowercase().chars().next().unwrap())-97) as i32;
    let n_num = n_pos[1].to_string().parse::<i32>().unwrap();

    println!("{} n {}; + result: {};- result: {}", s_index, s_num, s_index+s_num, s_num-s_index);
    println!("+result: {} -result: {}", n_index+n_num, n_num-n_index);
*/ //BISHOP CHECK :D

    let s_index = (Into::<u32>::into(start[0].to_string().to_lowercase().chars().next().unwrap())-97) as i32;
    let s_num = start[1].to_string().parse::<i32>().unwrap();

    let n_index = (Into::<u32>::into(end[0].to_string().to_lowercase().chars().next().unwrap())-97) as i32;
    let n_num = end[1].to_string().parse::<i32>().unwrap();

    if s_num+s_index == n_num+n_index || s_num-s_index == n_num-n_index {

      return true
    }

    return false
  }
  fn linear_hitscan(&mut self, start:Vec<char>, end:Vec<char>) -> bool {
    let s_index = (Into::<u32>::into(start[0].to_string().to_lowercase().chars().next().unwrap())-97) as i32;
    let s_num = start[1].to_string().parse::<i32>().unwrap();

    let n_index = (Into::<u32>::into(end[0].to_string().to_lowercase().chars().next().unwrap())-97) as i32;
    let n_num = end[1].to_string().parse::<i32>().unwrap();
    if s_num==n_num {
      let mut cur_row = self.board.as_array()[n_num as usize-1].clone();
      let mut iterator = match s_index>n_index {
        true => n_index,
        _ => s_index
      };
      let mut finishe = match s_index>n_index {
        true => s_index,
        _ => n_index
      }-1;
      while iterator<finishe {
        //println!("{:#?}", cur_row.as_array()[iterator as usize].locator);
        iterator+=1;
        if match cur_row.as_array()[iterator as usize].locator {
          Board::common::Pieces::Empty(_) => false,
          _ => true
        } {
          return true
        }
      }
    }
    else if n_index==s_index {
      let mut iterator = match s_num>n_num {
        true => n_num,
        _ => s_num
      };
      let mut finishe = match s_num>n_num {
        true => s_num,
        _ => n_num
      }-1;

      while iterator<finishe {
        //println!("{:#?}", self.board.as_array()[iterator as usize].as_array()[n_index as usize]);
        if match self.board.as_array()[iterator as usize].as_array()[n_index as usize].locator {
          Board::common::Pieces::Empty(_) => false,
          _ => true,
        } {
          return true
        }
        iterator+=1
      }
    }
    return false
  }
}







