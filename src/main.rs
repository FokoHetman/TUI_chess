mod Handlers;
use libc;
use std::{
  io::{self, Read, Write, IsTerminal},
  sync::Mutex,
};


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
}


fn clear() {
  print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
fn redraw(program: &mut Program) {
  //println!("{}", get_terminal_size().unwrap().cols);
  let mut showed = format!("{}\n{}\n{}", program.control.board.show(Some(Handlers::Control::Board::ShowMode::borders)), vec!['-'; get_terminal_size().unwrap().cols as usize].iter().collect::<String>(), program.input);

  let input_line = showed.split("\n").collect::<Vec<&str>>().len() as i32;
  let to_last_line = get_terminal_size().unwrap().rows as i32 - input_line;
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
          println!("{:#?} a {}", program.control.chk_move(program.input.clone()), program.input.clone());

          if program.control.chk_move(program.input.clone()) != Handlers::Control::Board::common::movestate::Illegal {
            let _ = program.control.mk_move(program.input.clone());
            println!("move made!");
          } else {
            program.io = String::from("Invalid Move!");
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


fn main_loop() {
  let mut board = Handlers::Control::Board::build();
  let mut program = Program{
    state: States::Control,
    io: String::new(),
    input: String::new(),
    control: Handlers::Control::Controller::new(board.clone()),
    exit: false,
  };



  //print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
  //println!("{}", board.show());
  clear();
  redraw(&mut program);


  for b in io::stdin().bytes() {
    
    println!("{:#?}", program.state);
    
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

    parse_event(event, &mut program);
    if program.exit {
      break
    }
    clear();
    redraw(&mut program);
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
