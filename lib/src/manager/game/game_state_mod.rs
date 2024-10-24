pub mod game_state {
    use crate::manager::game::player_mod::player::Player;

    pub enum GameStatus {
        Inprogress,
        Player1win,
        Player2win,
        Begin,
    }

    pub struct GameState {
        status: GameStatus,
        max_score: u32,
    }

    impl GameState {
        fn new(max_score: u32) -> Self {
            Self {
                status: GameStatus::Begin,
                max_score,
            }
        }

        fn update_status(&self) -> GameStatus {
            GameStatus::Begin
        }
    }
}
