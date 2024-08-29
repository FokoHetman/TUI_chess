mod Handlers;
use libc;
use std::{
  io::{self, Read, Write, IsTerminal},
  sync::{Arc,Mutex},
  thread,
  env,

  net::TcpStream,
  str,
};
use Handlers::Control::Board::common;


static termios: Mutex<libc::termios> = Mutex::new(libc::termios { c_iflag: 0, c_oflag: 0, c_cflag: 0, c_lflag: 0, c_line: 1, c_cc: [0 as u8; 32], c_ispeed: 1, c_ospeed: 1 });


fn setup_termios() {
  termios.lock().unwrap().c_cflag &= !libc::CSIZE;
  termios.lock().unwrap().c_cflag |= libc::CS8;
  termios.lock().unwrap().c_cc[libc::VMIN] = 1;
}

extern "C" fn disable_raw_mode() {
  unsafe {
    libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &(*termios.lock().unwrap()));
  }
}
fn enable_raw_mode() {
  unsafe {
    libc::tcgetattr(libc::STDIN_FILENO, &mut *termios.lock().unwrap());
    libc::atexit(disable_raw_mode);
    let mut raw = *termios.lock().unwrap();
    raw.c_lflag &= !(libc::ECHO | libc::ICANON);
    libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &raw);
  }
}


#[repr(C)]              /// shout out (github.com) softprops/termsize!!
#[derive(Debug)]
pub struct UnixSize {
    pub rows: libc::c_ushort,
    pub cols: libc::c_ushort,
    x: libc::c_ushort,
    y: libc::c_ushort,
}

pub struct TerminalSize {pub rows: u16, pub cols: u16}

fn get_terminal_size() -> Option<TerminalSize> {
  if !std::io::stdout().is_terminal() {
    return None;
  }
  let mut us = UnixSize { // fuck windows
    rows: 0,
    cols: 0,
    x: 0,
    y: 0,
  };
  let r = unsafe { libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut us) };
  if r == 0 {
    Some(TerminalSize{
      rows: us.rows,
      cols: us.cols,
    })
  } else {
    None
  }
}





#[derive(Debug)]
pub struct KeyEvent {
  pub code: KeyCode,
  pub modifiers: Vec<Modifier>,
}
#[derive(Debug,PartialEq)]
pub enum Modifier {
  Control,
  //why even do it at this point
}
#[derive(Debug,PartialEq)]
pub enum Direction {
  Up,
  Down,
  Right,
  Left
}

#[derive(Debug,PartialEq)]
pub enum KeyCode {
  Escape,
  Colon,
  Enter,
  Backspace,
  Arrow(Direction),
  Char(char),
}

#[derive(Debug,PartialEq)]
enum States {
  Control,
  Input,
  Command,
}

const escape_char: char = 27 as char;
const backspace_char: char = '\u{7f}';
const tab: char = '\t';
const enter: char = '\n';

struct Program {
  state: States,
  io: String,
  control: Handlers::Control::Controller,
  input: String,
  exit: bool,
  multiplayer: bool,
  current_player: Handlers::Control::Board::common::Colors,
  player: Handlers::Control::Board::common::Colors,
  server: String,
  client: String,
  passwd: String,
  is_hosting: bool,
}


