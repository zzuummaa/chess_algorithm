use chrono::{DateTime, Utc};
use rusqlite::{Connection, Error, params, Result};

pub const DEFAULT_PATH: &str = "chess_game.db";

#[derive(Debug)]
pub struct Game {
    pub id: i32,
    pub start_time: DateTime<Utc>
}

impl Game {
    pub fn now() -> Game {
        Game { id: 0, start_time: Utc::now() }
    }
}

#[derive(Debug)]
pub struct MoveRecord {
    pub game_id: i32,
    pub move_number: i32,
    pub p_from: String,
    pub p_to: String,
}

pub struct DataBaseInstance {
    connection: Connection
}

impl DataBaseInstance {
    pub fn new(path: &str) -> Result<Self> {
        Ok(DataBaseInstance { connection: Connection::open(path)? })
    }

    pub fn create_tables(&self) -> Result<(), Error> {
        self.connection.execute(
        "CREATE TABLE IF NOT EXISTS game (
                id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                start_time DATETIME NOT NULL
            )", params![],
        )?;

        self.connection.execute(
        "CREATE TABLE IF NOT EXISTS move_record (
                game_id INTEGER,
                move_number INTEGER,
                p_from CHAR(4),
                p_to CHAR(4),
                FOREIGN KEY (game_id) REFERENCES game
                ON DELETE CASCADE
            )", params![],
        )?;
        Ok(())
    }

    pub fn add_game(&mut self, game: Game) -> Result<Game, Error> {
        let transaction = self.connection.transaction()?;
        transaction.execute(
            "INSERT INTO game (start_time) VALUES (?1)",
            params![game.start_time],
        )?;

        let mut game = game;
        game.id = transaction.last_insert_rowid() as i32;

        transaction.commit()?;
        return Ok(game);
    }

    pub fn add_move(&self, record: &MoveRecord) -> Result<(), Error>{
        self.connection.execute(
            "INSERT INTO move_record (game_id, move_number, p_from, p_to) VALUES (?1, ?2, ?3, ?4)",
            params![record.game_id, record.move_number, record.p_from, record.p_to],
        )?;
        return Ok(());
    }
}

impl Default for DataBaseInstance {
    fn default() -> Self {
        let instance = DataBaseInstance::new(DEFAULT_PATH).unwrap();
        instance.create_tables().unwrap();
        return instance;
    }
}