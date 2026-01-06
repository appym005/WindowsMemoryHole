use std::path::Path;

use parking_lot::Mutex;
use rusqlite::{params, Connection};

use crate::errors::{AppError, Result};
use crate::models::Item;

pub struct Database {
    conn: Mutex<Connection>,
}

unsafe impl Send for Database {}
unsafe impl Sync for Database {}

impl Database {
    pub fn new(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.init()?;
        Ok(db)
    }

    fn init(&self) -> Result<()> {
        let conn = self.conn.lock();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS items (
                id TEXT PRIMARY KEY,
                created_at INTEGER NOT NULL,
                item_type TEXT NOT NULL,
                title TEXT,
                payload TEXT NOT NULL,
                thumb_path TEXT,
                pinned INTEGER NOT NULL DEFAULT 0
            );
            CREATE INDEX IF NOT EXISTS idx_items_created_at ON items(created_at DESC);
            CREATE INDEX IF NOT EXISTS idx_items_pinned ON items(pinned DESC);",
        )?;
        Ok(())
    }

    pub fn insert_item(&self, item: &Item) -> Result<()> {
        let conn = self.conn.lock();
        conn.execute(
            "INSERT INTO items (id, created_at, item_type, title, payload, thumb_path, pinned)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                item.id,
                item.created_at,
                item.item_type,
                item.title,
                item.payload,
                item.thumb_path,
                item.pinned
            ],
        )?;
        Ok(())
    }

    pub fn list_items(&self, limit: i64, include_pinned: bool) -> Result<Vec<Item>> {
        let conn = self.conn.lock();
        let mut statement = if include_pinned {
            conn.prepare(
                "SELECT id, created_at, item_type, title, payload, thumb_path, pinned
                 FROM items
                 ORDER BY pinned DESC, created_at DESC
                 LIMIT ?1",
            )?
        } else {
            conn.prepare(
                "SELECT id, created_at, item_type, title, payload, thumb_path, pinned
                 FROM items
                 WHERE pinned = 0
                 ORDER BY created_at DESC
                 LIMIT ?1",
            )?
        };

        let rows = statement.query_map([limit], |row| {
            Ok(Item {
                id: row.get(0)?,
                created_at: row.get(1)?,
                item_type: row.get(2)?,
                title: row.get(3)?,
                payload: row.get(4)?,
                thumb_path: row.get(5)?,
                pinned: row.get(6)?,
            })
        })?;

        Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
    }

    pub fn get_item(&self, id: &str) -> Result<Item> {
        let conn = self.conn.lock();
        let mut statement = conn.prepare(
            "SELECT id, created_at, item_type, title, payload, thumb_path, pinned
             FROM items WHERE id = ?1",
        )?;
        let item = statement
            .query_row([id], |row| {
                Ok(Item {
                    id: row.get(0)?,
                    created_at: row.get(1)?,
                    item_type: row.get(2)?,
                    title: row.get(3)?,
                    payload: row.get(4)?,
                    thumb_path: row.get(5)?,
                    pinned: row.get(6)?,
                })
            })
            .map_err(|err| match err {
                rusqlite::Error::QueryReturnedNoRows => AppError::NotFound,
                other => AppError::Db(other),
            })?;
        Ok(item)
    }

    pub fn delete_item(&self, id: &str) -> Result<Item> {
        let item = self.get_item(id)?;
        let conn = self.conn.lock();
        conn.execute("DELETE FROM items WHERE id = ?1", [id])?;
        Ok(item)
    }
}
