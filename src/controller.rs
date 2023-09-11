use crate::game::{LetterInWord, MatchingWord, Model};
use crate::view::{
    get_current_guess, output_congratulations, output_current_word, output_lost_message,
};
use crate::words::generate_word;

pub fn play_game() {
    let mut current_game = Model::new();

    current_game.set_word_to_find(&generate_word());
    while current_game.lives_left() != 0 {
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
        output_current_word(&current_game.current_word());
    }
    //let word_to_guess = current_game.get_word_to_guess();
    match current_game.check_word_is_found() {
        MatchingWord::Matched => output_congratulations(&current_game.word_to_guess()),
        MatchingWord::NotMatched => output_lost_message(&current_game.word_to_guess()),
    }
}

fn letter_has_been_found(game_model: &mut Model, letter: &str) {
    game_model.update_current_word_with_letter(letter);
    game_model.add_letter_to_letters_used(letter);
}
fn letter_has_not_been_found(game_model: &mut Model, letter: &str) {
    game_model.decrement_lives_left();
    game_model.add_letter_to_letters_used(letter);
}
