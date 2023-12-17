use std::fs;

use rusqlite::Connection;
use tauri::AppHandle;

use crate::dto::{DIR_ENC, DIR_THUMBNAILS};

const CURR_DB_VER: u32 = 2;

pub fn init_db(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("appdata dir should be exists");
    fs::create_dir_all(&app_dir).expect("appdata dir should be created");
    let sqlite_path = app_dir.join("emdata.db");

    fs::create_dir_all(app_dir.join(DIR_ENC)).expect("encrypted dir should be created");
    fs::create_dir_all(app_dir.join(DIR_THUMBNAILS)).expect("thumbnails dir should be created");

    let mut db = Connection::open(sqlite_path)?;

    let mut user_pragma = db.prepare("PRAGMA user_version")?;
    let exist_user_ver: u32 = match user_pragma.query_row([], |row| row.get(0)) {
        Ok(v) => v,
        Err(e) => match e {
            rusqlite::Error::QueryReturnedNoRows => 0,
            _ => return Err(e),
        },
    };
    drop(user_pragma);

    migrate_db(&mut db, exist_user_ver)?;

    Ok(db)
}

pub fn migrate_db(db: &mut Connection, exist_ver: u32) -> Result<(), rusqlite::Error> {
    if exist_ver < CURR_DB_VER {
        db.pragma_update(None, "journal_mode", "WAL")?;

        let tx = db.transaction()?;
        tx.pragma_update(None, "user_version", CURR_DB_VER)?;

        if exist_ver < 1 {
            tx.execute_batch(
                "
                    create table em_config (
                        key text not null,
                        data text
                    );
    
                    create table em_folders (
                        id integer primary key autoincrement,
                        name text not null default ''
                    );
    
                    create table em_data_dir (
                        id integer primary key autoincrement,
                        folder_id integer not null default '',
                        name text not null default ''
                    );
                ",
            )?;
        }
        
        if exist_ver < 2 {
            tx.execute_batch("
                alter table em_data_dir add column encrypted_at datetime;
                alter table em_data_dir add column accessed_at datetime;
                alter table em_data_dir add column file_uid text;
                alter table em_data_dir add column file_ext text;
                ",
            )?;
        }

        tx.commit()?;
    }

    Ok(())
}
