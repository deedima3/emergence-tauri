use std::sync::Mutex;

use rusqlite::{named_params, params, Connection, OptionalExtension, Result};

struct Config {
    key: String,
    data: Option<String>,
}

pub struct ConfigStore {
    pub db: Mutex<Option<Connection>>,
}

impl ConfigStore {
    pub(crate) fn get(&self, key: String) -> Result<Option<String>, rusqlite::Error> {
        let db = &mut *self.db.lock().unwrap();
        let conn = match db {
            Some(v) => v,
            None => panic!("cannot connect to config store"),
        };

        let val: Option<String> = conn
            .query_row(
                "select data from em_config where key = :key",
                &[(":key", &key)],
                |r| r.get(0),
            )
            .optional()?;

        Ok(val)
    }

    pub(crate) fn save(&self, key: String, val: Option<String>) -> Result<(), rusqlite::Error> {
        let db = &mut *self.db.lock().unwrap();
        let conn = match db {
            Some(v) => v,
            None => panic!("cannot connect to config store"),
        };

        let exists: u32 = conn.query_row(
            "select exists(select 1 from em_config where key = :key)",
            &[(":key", &key)],
            |r| r.get(0),
        )?;

        if exists == 0 {
            let mut stmt = conn.prepare("insert into em_config(key, data) values(:key, :value)")?;
            stmt.execute(named_params! {
                ":key": key,
                ":value": val,
            })?;
        } else {
            let mut stmt = conn.prepare("update em_config set data = :value where key = :key")?;
            stmt.execute(named_params! {
                ":key": key,
                ":value": val,
            })?;
        }

        Ok(())
    }
}
