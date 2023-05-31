use std::io::{Error, ErrorKind}
use std::net::ToSocketAddrs;
use std::net::TcpStream;
use std::io;
use std::fs::{OpenOptions, File};
use std::io::{Read, Write, Seek, SeekFrom, BufRead};
use std::os::windows::prelude::FileExt;
use std::str::from_utf8;

const BUF_LENGTH: usize = 1400;
pub struct User {
    username: String,
    password: String,
    handle: TcpStream
}

impl User {
    /// Creates a new user. Connets to remote addr server.
    pub fn new<A: ToSocketAddrs>(username: String, password: String, addr: A) -> Self{
        Self { 
            username, 
            password,
            handle: match TcpStream::connect(addr) {
                Ok(handle) => handle,
                Err(error) => panic!("Problem setting the stream: {:?}", error),
            }
        }
    }
    /// Begins interacting with server by authenticating user's credentials. Runs CLI if succesful.
    pub fn start_session(&self) {
        match self.auth_me() {
            Ok(_) => {
                self.run();
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
    /// Runs authentication interaction with server. Ok() for success, Err() for failure
    pub fn auth_me(&self) -> io::Result<()> {
        let mut buffer: [u8; 20] = [0;20];
        let mut credentials = String::from(self.username);
        credentials.push_str(" ");
        credentials.push_str(&self.password);
        match self.handle.write(credentials.as_bytes()) {
            Err(_) => Err(Error::new(ErrorKind::ConnectionRefused, "Unable to send credentials to server.")),
            Ok(_) => Ok(()),
        };
        match self.handle.read(&mut buffer) {
            Ok(0) => Err(Error::new(ErrorKind::ConnectionRefused, "Unable to read server response.")),
            Ok(n) => {
                match from_utf8(&buffer[..n]) {
                    Ok("0") => Err(Error::new(ErrorKind::ConnectionRefused, "Authentication denied, try again.")),
                    Ok("1") => Ok(())
                }
            },
            Err(_) => Err(Error::new(ErrorKind::ConnectionRefused, "Unable to read server response")),
        }
    }
    /// Runs CLI
    pub fn run(&self) {
        loop { 
            let command = get_input();
            match command.0.as_str() {
                "cd" => {
                    match command.1 {
                        None => self.send_command(command),
                        Some(_) => {
                            self.send_command(command);
                            self.fetch_output();
                        }
                    }
                }
                "dir" => {
                    self.send_command(command);
                    self.fetch_output();           
                }
                "mkdir" | "rmdir" | "echo." | "del "=> {
                    self.send_command(command);
                }
                "download" => {
                    
                }
                "upload" => {
                    sys_commands::receive_file(
                    self.user.as_ref().unwrap(), 
                    OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(path)
                            .unwrap()
                    );
                }
                _ => (),

                

            }

        // .trim().parse::<u8>().expect("Dupa");
        todo!();
        }
    }
    pub fn print_interface() {
        todo!();
    }
/// Sends command to the server. Option< String> handles possibility that the command may not return any output therefor no data packets are to be received.
    pub fn send_command(&self, mut commands: (String, Option<String>)){
        match commands.1 {
            Some(_) =>{
            commands.0.push_str(commands.1.unwrap().trim());
            self.handle.write( commands.0.as_bytes());
            }
            None => { self.handle.write( commands.0.as_bytes()); },
        }
    }
    ///Fetches the command output from the server and prints it in the terminal.
    pub fn fetch_output(&self) {
        let mut buffer = [0;BUF_LENGTH];
        self.handle.read(&mut buffer);
        println!("{}", from_utf8(&buffer).unwrap())
    }
    }
///Cut input into command and Option< argument>
pub fn get_input() -> (String, Option<String>) {
    let mut command = String::new();
    io::stdin().read_line(&mut command);
    let splited: Vec<&str>= command.trim().split(' ').collect();
    let mut arg = splited.get(2);
    let mut arg2: Option<String>;
    match arg {
        Some(n) => arg2 = Some(n.to_string()),
        None => arg2 = None,
    }
    return (splited[0].to_string(), arg2)
}