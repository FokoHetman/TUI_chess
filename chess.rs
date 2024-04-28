mod Handlers;
use std::{
  io,
};

fn main() {
  let mut board = Handlers::Control::Board::build(/*Handlers::Board::boards::empty*/);
  let mut control = Handlers::Control::Controller::new(board.clone());
  print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
  println!("{}", board.show());

  let mut input = String::new();
  loop {
    input = String::new();
    let _ = io::stdin().read_line(&mut input).unwrap();
    let _ = control.mk_move(input);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{}", control.board.show());
  }

  /*  let mut input: String = String::new();
  loop {
    input = String::new();
    let _ = io::stdin().read_line(&mut input).unwrap();
    println!("{}\n{}", input, pieces.move_there(input.split(" ").collect::<Vec<&str>>()));
  }*/
}
