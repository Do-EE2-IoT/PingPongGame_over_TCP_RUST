pub mod player {

    pub struct Player {
        pub id: u32,
        pub username: String,
        pub score: u32,
    }

    impl Player {
        pub fn increase_score(&mut self) {
            self.score += 1
        }
    }
}
