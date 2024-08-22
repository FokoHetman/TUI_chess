pub mod httpHandler;
mod threading;
use crate::{Program};
use crate::Handlers;
use self::threading::ThreadPool;
use std::{
  net::{TcpListener, TcpStream},
  io,
  fs,
  str,
  io::{prelude::*, BufReader},
  collections::HashMap,
  path::{Path, PathBuf},
  sync::{Arc,Mutex},
  thread,
};
use Handlers::Control::Board::common;

pub fn estabilishListener(program: Arc<Mutex<Program>>) {
  let listener = TcpListener::bind("0.0.0.0:2137").unwrap();

  let pool = ThreadPool::new(2);
    


  //let uprog = &mut (*program.lock().unwrap());
  for stream in listener.incoming() {
    let stream = stream.unwrap();
    let clone = Arc::clone(&program);
    thread::spawn(move || {let _ = handle_con(stream, clone);});

  }

}

pub fn estabilishClientListener(program: Arc<Mutex<Program>>) {
  let listener = TcpListener::bind("0.0.0.0:5050").unwrap();
  let pool = ThreadPool::new(2);

  
  for stream in listener.incoming() {
    let stream = stream.unwrap();
    let clone = Arc::clone(&program);
    thread::spawn(move || {let _ = handle_client_con(stream, clone);});
    //let _ = handle_client_con(stream, &mut (*program.lock().unwrap()));

  }

}


fn handle_con(mut stream: TcpStream, program: Arc<Mutex<Program>>) -> io::Result<()> {
  //println!("handling con..");
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();



  let request = str::from_utf8(&buffer).unwrap();

  //println!("req: {}", request);
  let x = request.split("\r\n\r\n").collect::<Vec<&str>>()[1].replace("\0","");

  //println!("{}", str::from_utf8(&buffer).unwrap());

  let full_dir = request.split(" HTTP/1.1").collect::<Vec<&str>>()[0].replace("GET ","");

  let mut dir = "";
  if full_dir.contains("?") {
    dir = full_dir.split("?").collect::<Vec<&str>>()[0];
  } else {
    dir = &full_dir;
  }
  //stream.peer_addr().unwrap().to_string().split(":").collect::<Vec<&str>>()[0].to_string();
  



  let mut contents: String = String::new();



  println!("pre-matching dir: {}", dir);

  let mut program_lock = program.try_lock();
  if let Ok(ref mut mutex) = program_lock {
    //(**mutex).io = format!("lock acquired");

  //let mut program = &mut (*program_lock.as_mut().unwrap());
  match dir {

    "/gamestate" => {
      //println!("matched gamestate");
      let mut args = parseArgs(full_dir);
      let mut access = false;

      //println!("before");

      let passwd = (**mutex).passwd.clone();
      //let passwd = program.passwd.clone();
      //println!("after");
      for i in args {
        let arg_name: &str = &i.0;
        match arg_name {
          "passwd" => {
            if i.1==passwd {
              access = true;
            }
          },
          _ => {}
        }
      }

      //println!("matched password");
      if access {
        //println!("access");
           
          for mut row in (**mutex).control.board.as_array() {
            for mut cell in row.as_array() {
              contents+=&cell.locator.getIcon();
            }
            contents+=":";
          }
          contents+=";";

          contents += match (**mutex).player.clone() { Handlers::Control::Board::common::Colors::White => "white", _ => "black"};
          contents += ";";
          contents += match (**mutex).current_player.clone() { Handlers::Control::Board::common::Colors::White => "white", _ => "black"};
          //println!("colors done");
          (**mutex).client = format!("{}:5050", format!("{}", stream.peer_addr().unwrap()).split(":").collect::<Vec<&str>>()[0]);

        contents +=";";

      } else {
        contents = String::from("Access rejected.");
      }
    },
    "/move" => {
      //println!("matched /move");
      let mut args = parseArgs(full_dir);
      let mut access = false;
      let passwd = (**mutex).passwd.clone();

      for i in args {
        let arg_name: &str = &i.0;
        match arg_name {
          "passwd" => {
            if i.1==passwd {
              access = true;
            }
          },
          _ => {}
        }
      }

      if access {

          if (**mutex).control.chk_move(x.clone()) != Handlers::Control::Board::common::movestate::Illegal {
            (**mutex).control.mk_move(x.clone());
            //println!("made move!!");
            let cur_player = &(**mutex).current_player.clone();
            (**mutex).current_player = match cur_player {
              Handlers::Control::Board::common::Colors::White => Handlers::Control::Board::common::Colors::Black,
              _ => Handlers::Control::Board::common::Colors::White,
            };
            //println!("current player color updated");

            (**mutex).io = format!("Opponent made a move: {}", x);
            contents = String::from("update");
          } else {
            (**mutex).io = format!("malformed http request with move: {}", x);
            contents = String::from("Malformed status. Access rejected.");
          }

      } else {
        contents = String::from("Access rejected.");
      }
      //println!("end of clause /move");
    },
    _ => {
      //println!("{}", request);

      (**mutex).io = format!("malformed http request: {}", request);

      contents = String::from("Malformed request. Access rejected.");
    }
  }
  //println!("pre-redraw");
  crate::clear();
  crate::redraw(&mut (**mutex));
  
  } else {
    contents = String::from("something holds the mcfoking lock sory");
    //println!("LOCK IS USED BY SOMETHING ELSE ALREADY");
  }
  drop(program_lock);
  //println!("LOCK DROPPED!!");






  let length = contents.len();
  let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}");

  //println!("res: {};;; to req:{}", response, request);


  stream.write_all(response.as_bytes()).unwrap();

  Ok(())
}



