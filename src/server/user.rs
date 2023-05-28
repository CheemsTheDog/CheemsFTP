use std::{net::TcpStream, io::{Read, BufReader}, fs::File};
/// Authenticates user with given credentials
pub fn auth_usr(mut connection: TcpStream, path: String) -> Result<(), ()> {
    let mut temp: String = String::new();
    connection.read_to_string(&mut temp);
    let temp_vec: Vec<&str> = temp.trim().split(" ").collect();
    let user_entries_file = File::open(&self.user_entries).unwrap();
    let buffer = BufReader::new(user_entries_file);
    for line in buffer.lines() {
        match line {
            Ok(line) => {
//1 -> login, 2 -> password, 3 -> usertype, 4 -> login, 5 -> login, 6 -> login, 7 -> login, 8 -> login, 
                    let user_entry: Vec<&str>= line.trim().split(" ").collect();
                    let mut user_type: logged_user::UserType;
                    if temp_vec[0] == user_entry[0] && temp_vec[1] == user_entry[1] {
                        let mut user_type: logged_user::UserType = {
                            match user_entry[2] {
                                "0" => logged_user::UserType::Admin,
                                "1" => logged_user::UserType::Moderator,
                                "2" => logged_user::UserType::Default,
                                 _  => logged_user::UserType::Default,
                            }};  
                        return Ok( LoggedUser::new( 
                                        user_type, 
                                        temp_vec[0].to_owned(), 
                                        temp_vec[1].to_owned(), 
                                        connection,
                                        1000 , 
                                        Privileges::new(
                                            true, 
                                            true, 
                                            true, 
                                            true
                        ))); }}
                    Err(_) => (),
                    }}
        return Err(());

}
