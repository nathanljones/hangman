#[allow(dead_code)]
struct GameStore {
    word_to_find: String,
    lives_left: i32,
    letters_used: Vec<String>,
    current_word: String,
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
