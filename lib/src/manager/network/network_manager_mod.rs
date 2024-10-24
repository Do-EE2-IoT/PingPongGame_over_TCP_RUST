pub mod network_manager{
    use crate::manager::network::{
        client_mod::client::Client, 
        server_mod::server::Server
    };

    pub struct NetworkManager{
        pub server : Server,
        pub client : Client
    }
    impl NetworkManager {
        pub fn new(server : Server, client : Client) -> Self {
            Self {
                server,
                client
            }
        }

        pub fn update_network(){
            todo!();
        }
    }
}