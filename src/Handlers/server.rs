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
    /*let clone = Arc::clone(&program);
    thread::spawn(move || {let _ = handle_client_con(stream, clone);});*/
    let _ = handle_client_con(stream, &mut (*program.lock().unwrap()));

  }

}


fn handle_con(mut stream: TcpStream, program: Arc<Mutex<Program>>) -> io::Result<()> {
  println!("handling con..");
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();



  let request = str::from_utf8(&buffer).unwrap();

  println!("req: {}", request);
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

  
  match dir {

    "/gamestate" => {
      println!("matched gamestate");
      let mut args = parseArgs(full_dir);
      let mut access = false;

      println!("before");

      let passwd = (*program.lock().unwrap()).passwd.clone();
      println!("after");
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
      println!("matched password");
      if access {
        println!("access");
        for mut row in (*program.lock().unwrap()).control.board.as_array() {
          for mut cell in row.as_array() {
            contents+=&cell.locator.getIcon();
          }
          contents+=":";
        }
        contents +=";";

        contents += match (*program.lock().unwrap()).player.clone() { Handlers::Control::Board::common::Colors::White => "white", _ => "black"};
        contents += ";";
        contents += match (*program.lock().unwrap()).current_player.clone() { Handlers::Control::Board::common::Colors::White => "white", _ => "black"};
        println!("colors done");
        (*program.lock().unwrap()).client = format!("{}:5050", format!("{}", stream.peer_addr().unwrap()).split(":").collect::<Vec<&str>>()[0]);
      } else {
        contents = String::from("Access rejected.");
      }
    },
    "/move" => {
      println!("matched /move");
      let mut args = parseArgs(full_dir);
      let mut access = false;
      let passwd = (*program.lock().unwrap()).passwd.clone();
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
        if (*program.lock().unwrap()).control.chk_move(x.clone()) != Handlers::Control::Board::common::movestate::Illegal {
          (*program.lock().unwrap()).control.mk_move(x.clone());
          println!("made move!!");
          let cur_player = &(*program.lock().unwrap()).current_player.clone();
          (*program.lock().unwrap()).current_player = match cur_player {
            Handlers::Control::Board::common::Colors::White => Handlers::Control::Board::common::Colors::Black,
            _ => Handlers::Control::Board::common::Colors::White,
          };
          println!("current player color updated");

          (*program.lock().unwrap()).io = format!("Opponent made a move: {}", x);
          contents = String::from("update");
        } else {
          (*program.lock().unwrap()).io = format!("malformed http request with move: {}", x);
          contents = String::from("Malformed status. Access rejected.");
        }
      } else {
        contents = String::from("Access rejected.");
      }
      println!("end of clause /move");
    },
    _ => {
      println!("{}", request);
      (*program.lock().unwrap()).io = format!("malformed http request: {}", request);
      contents = String::from("Malformed request. Access rejected.");
    }
  }
  println!("pre-redraw");
  crate::redraw(&mut (*program.lock().unwrap()));






  let length = contents.len();
  let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}");

  println!("res: {};;; to req:{}", response, request);

  //drop(program);
  stream.write_all(response.as_bytes()).unwrap();

  Ok(())
}



fn handle_client_con(mut stream: TcpStream, program: &mut Program) -> io::Result<()> {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();



  let request = str::from_utf8(&buffer).unwrap();

  println!("req: {}", request);
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

  match dir {
    "/update" => {
      println!("UPDATIN!!!");
      crate::update_gamestate(program);
      
    },
    _ => {
      println!("{}", request);
      program.io = format!("malformed http request: {}", request);
      contents = String::from("Malformed request. Access rejected.");
    }
  }
  crate::redraw(program);






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
