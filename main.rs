use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

fn send_msg(mut stream: &TcpStream, msg: &str) -> std::io::Result<()> {
    stream.write(msg.as_bytes())?;
    println!("{}", msg);
    return Ok(());
}

fn handle_ping(s: &str, stream: &TcpStream) -> std::io::Result<()> {
    // get the ping message
    let ping_msg = s.split("PING :").last().unwrap();
    // send the pong message
    let pong_msg = format!("PONG :{}\r\n", ping_msg);
    send_msg(stream, &pong_msg)?;
    return Ok(());
}

//fn handle_privmsg(s: &str, stream: &TcpStream) -> std::io::Result<()> {
    // get the ping message
//    let ping_msg = s.split("PING :").last().unwrap();
    // send the pong message
//    let pong_msg = format!("PONG :{}\r\n", ping_msg);
//    send_msg(stream, &pong_msg)?;
//    return Ok(());
//}

fn handle_line(s: &str, stream: &TcpStream) -> std::io::Result<()> {
    println!("[{}]", s);
    if s.contains("PING") {
        return handle_ping(&s, &stream);
    }
    // if channel successfully joined, add to channel list
    else if s.contains("332") {
        let mut split_0 = s.split(" ");
        // grab the first part of the split
        let split_0_0 = split_0.next().unwrap();
        // grab the second part of the split
        let split_0_1 = split_0.last().unwrap();
        println!("##### s: {}", s);
        println!("split_0_0: {}", split_0_0);
        println!("split_0_1: {}", split_0_1);
    }
    else if s.contains("353") {
        // get the channel name
        println!("######################");
        let channel_name = s.split("353 ").last().unwrap().split(" :").next().unwrap();
        println!("Channel name: {}", channel_name);
        // get the users
        let users = s.split("353 ").last().unwrap().split(" :").last().unwrap().split(" ");
        for user in users {
            println!("User: {}", user);
        }
    }
    return Ok(());
}


fn main() -> std::io::Result<()> {
    // get args
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        println!("Usage: {} <server:port> <nick> <channel>", args[0]);
        std::process::exit(1);
    }
    // parse args
    let server = &args[1];
    let nick = &args[2];
    let channel = &args[3];
    //let mut channel_list: Vec<Channel> = Vec::new();
    // check if server is empty
    if server.is_empty() {
        println!("Server must be in the format <server:port>");
        std::process::exit(1);
    }
    if !server.contains(":") {
        println!("Server must be in the format <server:port>");
        std::process::exit(1);
    }
    if nick.is_empty() {
        println!("Nick must not be empty");
        std::process::exit(1);
    }
    if channel.is_empty() {
        println!("Channel must not be empty");
        std::process::exit(1);
    }
    if !channel.starts_with("#") {
        println!("Channel must start with #");
        std::process::exit(1);
    }

    let nick_msg = format!("NICK {}\r\n", nick);
    let user_msg = format!("USER {} 0 * :{}\r\n", nick, nick);
    let join_msg = format!("JOIN {}\r\n", channel);

    let mut stream = TcpStream::connect(server)?;
    // Send the message
    stream.write(nick_msg.as_bytes())?;
    stream.write(user_msg.as_bytes())?;
    stream.write(join_msg.as_bytes())?;

    // read the response in a loop
    loop {
        // clear the buffer
        let mut buf = [0; 4096];
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        let s = String::from_utf8_lossy(&buf[..]);
        // split the string into lines
        let lines = s.split("\r\n");
        for line in lines {
            if line.is_empty() {
                continue;
            }
            handle_line(line, &stream)?;
        }
    }
    //return Ok(());
} 

