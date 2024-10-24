pub mod client{
    use std::net::TcpStream;
    pub struct Client{
        pub stream : TcpStream
    }

    impl Client {
        pub fn connect_to_server(){
            todo!();
        }

        pub fn send_action(){
            todo!();
        }
        pub fn receive_game_state(){
            todo!();
        }
    }
}