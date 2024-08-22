use std::{
  thread,
  sync::{Mutex,Arc},
  io::{Read,Write,self},
  net::{TcpStream,TcpListener},
  str
};

struct Program {
  value: String,
}

fn main() {
  let mut fprogram = Program {value: String::from("hello there") };
  
  let mutex = Mutex::new(fprogram);
  let program = Arc::new(mutex);
  thread::spawn(move || {setup_networking(program)});
  (mutex.lock().unwrap()).value = String::from("hi");
  loop {}
}

fn setup_networking(program: Arc<Mutex<Program>>) {
  let listener = TcpListener::bind("0.0.0.0:2137").unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();
    let clone = Arc::clone(&program);
    thread::spawn(move || {let _ = handle_con(stream, clone);});
  }
}

fn handle_con(mut stream: TcpStream, program: Arc<Mutex<Program>>) -> io::Result<()> {
  let mut buffer = [0;1024];
  stream.read(&mut buffer).unwrap();
  println!("{}", str::from_utf8(&buffer).unwrap());

  let contents = format!("hi: {}", (*program.lock().unwrap()).value);
  (*program.lock().unwrap()).value = String::from("hi2");
  let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}", length = contents.len());
  stream.write_all(response.as_bytes()).unwrap();
  Ok(())
}
