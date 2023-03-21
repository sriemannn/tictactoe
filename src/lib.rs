pub mod game {

    use ndarray::prelude::*;
    use std::io::{Error, ErrorKind};

    pub struct Game {
        fields: Array2<i32>,
        pub won: bool,
        pub tie: bool,
        pub player_one_start: bool,
    }

    impl Game {
        pub fn new(player_one_start: bool) -> Game {
            Game {
                fields: Array2::zeros((3, 3)),
                won: false,
                tie: false,
                player_one_start,
            }
        }

        pub fn choose_field(
            &mut self,
            x: usize,
            y: usize,
            player_marker: &i32,
        ) -> Result<(), Error> {
            let marker = match player_marker {
                1 => 1,
                _ => -1,
            };
            if self.fields[[x, y]] == 0 {
                Ok(self.fields[[x, y]] = marker)
            } else {
                Err(Error::new(ErrorKind::Other, "Field already played"))
            }
        }

        pub fn check_winner(&mut self, player_marker: &i32) {
            for i in 0..=1 {
                for j in self.fields.sum_axis(Axis(i)).iter() {
                    if (j * player_marker).abs() == 3 {
                        self.won = true;
                    }
                }
            }

            let check_reverse_diag = array![[0, 0, 1], [0, 1, 0], [1, 0, 0]];

            if self.fields.diag().sum().abs() == 3 {
                self.won = true;
            } else if self.fields.dot(&check_reverse_diag).diag().sum().abs() == 3 {
                self.won = true;
            }
            if self
                .fields
                .mapv(|x| {
                    let y = if x < 0 { x * -1 } else { x };
                    u32::try_from(y).unwrap()
                })
                .sum()
                == 9
            {
                self.tie = true;
            }
        }

        pub fn int_converter(integer: i32) -> String {
            if integer == 1 {
                String::from("x")
            } else if integer == -1 {
                String::from("o")
            } else {
                String::from(" ")
            }
        }

        pub fn terminal_out(&self) {
            println!(" |0|1|2|");
            println!("--------");
            println!(
                "0|{}|{}|{}|",
                Game::int_converter(self.fields[[0, 0]]),
                Game::int_converter(self.fields[[0, 1]]),
                Game::int_converter(self.fields[[0, 2]])
            );
            println!("--------");
            println!(
                "1|{}|{}|{}|",
                Game::int_converter(self.fields[[1, 0]]),
                Game::int_converter(self.fields[[1, 1]]),
                Game::int_converter(self.fields[[1, 2]])
            );
            println!("--------");
            println!(
                "2|{}|{}|{}|",
                Game::int_converter(self.fields[[2, 0]]),
                Game::int_converter(self.fields[[2, 1]]),
                Game::int_converter(self.fields[[2, 2]])
            );
            println!("--------\n");
        }

        pub fn terminal_game(player_one_start: bool) {
            let mut game = Game::new(player_one_start);

            let player_one = Player::new(String::from("Player 1"), 1);

            let player_two = Player::new(String::from("Player 2"), -1);

            let players = [player_one, player_two];

            let mut player_idx = if player_one_start { 0 } else { 1 };

            game.terminal_out();

            loop {
                let mut field = String::new();

                println!(
                    "{} ({}):\nWrite your chosen field in the form 'row,column'!\n",
                    players[player_idx].name,
                    Game::int_converter(players[player_idx].marker)
                );

                std::io::stdin()
                    .read_line(&mut field)
                    .expect("Cannot read input");

                let mut split = field.trim().split(",");

                let x_value = match split.next() {
                    Some(num) => num,
                    _ => {
                        println!("Expected input in the form 'x,y'");
                        continue;
                    }
                };

                let x = match x_value.trim().parse::<i32>() {
                    Ok(0) => 0,
                    Ok(1) => 1,
                    Ok(2) => 2,
                    _ => {
                        println!("Integer between 0 and 2 expected");
                        continue;
                    }
                };

                let y_value = match split.next() {
                    Some(num) => num,
                    _ => {
                        println!("Expected input in the form of 'x,y'");
                        continue;
                    }
                };

                let y = match y_value.trim().parse::<i32>() {
                    Ok(0) => 0,
                    Ok(1) => 1,
                    Ok(2) => 2,
                    _ => {
                        println!("Integer between 0 and 2 expected");
                        continue;
                    }
                };

                match game.choose_field(x, y, &players[player_idx].marker) {
                    Err(e) => {
                        println!("{}", e);
                        game.terminal_out();
                        continue;
                    }
                    _ => (),
                };

                game.terminal_out();

                game.check_winner(&players[player_idx].marker);

                if game.won == true {
                    println!("{} won!", players[player_idx].name);
                    return;
                }

                if game.tie == true {
                    println!("Tie");
                    return;
                }

                player_idx = if players[player_idx].name == "Player 1" {
                    1
                } else {
                    0
                };
            }
        }
    }

    pub struct Player {
        pub name: String,
        pub marker: i32,
    }

    impl Player {
        pub fn new(name: String, marker: i32) -> Player {
            Player { name, marker }
        }
    }
}
