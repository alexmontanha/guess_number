pub mod machine {
    use druid::{Data, Lens};
    use rand::Rng;

    #[derive(Clone, Data, Lens)]
    pub struct StateMachine {
        pub name: String,
        pub player_name: String,
        pub entry: String,
        pub place_holder: String,
        pub message: String,
        pub status: i32,
        pub magic_number: i32,
        pub guess: i32
    }

    impl StateMachine {
        pub fn new() -> StateMachine {
            StateMachine {
                name: "Enter your name".into(),
                player_name: "".into(),
                entry: "".into(),
                place_holder: "Enter your name".into(),
                message: "Please, give us your name".into(),
                status: 0,
                magic_number: rand::thread_rng().gen_range(0..1001),
                guess: 0,
            }
        }

        pub fn increment(&mut self) {
            if self.entry != "" {
                self.status += 1;

                self.message = format!("Guess the number between 1 and 1000 - Try #{}", self.status);

                if self.status == 1 {
                    self.player_name = self.entry.clone();
                    self.entry = "".into();
                } else {
                    self.guess = self.entry.parse::<i32>().unwrap();
                    self.guess_number();
                    self.entry = "".into();
                }
            }
        }

        fn guess_number(&mut self) {
            if self.guess == self.magic_number {
                self.message =  format!("Win!!! {} = {} on Try #{}", self.guess, self.magic_number, self.status);
            } else if self.guess > self.magic_number {
                self.name = format!("Too high");
            } else if self.guess < self.magic_number {
                self.name = format!("Too low");
            }
        }
    }
}
