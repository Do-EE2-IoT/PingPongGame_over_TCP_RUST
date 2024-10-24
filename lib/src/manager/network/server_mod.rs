pub mod server{
    use std::net::TcpStream;
    use crate::manager::game::player_mod::player::Player;

    pub struct Server{
        pub game : Player,//game manager
        pub client : Vec<TcpStream>
    }

    impl Server{
        pub fn new(game : Player , client : Vec<TcpStream>)->Self{
            Self{
                game,
                client
            }
        }

        pub fn start_server(){
            todo!();
        }

        pub fn update_status(){
            todo!();
        }
    }
}