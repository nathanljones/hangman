#[allow(dead_code)]
struct GameStore {
    word_to_find: String,
    lives_left: i32,
    letters_used: Vec<String>,
    current_word: String,
}

#[derive(PartialEq, Debug)]
enum LetterInWord {
    Found,
    NotFound,
}
#[derive(PartialEq, Debug)]
enum MatchingWord {
    Matched,
    NotMatched,
}
impl GameStore {
    fn new() -> Self {
        GameStore {
            word_to_find: String::from(""),
            lives_left: 0,
            letters_used: vec![],
            current_word: String::from(""),
        }
    }
    fn set_word_to_find(&mut self, secret_word: &str) {
        self.word_to_find = secret_word.to_owned();
        self.current_word = self.word_to_find.chars().map(|_c| "_").collect();
    }
    fn check_for_letter_in_word(self, letter: &str) -> LetterInWord {
        match self.word_to_find.find(letter) {
            Some(_) => LetterInWord::Found,
            None => LetterInWord::NotFound,
        }
    }
    fn add_letter_to_letters_used(&mut self, letter_to_add: &str) {
        if !self.letters_used.iter().any(|i| i == "letter_to_add") {
            self.letters_used.push(letter_to_add.to_owned());
        };
    }
    fn decrement_lives_left(&mut self) {
        self.lives_left -= 1;
    }
    fn check_word_is_found(self) -> MatchingWord {
        if self.word_to_find == self.current_word {
            MatchingWord::Matched
        } else {
            MatchingWord::NotMatched
        }
    }
    fn update_current_word_with_letter(&mut self, letter_to_update: &str) {
        for (i, c) in self.word_to_find.chars().enumerate() {
            if c.to_string() == letter_to_update {
                self.current_word.replace_range(i..i + 1, letter_to_update)
            }
        }
    }
}
#[allow(dead_code)]
const NO_OF_LIVES: i32 = 5;

fn generate_word_to_guess() -> String {
    String::from("secret")
}

pub fn game_run() {
    let mut current_game_store = GameStore::new();
    current_game_store.word_to_find = generate_word_to_guess();
}

#[test]
fn secret_word_masks() {
    let mut current_game_store = GameStore::new();
    current_game_store.set_word_to_find("secret");
    assert_eq!(current_game_store.current_word, "______");
}

#[test]
fn letter_is_in_word() {
    let mut current_game_store = GameStore::new();
    current_game_store.set_word_to_find("secret");
    assert_eq!(
        current_game_store.check_for_letter_in_word("s"),
        LetterInWord::Found,
    );
}
#[test]
fn letter_is_not_in_word() {
    let mut current_game_store = GameStore::new();
    current_game_store.set_word_to_find("secret");
    assert_eq!(
        current_game_store.check_for_letter_in_word("q"),
        LetterInWord::NotFound,
    );
}
#[test]
fn letter_added_to_letters_used() {
    let mut current_game_store = GameStore::new();
    current_game_store.add_letter_to_letters_used("f");
    assert!(current_game_store.letters_used.iter().any(|i| i == "f"));
}
#[test]
fn lives_can_be_lost() {
    let mut current_game_store = GameStore::new();
    current_game_store.lives_left = 30;
    current_game_store.decrement_lives_left();
    assert_eq!(current_game_store.lives_left, 29);
}
#[test]
fn matching_word_is_found() {
    let mut current_game_store = GameStore::new();
    current_game_store.set_word_to_find("secret");
    current_game_store.current_word = "secret".to_string();
    assert_eq!(
        current_game_store.check_word_is_found(),
        MatchingWord::Matched
    );
}
#[test]
fn matching_word_is_not_found() {
    let mut current_game_store = GameStore::new();
    current_game_store.set_word_to_find("secret");
    current_game_store.current_word = "fred".to_string();
    assert_eq!(
        current_game_store.check_word_is_found(),
        MatchingWord::NotMatched
    );
}

#[test]
fn current_word_updates_with_letter() {
    let mut current_game_store = GameStore::new();
    current_game_store.set_word_to_find("secret");
    current_game_store.update_current_word_with_letter("e");
    assert_eq!(current_game_store.current_word, "_e__e_");
}
