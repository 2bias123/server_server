use std::{net::{TcpListener, TcpStream}, io::{Read, Write, BufReader, BufRead}};
use std::fs::File;


fn main() {
    let listener = TcpListener::bind("localhost:8080").unwrap();

    //Stream that listens for connections
    for stream in listener.incoming(){
        match stream {
            Ok(tcp_stream) => {
                println!("Connected");
                handle_connection(tcp_stream);
            },

            Err(error) => println!("Some error{:?}",error),
        }
    }
}

//Runs when someone connects to the server
fn handle_connection(mut stream: TcpStream){


    //Creates a new text file
    let mut file = match File::create("helloe.txt") {
        Ok(file) => file,
        Err(error) =>{
            eprintln!("Something went wrong{}",error);
            return;
        },
    };

    //I use a buffreader beacause its more efficient than read
    let mut reader = BufReader::new(stream);

    
    loop {
        let mut buffer = String::new();
        //Gets input from the client 
        let _input = reader.read_line(&mut buffer).unwrap();

        //Writes back to the client when the 
        reader.get_mut().write(buffer.as_bytes()).unwrap();

        file.write()
    }
}





