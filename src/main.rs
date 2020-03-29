use std::io;
use std::collections::HashMap;

fn main () {
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

    let mut scoreboard_1: i32 = 0;
    let mut scoreboard_2: i32 = 0;

    loop {
        println! ("ENTER PLAYER 1 WORD");
        let mut word_1 = String::new();
        io::stdin().read_line(&mut word_1).expect("Failed to read the choice.");

        let mut score_1 : i32;

        for letter_1 in word_1.chars() {
            let l_1 = letter_1.to_string();
            let score_1 = match scrabble_score.get(&l_1) {
                Some(score_1) => {
                    println!("{:?} : {:?}", l_1, score_1);
                    scoreboard_1 += score_1;
                },
                None => println!("{:?} is not mentioned.", l_1)
            };
        }
        println! ("PLAYER SCORE 1 = {}", scoreboard_1);

        println! ("ENTER PLAYER 2 WORD");
        let mut word_2= String::new();
        io::stdin().read_line(&mut word_2).expect("Failed to read the choice.");

        let mut score_2 : i32;

        for letter_2 in word_2.chars() {
            let l_2 = letter_2.to_string();
            let score_2 = match scrabble_score.get(&l_2) {
                Some(score_2) => {
                    println!("{:?} : {:?}", l_2, score_2);
                    scoreboard_2 += score_2;
                },
                None => println!("{:?} is not mentioned.", l_2)
            };
        }
        println! ("PLAYER SCORE 2 = {}", scoreboard_2);
    }
}
