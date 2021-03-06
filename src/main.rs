// Author: batmine3
// Date: 26/10/2021
// Subject: Make the famous mastermind game

struct Game {
    word: String,
    #[allow(dead_code)]
    difficulty: Difficulty,
    tries: u32,
    is_correct: bool
}

impl Game {
    fn user_try(&mut self) {
        self.tries -= 1;
    }

    fn get_tries_number(&self) -> u32 {
        return self.tries;
    }

    fn is_correct(&self) -> bool {
        return self.is_correct;
    }

    fn get_word(&self) -> &String {
        return &self.word;
    }

    fn set_correct(&mut self, state:bool) {
        self.is_correct = state;
    }
}

enum Difficulty {
    Easy,
    Medium,
    Hard
}

fn main() {
    println!("Welcome to M4ST3RM1ND in Rust");
    println!("Let launch a new game");
    println!("Authorized characters: A, B, C, D, E");
    println!("");
    println!("Correction meaning:");
    println!("O -> is correct and well placed");
    println!("X -> is correct and bad placed");
    println!("_ -> is incorrect");
    println!("");

    let difficulty = user_entry();

    let mut game = new_game(difficulty);

    loop {
        if game.is_correct() || game.get_tries_number() == 0 {
            break;
        }

        println!("Essai restant: {}", game.get_tries_number());

        let entry = read_user_entry();
        let correct = make_correction(entry, game.get_word().to_string());
        let mut pattern:String = String::from("");
        
        let mut i = 0;

        while i < game.get_word().to_string().len() {
            pattern.push('O');
            i += 1; 
        }

        if pattern == correct {
            game.set_correct(true);
        } else {
            game.user_try();
        }

        println!("{}", correct);
    }

    if game.is_correct() {
        println!("You win !!");
    } else if !game.is_correct() && game.get_tries_number() == 0 {
        println!("You lose")
    }
}

fn read_user_entry() -> String {
    use std::io;
    let mut user_entry = String::new();

    let _ = io::stdin().read_line(&mut user_entry);
    let chara:String = user_entry.trim().parse().unwrap();
    
    return chara.to_lowercase();
}

fn user_entry() -> Difficulty {
    use std::io;
    let mut user_entry = String::new();

    println!("Choose the difficulty level:");
    println!("1 - Easy");
    println!("2 - Medium");
    println!("3 - Hard");
    println!("What is you choice :");

    let _ = io::stdin().read_line(&mut user_entry);

    let number: i32 = user_entry.trim().parse().unwrap();
    
    if number == 3 {
        return Difficulty::Hard;
    } else if number == 2 {
        return Difficulty::Medium;
    } else {
        return Difficulty::Easy;
    }
}

fn make_correction(entry: String, word: String) -> String {
    
    let mut correction:String = String::from("");
    let mut found = false;

    'outter: for (eindex, eval) in entry.chars().enumerate() {
        let mut bad_place = false;
        
        for (windex, wval) in word.chars().enumerate() {
            if wval == eval && eindex == windex {
                correction.push('O');
                found = true;
                continue 'outter;
            } else if wval == eval {
                bad_place = true;
                found = true;
            }
        }

        if found == true && bad_place == true {
            correction.push('X');
        }else {
            correction.push('_');
        }
    }

    return String::from(correction);
}

fn new_game(difficulty: Difficulty) -> Game {
    let word = gen_word(&difficulty);

    Game {
        tries: 10,
        difficulty,
        word,
        is_correct: false
    }

}

fn gen_word(difficulty:&Difficulty) -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let mut word:String = String::from("");
    let max:usize;

    match difficulty {
        Difficulty::Easy => {
            max = 5;
        },
        Difficulty::Medium => {
            max = 7
        },
        Difficulty::Hard => {
            max = 10;
        }
    }

    while word.len() < max {
        let number = rng.gen_range(0..5);
        word.push(gen_letter(number));
    }

    return word;
}

fn gen_letter(number: usize) -> char {
    let letter = ['a', 'b', 'c', 'd', 'e'];
    return letter[number];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_easy_world() {
        let diff = Difficulty::Easy;
        assert_eq!(gen_word(&diff).len(), 5);
    }
    
    #[test]
    fn test_gen_medium_world() {
        let diff = Difficulty::Medium;
        assert_eq!(gen_word(&diff).len(), 7);
    }

    #[test]
    fn test_gen_hard_world() {
        let diff = Difficulty::Hard;
        assert_eq!(gen_word(&diff).len(), 10);
    }

    #[test]
    fn test_return_gen_letter() {
        assert_eq!(gen_letter(0), 'a');
        assert_eq!(gen_letter(1), 'b');
        assert_eq!(gen_letter(2), 'c');
        assert_eq!(gen_letter(3), 'd');
        assert_eq!(gen_letter(4), 'e');
    }

    #[test]
    fn test_make_correction() {
        let word = String::from("abc");
        let mut entry = String::from("abc");

        assert_eq!(make_correction(entry.clone(), word.clone()), "OOO");
        
        entry = String::from("acb");
        assert_eq!(make_correction(entry.clone(), word.clone()), "OXX");
        
        entry = String::from("adb");
        assert_eq!(make_correction(entry.clone(), word.clone()), "O_X");
        
        entry = String::from("adc");
        assert_eq!(make_correction(entry.clone(), word.clone()), "O_O");
    }
}