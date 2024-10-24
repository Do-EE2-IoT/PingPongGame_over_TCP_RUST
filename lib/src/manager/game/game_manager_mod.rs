pub mod game_manager {
    use crate::manager::game::{
        ball_mod::ball::Ball, game_state_mod::game_state::GameState, paddle_mod::paddle::Paddle,
        player_mod::player::Player,
    };

    pub struct GameManager {
        player1: Player,
        player2: Player,
        ball: Ball,
        paddle1: Paddle,
        paddle2: Paddle,
        game_state: GameState,
    }

    impl GameManager {
        fn new(
            player1: Player,
            player2: Player,
            ball: Ball,
            paddle1: Paddle,
            paddle2: Paddle,
            game_state: GameState,
        ) -> Self {
            Self {
                player1,
                player2,
                ball,
                paddle1,
                paddle2,
                game_state,
            }
        }
    }
}
