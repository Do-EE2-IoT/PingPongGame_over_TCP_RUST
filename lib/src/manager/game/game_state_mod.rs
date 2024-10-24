pub mod game_state {
    use crate::manager::game::player_mod::player::Player;

    pub enum GameStatus {
        Inprogress,
        Player1win,
        Player2win,
        Begin,
    }

    pub struct GameState {
        player: Player,
        status: GameStatus,
        max_score: u32,
    }

    impl GameState {
        fn new(id: u32, username: String, score: u32, max_score: u32) -> Self {
            Self {
                player: Player {
                    id,
                    username,
                    score,
                },
                status: GameStatus::Begin,
                max_score,
            }
        }

        fn update_status(&self) -> GameStatus {
            GameStatus::Begin
        }
    }
}