fn handle_client_con(mut stream: TcpStream, program: Arc<Mutex<Program>>) -> io::Result<()> {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();



  let request = str::from_utf8(&buffer).unwrap();

  //println!("req: {}", request);
  let x = request.split("\r\n\r\n").collect::<Vec<&str>>()[1].replace("\0","");

  //println!("{}", str::from_utf8(&buffer).unwrap());

  let full_dir = request.split(" HTTP/1.1").collect::<Vec<&str>>()[0].replace("GET ","");

  let mut dir = "";
  if full_dir.contains("?") {
    dir = full_dir.split("?").collect::<Vec<&str>>()[0];
  } else {
    dir = &full_dir;
  }


  let mut contents: String = String::new();

  let mut program_lock = program.try_lock();
  if let Ok(ref mut program) = program_lock {
    match dir {
      "/update" => {
        //println!("UPDATIN!!!");
        //crate::update_gamestate(&mut (**program));
      
        let game_state = x.split(";").collect::<Vec<&str>>();

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
     },
      _ => {
        //println!("{}", request);
        (**program).io = format!("malformed http request: {}", request);
         contents = String::from("Malformed request. Access rejected.");
      }
    }
    crate::clear();
    crate::redraw(&mut (**program));
  } else {
    contents = String::from("something holds the lock, sori :<");
  }
  drop(program_lock);





  let length = contents.len();
  let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}");
  stream.write_all(response.as_bytes()).unwrap();

  Ok(())
}








fn parseArgs(path: String) -> Vec<(String, String)> {
  if !path.contains("?") {
    return vec![];
  }
  let mut rvalues = path.split("?").collect::<Vec<&str>>().into_iter();
  rvalues.next();
  let mut result: Vec<(String, String)> = vec![];
  for i in rvalues {
    if i.contains("=") {
      result.push((i.split("=").collect::<Vec<&str>>()[0].to_string(), i.split("=").collect::<Vec<&str>>()[1].to_string()));
    } else {
      result.push((i.split("=").collect::<Vec<&str>>()[0].to_string(), i.split("=").collect::<Vec<&str>>()[0].to_string()));
    }
  }
  return result
}
