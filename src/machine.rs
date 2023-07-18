pub mod machine {
    use std::io::BufWriter;
    use std::io::Write;

    use druid::{Data, Lens};
    use rand::Rng;

    use std::fs::File;

    #[derive(Clone, Data, Lens)]
    pub struct StateMachine {
        pub name: String,
        pub player_name: String,
        pub entry: String,
        pub place_holder: String,
        pub message: String,
        pub status: i32,
        pub magic_number: i32,
        pub guess: i32,
        pub best_guessing: i32,
        pub points: i32,
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
                best_guessing: 0,
                points: 0,
            }
        }

        pub fn increment(&mut self) {
            if self.entry != "" {
                self.status += 1;

                self.message = format!("Guess the number between 1 and 1000 - Try #{}/10", self.status);

                self.recalculate_best_guessing();

                if self.status == 1 {
                    self.player_name = self.entry.clone();
                    self.entry = "".into();
                    self.name = format!("{}'s turn", self.player_name);
                } else if self.status < 10 {
                    self.guess = self.entry.parse::<i32>().unwrap();
                    self.guess_number();
                    self.entry = "".into();
                } else {
                    self.guess = self.entry.parse::<i32>().unwrap();
                    self.guess_number();
                    self.game_over();
                }
            }
        }

        fn guess_number(&mut self) {
            if self.guess == self.magic_number {
                self.name = format!("Win");
                self.game_over();
            } else if self.guess > self.magic_number {
                self.name = format!("Too high");
            } else if self.guess < self.magic_number {
                self.name = format!("Too low");
            }
        }

        fn game_over(&mut self) {
            if self.guess == self.magic_number {
                self.points = 1000 + (1000 - ((self.status - 1) * 100));
                self.message =  format!("Win!!! {} = {} on Try #{}. You did {} points!", self.guess, self.magic_number, self.status, self.points);
            } else {
                let guessing_distance = (self.best_guessing - self.magic_number).abs();
                self.points = 1000 - guessing_distance;
                self.message = format!("Game Over!!! Better Guessing {} => Magic Number {} on Try #{}. You did {} points!", self.best_guessing, self.magic_number, self.status, self.points);
            }

            self.save_score().expect("Failed to save score");

            self.name = format!("Game Over");
            self.entry = "".into();
            self.status = 0;
            self.magic_number = rand::thread_rng().gen_range(0..1001);
        }

        fn recalculate_best_guessing(&mut self) {
            if (self.best_guessing - self.magic_number).abs() >= (self.guess - self.magic_number).abs() {
                self.best_guessing = self.guess;
            }
        }

        fn save_score(&mut self) -> std::io::Result<()> {
            let mut scores = Self::load_score().unwrap_or_default();
            scores.push((format!("{}", self.player_name), self.points));
            scores = self.order_scores(&mut scores);
            let file = File::create("scores.txt")?;
            let mut writer = BufWriter::new(file);
            serde_json::to_writer(&mut writer, &scores)?;
            writer.flush()?;
            Ok(())
        }

        pub fn load_score() -> std::io::Result<Vec<(String, i32)>> {
            let file = File::open("scores.txt")?;
            let scores: Vec<(String, i32)> = serde_json::from_reader(file)?;
            Ok(scores)
        }

        //order_scores(&mut self) recibe a vector of tuplas, order desc by i32 and return top 10 on a vector of tuplas
        pub fn order_scores(&mut self, scores: &mut Vec<(String, i32)>) -> Vec<(String, i32)> {
            scores.sort_by(|a, b| b.1.cmp(&a.1));
            scores.truncate(10);
            scores.to_vec()
        }


    }
}
