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
};



pub fn estabilishListener(program: Arc<Mutex<Program>>) {
  let listener = TcpListener::bind("0.0.0.0:2137").unwrap();
  let pool = ThreadPool::new(8);

  
  for stream in listener.incoming() {
    let stream = stream.unwrap();
    let arc_cloned = std::sync::Arc::clone(&program);
    pool.execute(|| {let _ = handle_con(stream, arc_cloned);});
  }

}


fn handle_con(mut stream: TcpStream, arc_program: Arc<Mutex<Program>>) -> io::Result<()> {

  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();
  let x = str::from_utf8(&buffer).unwrap().split("\r\n\r\n").collect::<Vec<&str>>()[1].replace("\0","");


  //stream.peer_addr().unwrap().to_string().split(":").collect::<Vec<&str>>()[0].to_string();
  
  let mut contents: &str = "";

  if arc_program.lock().unwrap().control.chk_move(x.clone()) != Handlers::Control::Board::common::movestate::Illegal {
    arc_program.lock().unwrap().control.mk_move(x.clone());
    arc_program.lock().unwrap().io = format!("Opponent made a move: {}", x);
  } else {
    arc_program.lock().unwrap().io = format!("malformed http request with move: {}", x);
  }



  let length = contents.len();
  let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}");

  stream.write_all(response.as_bytes()).unwrap();
  Ok(())
}