fn clear() {
  print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
fn redraw(program: &mut Program) {
  //println!("{}", get_terminal_size().unwrap().cols);
  let mut showed = format!("{}\n{}\n{}", program.control.board.show(Some(Handlers::Control::Board::ShowMode::borders)), 
      vec!['-'; get_terminal_size().unwrap().cols as usize].iter().collect::<String>(), program.input);

  let input_line = showed.split("\n").collect::<Vec<&str>>().len() as i32;
  showed += &format!("\n{}", vec!['-'; get_terminal_size().unwrap().cols as usize].iter().collect::<String>());

  showed += &format!("\nYour Pieces: {}\nCurrent Turn: {}\n", 
      match program.player {Handlers::Control::Board::common::Colors::White => "White", _ => "Black"}, match program.current_player {Handlers::Control::Board::common::Colors::White => "White", _ => "Black"});

  let to_last_line = get_terminal_size().unwrap().rows as i32 - showed.split("\n").collect::<Vec<&str>>().len() as i32;
  for i in 0..to_last_line {
    showed += "\n"
  }
  showed += &format!("\x1b[48;5;54m{}{}\x1b[0m", &program.io, vec![' '; get_terminal_size().unwrap().cols as usize - program.io.len()].into_iter().collect::<String>());

  
  print!("{}", showed);
  match program.state {
    States::Control => {},
    States::Input => {
      print!("\x1b[{line};{col}H", line=input_line, col = program.input.len()+1);
    },
    States::Command => {
      print!("\x1b[{line};{col}H", line = showed.split("\n").collect::<Vec<&str>>().len() as i32,col = program.io.len()+1);
    },
  }
  std::io::stdout().flush().unwrap();
}


fn parse_event(event: KeyEvent, program: &mut Program) {
  match program.state {
    States::Control => {
      match event.code {
        KeyCode::Escape => {
          program.io = String::from("use :q to leave");
        },
        KeyCode::Colon => {
          program.state = States::Command;
          program.io = String::from(":");
        },
        KeyCode::Char(c) => {
          match c {
            'i' => {program.state = States::Input},
            _ => {},
          }
        },
        _ => {},
      }
    },
    States::Input => {
      match event.code {
        KeyCode::Enter => {
          if program.current_player == program.player {
            if !program.is_hosting && program.multiplayer {
              if program.control.chk_move(program.input.clone()) != Handlers::Control::Board::common::movestate::Illegal && program.control.chk_color(program.input.clone(), program.player.clone()) {
              //let _ = program.control.mk_move(program.input.clone());
                
                program.control.temp_move(program.input.clone());
                let king_pos = program.control.get_king(program.current_player.clone()).chars().collect();
                if !program.control.check_checks(king_pos, program.current_player.clone()) {
                
              

                  /*let sgame_state = Handlers::server::httpHandler::Response::new(
                    Handlers::server::httpHandler::make_request(
                    &program.server, &format!("/move?passwd={}", program.passwd), &program.input.clone()
                   ).unwrap()).content;*/

                  let _ = program.control.mk_move(program.input.clone());


                  //let mut stream = TcpStream::connect(&program.server).map_err(|e| e.to_string()).unwrap();

                  let response = Handlers::server::httpHandler::Response::new(Handlers::server::httpHandler::make_request(&program.server, &format!("/move?passwd={}", program.passwd), &program.input.clone()).unwrap());

                  if response.content.contains("update") {
                    update_gamestate(program);
                  }
                  //program.io = response.content;
                  //program.stream = Some(stream);
                } else {
                  let rev_code = program.input.clone().clone().split(" ").collect::<Vec<&str>>()[1].to_string() + " " + program.input.clone().split(" ").collect::<Vec<&str>>()[0];
                  program.control.temp_move(rev_code);
                  program.io = String::from("You're in check!");
                }
                

                
                
              } else {
                program.io = String::from("Illegal move!");
              }



            } else {
              
              //println!("{:#?} a {}", program.control.chk_move(program.input.clone()), program.input.clone());

              if program.control.chk_move(program.input.clone()) != Handlers::Control::Board::common::movestate::Illegal && program.control.chk_color(program.input.clone(), program.player.clone()) {


                program.control.temp_move(program.input.clone());
                let king_pos: Vec<char> = program.control.get_king(program.current_player.clone()).chars().collect();
                program.io = king_pos.clone().into_iter().collect::<String>();
                if !program.control.check_checks(king_pos, program.current_player.clone()) {


                let _ = program.control.mk_move(program.input.clone());
              //println!("move made!");
              
                program.current_player = match program.current_player {
                  Handlers::Control::Board::common::Colors::White => Handlers::Control::Board::common::Colors::Black,
                  _ => Handlers::Control::Board::common::Colors::White,
                };


                if program.multiplayer {
                 

                  let mut contents = String::from("");
                  for mut row in program.control.board.as_array() {
                    for mut cell in row.as_array() {
                      contents+=&cell.locator.getIcon();
                    }
                    contents+=":";
                  }
                  contents+=";";

                  contents += match program.player.clone() { Handlers::Control::Board::common::Colors::White => "white", _ => "black"};
                  contents += ";";
                  contents += match program.current_player.clone() { Handlers::Control::Board::common::Colors::White => "white", _ => "black"};
                  //println!("colors done");
                  //program.client = format!("{}:5050", format!("{}", stream.peer_addr().unwrap()).split(":").collect::<Vec<&str>>()[0]);

                  contents +=";";

                  Handlers::server::httpHandler::make_request(&program.client.clone(), "/update", &contents);
                }

                } else {
                  let rev_code = program.input.clone().clone().split(" ").collect::<Vec<&str>>()[1].to_string() + " " + program.input.clone().split(" ").collect::<Vec<&str>>()[0];
                  program.control.temp_move(rev_code);
                  program.io = String::from("You're in check!");
                }
                

                

              } else {
                program.io = String::from("Illegal Move!");
              }
            }
            
          } else {
            program.io = String::from("Not your turn!");
          }
          program.input = String::new();
        },
        KeyCode::Backspace => {
          let mut chars = program.input.chars();
          chars.next_back();
          program.input = chars.as_str().to_string();
        },
        KeyCode::Char(c) => {
          program.input += &c.to_string();
        },
        KeyCode::Colon => {
          program.input += ":";
        },
        KeyCode::Escape => {
          program.state = States::Control;
        },
        KeyCode::Arrow(d) => {
          /*match d {
            Direction::Left => {
              
            }
          }*/
        },
        _ => {}
      }
    },
    States::Command => {
      match event.code {
        KeyCode::Escape => {
          program.state = States::Control;
        },
        KeyCode::Backspace => {
          let mut chars = program.io.chars();
          chars.next_back();
          program.io = chars.as_str().to_string();

          if program.io.len() == 0 {
            program.state = States::Control;
          }
        },
        KeyCode::Colon => {
          program.io += ":";
        },
        KeyCode::Char(c) => {
          program.io += &c.to_string();
        },
        KeyCode::Enter => {
          let mut iochars = program.io.chars();
          iochars.next();
          match iochars.as_str() {
            "q" => {
              program.exit = true;
            },
            _ => {program.io = String::from("unknown command!")}
          }
          //sometime in the futrureererree!
          //program.io = String::new();
          program.state = States::Control;
        },
        KeyCode::Arrow(d) => {

        },
        _ => {}
      }
    }
  }
}

fn getch() -> char {
  io::stdin().bytes().next().unwrap().unwrap() as char
}


fn update_gamestate(program: &mut Program) {

  let response = Handlers::server::httpHandler::make_request(&program.server.clone(), &format!("/gamestate?passwd={}", program.passwd), "");


  let sgame_state = Handlers::server::httpHandler::Response::new(response.unwrap()).content;
  //program.stream = Some(stream);



  let game_state = sgame_state.split(";").collect::<Vec<&str>>();

  if game_state.len()==1 {
    panic!("no access, gamestate: {:#?}", game_state);
  }

  program.player = match game_state[1] {
    "black" => common::Colors::White,
    "white" => common::Colors::Black,
    _ => panic!("improper host's  game state, color: {}\nfull_gamestate: {:#?}", game_state[1], game_state),
  };
  program.current_player = match game_state[2] {
    "black" => common::Colors::Black,
    "white" => common::Colors::White,
    _ => panic!("improper host's  game state, current_color: {}\nfull_gamestate: {:#?}", game_state[2], game_state),
  };
  let rows = game_state[0].split(":").collect::<Vec<&str>>();
  let mut row_id = 0;
  let mut boarda = program.control.board.as_array();
  for mut row in &mut boarda {
    let mut cell_id = 0;
    let mut rcells = rows[row_id].split("").collect::<Vec<&str>>().into_iter();
    rcells.next();
    rcells.next_back();
    let cells = rcells.collect::<Vec<&str>>();
    //println!("{:#?}", cells);
    let mut rowa = row.as_array();
    for mut cell in &mut rowa {
     cell.locator = match cells[cell_id] {
        "P" => common::Pawn::new(common::Colors::White),
        "R" => common::Rook::new(common::Colors::White),
        "K" => common::Knight::new(common::Colors::White),
        "B" => common::Bishop::new(common::Colors::White),
        "Q" => common::Queen::new(common::Colors::White),
        "T" => common::King::new(common::Colors::White),
        "p" => common::Pawn::new(common::Colors::Black),
        "r" => common::Rook::new(common::Colors::Black),
        "k" => common::Knight::new(common::Colors::Black),
        "b" => common::Bishop::new(common::Colors::Black),
        "q" => common::Queen::new(common::Colors::Black),
        "t" => common::King::new(common::Colors::Black),
        " " => common::Empty::new(),
        _ => panic!("improper host's game state (board): {:#?}", cells[cell_id]),
        
      };
      cell_id+=1;
    }
    row.dump_array(rowa.clone());
    row_id+=1;
  }
  program.control.board.dump_array(boarda.clone());
}



fn main_loop() {
  let mut board = Handlers::Control::Board::build();
  let mut fprogram = Program{
    state: States::Control,
    io: String::new(),
    input: String::new(),
    control: Handlers::Control::Controller::new(board.clone()),
    exit: false,
    multiplayer: env!("CHESS_MULTIPLAYER")=="1",
    current_player: Handlers::Control::Board::common::Colors::White,
    player: Handlers::Control::Board::common::Colors::White,
    server: String::new(),
    client: String::new(),
    passwd: String::new(),
    is_hosting: false,

  };


  let program = Arc::new(Mutex::new(fprogram));

  let args = env::args().collect::<Vec<String>>();
  //println!("{:#?}", args);
  let mut program_lock = program.lock();
  if (*program_lock.as_mut().unwrap()).multiplayer {
    let clone = Arc::clone(&program);
    if args[1] == String::from("host") {
      (*program_lock.as_mut().unwrap()).is_hosting = true;
      if args.len()>2 {
        (*program_lock.as_mut().unwrap()).passwd = args[2].clone();
      }
      
      thread::spawn(move || {Handlers::server::estabilishListener(clone)});

    } else {
      (*program_lock.as_mut().unwrap()).server = args[2].clone();
      (*program_lock.as_mut().unwrap()).passwd = args[3].clone();

      update_gamestate(&mut (*program_lock.as_mut().unwrap()));

      thread::spawn(move || {Handlers::server::estabilishClientListener(clone)});

      /*let sgame_state = Handlers::server::httpHandler::Response::new(
          Handlers::server::httpHandler::make_request(
            &args[2].clone(), &format!("/gamestate?passwd={}", args[3].clone()), ""
          ).unwrap()).content;*/
    }
  }
  

  //print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
  //println!("{}", board.show());
  clear();
  redraw(&mut (*program_lock.as_mut().unwrap()));

  drop(program_lock);

  for b in io::stdin().bytes() {
    
    //println!("{:#?}", (*program.lock().unwrap()).state);
    
    let c = b.unwrap() as char;
    //println!("{}", c);
    let mut modifiers: Vec<Modifier> = vec![];
    if c.is_control() && ![escape_char, backspace_char, tab, enter].contains(&c) {
      modifiers.push(Modifier::Control);
    }
    let event = KeyEvent{
      code: match c { escape_char => KeyCode::Escape, backspace_char => KeyCode::Backspace,':' => KeyCode::Colon, '\n' => KeyCode::Enter, 
          '\u{1b}' => {let _ = getch(); KeyCode::Arrow(match getch() {'A' => Direction::Up, 'B' => Direction::Down, 'C' => Direction::Right, 'D' => Direction::Left, _ => panic!("literally how")})},
          _ => KeyCode::Char(c)},
      modifiers,
    };
    //println!("{:#?}", event);
    let mut program_lock = program.lock();

    parse_event(event, &mut (*program_lock.as_mut().unwrap()));
    clear();
    if (*program_lock.as_mut().unwrap()).exit {
      break
    }
    redraw(&mut (*program_lock.as_mut().unwrap()));
    drop(program_lock);
  }
}






fn main() {
  //println!("{}", env!("HELP_ME").to_string());
  setup_termios();

  enable_raw_mode();

  



  main_loop();

  disable_raw_mode();

  /*let mut input = String::new();
  let mut current_move = Handlers::Control::Board::common::Colors::White;
  loop {
    print!("$ ");
    input = String::new();
    let _ = io::stdin().read_line(&mut input).unwrap();
    if control.chk_move(input.clone()) == Handlers::Control::Board::common::movestate::Illegal {
      let _ = control.mk_move(input);
      print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
      println!("{}", control.board.show());
    }
  }*/
}
