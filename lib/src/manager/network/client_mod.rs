pub mod client{
    use std::net::TcpStream;
    pub struct Client{
        pub stream : TcpStream
    }

    impl Client {
        pub fn connect_to_server(addr:&str) -> std::io::Result<Client>{
            match TcpStream::connect(addr){
                Ok(stream)=>{
                    Ok(Client {stream})
                }Err(e)=>{
                    Err(e)
                }
            }
        }

        pub fn send_action(){
            todo!();
        }
        pub fn receive_game_state(){
            todo!();
        }
    }
}



