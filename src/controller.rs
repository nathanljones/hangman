use crate::game::GameModel;
use crate::game::LetterInWord;
use crate::game::MatchingWord;
use crate::words::generate_word;
use std::io;

pub fn play_game() {
    let mut current_game = GameModel::new();
    current_game.set_word_to_find(&generate_word());
    while current_game.get_lives_left() != 0 {
        let guess = get_current_guess();
        match current_game.check_for_letter_in_word(&guess) {
            LetterInWord::Found => {
                letter_has_been_found(&mut current_game, &guess);
                if current_game.check_word_is_found() == MatchingWord::Matched {
                    break;
                }
            }
            LetterInWord::NotFound => letter_has_not_been_found(&mut current_game, &guess),
        }
        println!("{}", current_game.get_current_word());
    }
    //let word_to_guess = current_game.get_word_to_guess();
    match current_game.check_word_is_found() {
        MatchingWord::Matched => println!(
            "Congratulations you have found the word, which was {}",
            current_game.get_word_to_guess()
        ),
        MatchingWord::NotMatched => println!(
            "You lost, the word was {}",
            current_game.get_word_to_guess()
        ),
    }
}
fn get_current_guess() -> String {
    println!("enter letter to guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess.chars().next().unwrap().to_string().to_uppercase()
}
fn letter_has_been_found(game_model: &mut GameModel, letter: &str) {
    game_model.update_current_word_with_letter(letter);
    game_model.add_letter_to_letters_used(letter);
}
fn letter_has_not_been_found(game_model: &mut GameModel, letter: &str) {
    game_model.decrement_lives_left();
    game_model.add_letter_to_letters_used(letter);
}
