use std::old_io::TcpStream;
use std::old_io::net::ip::ToSocketAddr;
use std::old_io::net::ip::SocketAddr;
use std::old_io::BufferedStream;
use std::old_io::stdin;
use std::thread::Thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::Arc;

fn main(){
  let mut reader = stdin();
  print!("[Pick a username]:");
  let mut name: String = reader.read_line().ok().expect("Something went wrong");
  print!("[Enter host:port to connect to]:");
  let server = reader.read_line().ok().expect("Something went wrong").trim().to_socket_addr().unwrap();
  print!("[Enter chan to join]:");
  let mut chan: String = reader.read_line().ok().expect("Something went wrong");
  let mut sock = TcpStream::connect(server).unwrap();
  let mut q = sock.clone();
  let mut chanx = chan.clone();
  let thread = Thread::spawn(move || {
    let mut soc = BufferedStream::new(sock.clone());
	let nick_packet = format!("NICK {}\r\n", name);
	let user_packet = format!("USER {0} {0} {0} :{0}\r\n", name.trim());
	sock.write_all(nick_packet.as_bytes());
	sock.flush();
	sock.write_all(user_packet.as_bytes());
	sock.flush();
	let mut connected = true;
	let mut soc = BufferedStream::new(sock.clone());
	while connected {
	  let mut data = soc.read_line().unwrap();
	  println!("{}", data);
	  if data.contains("PING "){
        let hashbit = data.split_str("PING ").nth(1).unwrap().trim();      
        let packet: String = format!("PONG {}\r\n", hashbit);
	    sock.write_all(packet.as_bytes());
	    sock.flush();
	  }
	  if data.contains(" 376 "){
	    sock.write_all(format!("JOIN {}\r\n", chanx).as_bytes());
	    sock.flush();
	  }
	  if data.contains("ERROR :Closing "){
	    connected = false;
	  }
    }
  });
  let thread_ = Thread::spawn( move || {
    let mut shared_s = BufferedStream::new(q);
    let mut connected = true;
    let mut chan_ = chan.clone();
    while connected{
      print!("[You]>");
      let mut reader = stdin();
      let input = reader.read_line().ok().expect("Something went wrong");
      let command = input.clone();
      if input.as_bytes()[0] == b'/' {
        if command.contains("/quit") {
          println!("QUIT COMMAND FOUND!");
          shared_s.write("QUIT : Rust IRC Client exited\r\n".as_bytes());
          shared_s.flush();
          connected = false;
          panic!("Client killed!");
        }
        else if command.contains("/nick"){
          let mut nick = command.split_str("/nick ").nth(1).unwrap();
          shared_s.write_all(format!("NICK {}\r\n", nick).as_bytes());
          shared_s.flush();
        }
        else if command.contains("/join"){
          let channel = command.split_str("/join ").nth(1).unwrap();
        }
      }
      else{
        let msg = format!("PRIVMSG {} :{}\r\n", chan.clone().trim(), command);
        println!("{}", msg);
        shared_s.write_all(msg.as_bytes());
        shared_s.flush();
      }
    }
  });
  while true{}
}
