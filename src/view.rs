use std::io;
pub fn get_current_guess() -> String {
    println!("enter letter to guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess.chars().next().unwrap().to_string().to_uppercase()
}
pub fn output_congratulations(guessed_word: &str) {
    println!("Congratulations you have found the word, which was {guessed_word}");
}
pub fn output_lost_message(guessed_word: &str) {
    println!("You lost, the word was {guessed_word}");
}
pub fn output_current_word(current_word: &str) {
    println!("{current_word}");
}
