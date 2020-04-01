use std::collections::HashMap;
use std::io;
use std::convert::TryInto;

fn main() {
    println!("ENTER NUMBER OF PLAYERS");
    let mut players = String::new();
    io::stdin()
        .read_line(&mut players)
        .expect("Failed to read the choice.");

    let num_players: i32 = match players.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let mut player1_score: i32 = 0;
    let mut player2_score: i32 = 0;
    let mut player3_score: i32 = 0;
    let mut player4_score: i32 = 0;

    if num_players < 5 {
        loop {
            for x in 1..num_players + 1 {
                println!("ENTER PLAYER {} WORD", x);
                let mut word = String::new();
                io::stdin()
                    .read_line(&mut word)
                    .expect("Failed to read the choice.");

                let score = calculate_word_score(word);

                if x == 1 {
                    player1_score += score;
                }
                if x == 2 {
                    player2_score += score;
                }
                if x == 3 {
                    player3_score += score;
                }
                if x == 4 {
                    player4_score += score;
                }
                println!("{:?}", score);
            }
            if num_players == 1 {
                println!("Player 1 score: {}", player1_score);
            }
            if num_players == 2 {
                println!("Player 1 score: {}", player1_score);
                println!("Player 2 score: {}", player2_score);
            }
            if num_players == 3 {
                println!("Player 1 score: {}", player1_score);
                println!("Player 2 score: {}", player2_score);
                println!("Player 3 score: {}", player3_score);
            }
            if num_players == 4 {
                println!("Player 1 score: {}", player1_score);
                println!("Player 2 score: {}", player2_score);
                println!("Player 3 score: {}", player3_score);
                println!("Player 4 score: {}", player4_score);
            }
        }
    } else {
        println!("Max number of players supported is 4.");
    }
}

fn calculate_word_score(word: String) -> i32 {
    let mut scrabble_score: HashMap<String, i32> = HashMap::new();

    scrabble_score.insert("a".to_string(), 1);
    scrabble_score.insert("b".to_string(), 3);
    scrabble_score.insert("c".to_string(), 3);
    scrabble_score.insert("d".to_string(), 2);
    scrabble_score.insert("e".to_string(), 1);
    scrabble_score.insert("f".to_string(), 4);
    scrabble_score.insert("g".to_string(), 2);
    scrabble_score.insert("h".to_string(), 4);
    scrabble_score.insert("i".to_string(), 1);
    scrabble_score.insert("j".to_string(), 8);
    scrabble_score.insert("k".to_string(), 5);
    scrabble_score.insert("l".to_string(), 1);
    scrabble_score.insert("m".to_string(), 3);
    scrabble_score.insert("n".to_string(), 1);
    scrabble_score.insert("o".to_string(), 1);
    scrabble_score.insert("p".to_string(), 3);
    scrabble_score.insert("q".to_string(), 10);
    scrabble_score.insert("r".to_string(), 1);
    scrabble_score.insert("s".to_string(), 1);
    scrabble_score.insert("t".to_string(), 1);
    scrabble_score.insert("u".to_string(), 1);
    scrabble_score.insert("v".to_string(), 4);
    scrabble_score.insert("w".to_string(), 4);
    scrabble_score.insert("x".to_string(), 8);
    scrabble_score.insert("y".to_string(), 4);
    scrabble_score.insert("z".to_string(), 10);

    scrabble_score.insert("A".to_string(), 1);
    scrabble_score.insert("B".to_string(), 3);
    scrabble_score.insert("C".to_string(), 3);
    scrabble_score.insert("D".to_string(), 2);
    scrabble_score.insert("E".to_string(), 1);
    scrabble_score.insert("F".to_string(), 4);
    scrabble_score.insert("G".to_string(), 2);
    scrabble_score.insert("H".to_string(), 4);
    scrabble_score.insert("I".to_string(), 1);
    scrabble_score.insert("J".to_string(), 8);
    scrabble_score.insert("K".to_string(), 5);
    scrabble_score.insert("L".to_string(), 1);
    scrabble_score.insert("M".to_string(), 3);
    scrabble_score.insert("N".to_string(), 1);
    scrabble_score.insert("O".to_string(), 1);
    scrabble_score.insert("P".to_string(), 3);
    scrabble_score.insert("Q".to_string(), 10);
    scrabble_score.insert("R".to_string(), 1);
    scrabble_score.insert("S".to_string(), 1);
    scrabble_score.insert("T".to_string(), 1);
    scrabble_score.insert("U".to_string(), 1);
    scrabble_score.insert("V".to_string(), 4);
    scrabble_score.insert("W".to_string(), 4);
    scrabble_score.insert("X".to_string(), 8);
    scrabble_score.insert("Y".to_string(), 4);
    scrabble_score.insert("Z".to_string(), 10);

    println!("PRESS 1 FOR DOUBLE WORD. PRESS 2 FOR TRIPLE WORD. PRESS RETURN FOR NONE.");
    let mut multiply_word = String::new();
    io::stdin()
        .read_line(&mut multiply_word)
        .expect("Failed to read the choice.");

    let multiply_word_choice: i32 = match multiply_word.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    // println!("PLEASE TELL POSITION OF DOUBLE LETTER. IF NOT, PRESS RETURN.");
    // let mut double_bonus = String::new();
    // io::stdin()
    //     .read_line(&mut double_bonus)
    //     .expect("Failed to read the choice.");

    // let double_bonus_letter: usize = match double_bonus.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => 0,
    // };

    // println!("PLEASE TELL POSITION OF TRIPLE LETTER. IF NOT, PRESS RETURN.");
    // let mut triple_bonus = String::new();
    // io::stdin()
    //     .read_line(&mut triple_bonus)
    //     .expect("Failed to read the choice.");

    // let triple_bonus_letter: usize = match triple_bonus.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => 0,
    // };

    let mut scoreboard: i32 = 0;
    for (i,l) in word.chars().enumerate() {
        let letter = l.to_string();
        let mut bonus : i32 = 0;
        let score = match scrabble_score.get(&letter) {
            Some(mut score) => {
                scoreboard += score;
            }
            None => continue,
        };
    }
    if multiply_word_choice == 1 {
        scoreboard = scoreboard*2;
    } else if multiply_word_choice == 2 {
        scoreboard = scoreboard*3;
    }
    scoreboard
}

