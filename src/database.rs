use chrono::{DateTime, Utc};
use rusqlite::{Connection, Error, params, Result};

use crate::movement::Move;
use crate::point::Point;
use std::fmt;

pub const DEFAULT_PATH: &str = "chess_game.db";

#[derive(Debug)]
pub struct Game {
    pub id: i32,
    pub start_time: DateTime<Utc>,
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
    pub m_type: String,
}

impl MoveRecord {
    pub fn new(game: &Game) -> MoveRecord {
        MoveRecord {
            game_id: game.id,
            move_number: -1,
            p_from: String::new(),
            p_to: String::new(),
            m_type: String::new(),
        }
    }

    pub fn to_next(&self, movement: &Move) -> MoveRecord {
        MoveRecord {
            game_id: self.game_id,
            move_number: self.move_number + 1,
            p_from: movement.from.to_string(),
            p_to: movement.to.to_string(),
            m_type: movement.m_type.to_string()
        }
    }

    pub fn to_move(&self) -> Result<Move, fmt::Error> {
        let mut m = Move::default();
        m.from = Point::from_string(&self.p_from)?;
        m.to = Point::from_string(&self.p_to)?;
        Ok(m)
    }
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
                type CHAR(10),
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

    pub fn find_game(&mut self, game_id: i32) -> Result<Game, Error> {
        let mut stmt = self.connection.prepare(
            "SELECT id, start_time FROM game WHERE id = ?",
        )?;

        stmt.query_row(params![game_id], |row| {
            Ok(Game {
                id: row.get(0)?,
                start_time: row.get(1)?,
            })
        })
    }

    pub fn add_move(&self, record: &MoveRecord) -> Result<(), Error> {
        if record.move_number < 0 { return Err(Error::InvalidQuery) }
        self.connection.execute(
            "INSERT INTO move_record (game_id, move_number, p_from, p_to, type) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![record.game_id, record.move_number, record.p_from, record.p_to, record.m_type.to_string()],
        )?;
        return Ok(());
    }

    pub fn find_moves_by_game_id(&self, game_id: i32) -> Result<Vec<MoveRecord>, Error> {
        let mut stmt = self.connection.prepare(
            "SELECT game_id, move_number, p_from, p_to, type FROM move_record WHERE game_id = ?",
        )?;

        let mapped_rows = stmt.query_map(params![game_id], |row| {
            Ok(MoveRecord {
                game_id: row.get(0)?,
                move_number: row.get(1)?,
                p_from: row.get(2)?,
                p_to: row.get(3)?,
                m_type: row.get(4)?,
            })
        })?;

        mapped_rows.into_iter().collect::<Result<Vec<_>>>()
    }

    pub fn find_moves(&self, game: &Game) -> Result<Vec<MoveRecord>, Error> {
        self.find_moves_by_game_id(game.id)
    }
}

impl Default for DataBaseInstance {
    fn default() -> Self {
        let instance = DataBaseInstance::new(DEFAULT_PATH).unwrap();
        instance.create_tables().unwrap();
        return instance;
    }
}