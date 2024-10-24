pub mod ball {

    #[derive(Debug)]
    pub struct Ball {
        pub position_x: u32,
        pub position_y: u32,
        pub speed_x: u32,
        pub speed_y: u32,
    }

    impl Ball {
        fn new(speed_x: u32, speed_y: u32) -> Self {
            Self {
                position_x: 0,
                position_y: 0,
                speed_x,
                speed_y,
            }
        }

        fn update_position(&mut self) {
            todo!();
        }
    }
}
