pub mod player {

    pub struct Player {
        pub id: u32,
        pub username: String,
        pub score: u32,
    }

    impl Player {
        pub fn new(id: u32, username: String, score: u32) -> Self {
            Self {
                id,
                username,
                score,
            }
        }

        fn increase_score(&mut self) {
            self.score += 1
        }
    }
}