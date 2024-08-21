mod httpHandler;

fn main() {
  println!("{:#?}", httpHandler::Response::new(httpHandler::make_request("0.0.0.0:2137", "/hi", "").unwrap()));
}
