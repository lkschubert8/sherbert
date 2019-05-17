use std::net::TcpListener;
use std::env;

mod key_store;

fn main() {
    let args: Vec<String> = env::args().collect();

    //Get port from environment variables
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    println!("Starting up Sherbert server");
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        println!("Hit");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
