use chess_algorithm::database::{DataBaseInstance, Game, DEFAULT_PATH, MoveRecord};

#[test]
fn test_open_db() {
    match DataBaseInstance::new(DEFAULT_PATH) {
        Err(e) => {
            assert!(false, "{}", e)
        }
        Ok(_) => {}
    }
}

#[test]
fn test_create_tables() {
    let instance = DataBaseInstance::default();
    assert_eq!(instance.create_tables(), Ok(()));
}

#[test]
fn test_add_game() {
    let game = Game::now();
    let mut instance = DataBaseInstance::default();
    match instance.add_game(game) {
        Err(e) => {
            assert!(false, "{}", e)
        }
        Ok(_) => {}
    }
}

#[test]
fn test_add_move_record() {
    let move_record = MoveRecord {
        game_id: 1,
        move_number: 0,
        p_from: "c1".to_string(),
        p_to: "c2".to_string()
    };
    let instance = DataBaseInstance::default();
    match instance.add_move(&move_record) {
        Err(e) => {
            assert!(false, "{}", e)
        }
        Ok(_) => {}
    }
}