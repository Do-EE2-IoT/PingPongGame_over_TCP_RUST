pub mod paddle {
    use crate::manager::game::player_mod::player::Player;

    pub struct Paddle {
        pub player: Player,
        pub position_x: u32,
        pub position_y: u32,
        pub width: u32,
        pub height: u32,
    }

    impl Paddle {
        fn new(player: Player, position_x: u32, position_y: u32, width: u32, height: u32) -> Self {
            Self {
                player,
                position_x,
                position_y,
                width,
                height,
            }
        }

        fn move_paddle(&mut self) {
            todo!();
        }
    }
}
