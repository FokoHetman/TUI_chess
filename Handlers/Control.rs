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
        let mut n_row: Board::Row = self.board.as_array()[n_pos[1].to_string().parse::<i32>().unwrap() as usize-1].clone();
        let mut n_cell = n_row.as_array()[nindex as usize].clone();
        n_cell.locator = o_cell.locator;
        o_cell.locator = Board::common::Empty::new();

        let id = n_cell.locator.getIcon();

        self.lmove = [id, moves[0].to_string(), moves[1].to_string()].to_vec();
        let mut arn_board = self.board.as_array();
        let mut arn_row = n_row.as_array();
        let mut aro_row = o_row.as_array();
      
        arn_row[nindex as usize] = n_cell;
        aro_row[index as usize] = o_cell;
      
        n_row.dump_array(arn_row);
        o_row.dump_array(aro_row);

        arn_board[n_pos[1].to_string().parse::<i32>().unwrap() as usize-1] = n_row;
        arn_board[o_pos[1].to_string().parse::<i32>().unwrap() as usize-1] = o_row;

        self.board.dump_array(arn_board);
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

        arn_row[nindex as usize] = n_cell;
        aro_row[index as usize] = o_cell;
        aro_row[lnindex as usize] = ln_cell;

        n_row.dump_array(arn_row);
        o_row.dump_array(aro_row);
 

        arn_board[n_pos[1].to_string().parse::<i32>().unwrap() as usize-1] = n_row;
        arn_board[o_pos[1].to_string().parse::<i32>().unwrap() as usize-1] = o_row;

        self.board.dump_array(arn_board);
      },
      Board::common::movestate::Illegal => {
        println!("Invalid move!");
      }
    }
    return true
  }
  fn hitscan(start:Vec<&str>, end:Vec<&str>) -> bool {
/*    let sindex = (Into::<u32>::into(start[0].to_string().to_lowercase().chars().next().unwrap()) -97) as i32;
    let eindex = (Into::<u32>::into(end[0].to_string().to_lowercase().chars().next().unwrap())-97) as i32;

    let n_pos = end.to_string().chars().collect::<Vec<char>>();
*/

    return true
  }
}
