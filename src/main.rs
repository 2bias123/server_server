use std::{
    net::{TcpListener, TcpStream}, 
    io::{ Write, BufReader, BufRead, ErrorKind},
    fs::{File}
};


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
fn handle_connection(stream: TcpStream){


    //I use a buffreader beacause its more efficient than read
    let mut reader = BufReader::new(stream);

    let mut file = createfile("Helloe.txt");

    loop {
        let mut buffer = String::new();
        //Gets input from the client 
        let _input = reader.read_line(&mut buffer).unwrap();

        //Writes back to the client when the 
        reader.get_mut().write(&buffer.as_bytes()).unwrap();

        file.write(&buffer.as_bytes()).unwrap();
    }
}

fn createfile(filename: &str) -> File{
     //Creates a new text file
     let file = match File::options().read(true).write(true).open(filename){
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(file_created) => file_created,
                Err(err) => panic!("Couldnt create file {:?}",err),
            }, other_error => {
                panic!("Couldnt find the given file {:?}",other_error)
            }
        },
    };
    file
}

