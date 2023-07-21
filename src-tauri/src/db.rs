use rusqlite::{params, Connection, Error, Result};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Doc {
    pub id: Option<i32>,
    pub name: String,
    pub sex: String,
    pub birthday: String,
    pub candidate_type: String,
    pub identity_num: String,
    pub company_name: String,
    pub company_type: String,
    pub contract_time: String,
    pub create_time: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub user_name: String,
    pub user_pwd: String,
    pub create_time: Option<u64>,
}

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new(file_path: impl AsRef<std::path::Path>) -> Result<Database, Error> {
        let conn = Connection::open(file_path)?;
        Database::init_database(&conn)?;
        Ok(Database { conn })
    }

    pub fn add_doc(&mut self, doc: &Doc) -> Result<(), Error> {
        let current_time = SystemTime::now();
        let timestamp = current_time
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get timestamp")
            .as_secs();
        let tx = self.conn.transaction()?;
        // let id: i32 = tx.last_insert_rowid() as i32;
        tx.execute("INSERT INTO docs (name,sex,birthday,candidate_type,identity_num,company_name,company_type,contract_time,create_time) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)", 
        params![doc.name,doc.sex,doc.birthday,doc.candidate_type,doc.identity_num,doc.company_name,doc.company_type,doc.contract_time,timestamp])?;
        tx.commit()?;
        Ok(())
    }

    pub fn add_user(&mut self, user: &User) -> Result<(), Error> {
        let current_time = SystemTime::now();
        let timestamp = current_time
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get timestamp")
            .as_secs();
        let tx = self.conn.transaction()?;
        // let id = tx.last_insert_rowid() as i32;
        tx.execute(
            "INSERT INTO users (user_name,user_pwd,create_time) VALUES (?, ?, ?)",
            params![user.user_name, user.user_pwd, timestamp],
        )?;
        tx.commit()?;
        Ok(())
    }

    pub fn delete_doc(&mut self, id: i32) -> Result<(), Error> {
        let tx = self.conn.transaction()?;
        tx.execute("DELETE FROM docs WHERE id=?", [id])?;
        tx.commit()?;
        Ok(())
    }

    pub fn delete_user(&mut self, id: i32) -> Result<(), Error> {
        let tx = self.conn.transaction()?;
        tx.execute("DELETE FROM users WHERE id=?", [id])?;
        tx.commit()?;
        Ok(())
    }

    pub fn update_doc(&mut self, id: i32, name: &str) -> Result<(), Error> {
        let tx = self.conn.transaction()?;
        tx.execute("UPDATE docs SET name=? WHERE id=?", params![name, id])?;
        tx.commit()?;
        Ok(())
    }

    pub fn update_user(&mut self, id: i32, user_name: &str) -> Result<(), Error> {
        let tx = self.conn.transaction()?;
        tx.execute(
            "UPDATE users SET user_name=? WHERE id=?",
            params![user_name, id],
        )?;
        tx.commit()?;
        Ok(())
    }

    pub fn get_all_docs(&mut self) -> Result<Vec<Doc>, Error> {
        let mut stmt = self.conn.prepare("SELECT id, name, sex, birthday, candidate_type, identity_num, company_name, company_type, contract_time, create_time FROM docs ORDER BY id")?;
        let doc_iter = stmt.query_map([], |row| {
            Ok(Doc {
                id: row.get(0)?,
                name: row.get(1)?,
                sex: row.get(2)?,
                birthday: row.get(3)?,
                candidate_type: row.get(4)?,
                identity_num: row.get(5)?,
                company_name: row.get(6)?,
                company_type: row.get(7)?,
                contract_time: row.get(8)?,
                create_time: row.get(9)?,
            })
        })?;
        let mut docs = Vec::new();
        for doc_result in doc_iter {
            docs.push(doc_result?);
        }
        Ok(docs)
    }

    pub fn get_docs_by_stmt(&mut self, condition: &str) -> Result<Vec<Doc>, Error> {
        let query = format!("SELECT id, name, sex, birthday, candidate_type, identity_num, company_name, company_type, contract_time, create_time FROM docs WHERE {} ORDER BY id", condition);
        let mut stmt = self.conn.prepare(&query)?;
        let doc_iter = stmt.query_map([], |row| {
            Ok(Doc {
                id: row.get(0)?,
                name: row.get(1)?,
                sex: row.get(2)?,
                birthday: row.get(3)?,
                candidate_type: row.get(4)?,
                identity_num: row.get(5)?,
                company_name: row.get(6)?,
                company_type: row.get(7)?,
                contract_time: row.get(8)?,
                create_time: row.get(9)?,
            })
        })?;
        let mut docs = Vec::new();
        for doc_result in doc_iter {
            docs.push(doc_result?);
        }
        Ok(docs)
    }

    pub fn get_all_users(&mut self) -> Result<Vec<User>, Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, user_name, user_pwd, create_time FROM users ORDER BY id")?;
        let user_iter = stmt.query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                user_name: row.get(1)?,
                user_pwd: row.get(2)?,
                create_time: row.get(3)?,
            })
        })?;
        let mut users = Vec::new();
        for user_result in user_iter {
            users.push(user_result?);
        }
        Ok(users)
    }

    pub fn get_user_by_name(&mut self, name: String) -> Result<Vec<User>, Error> {
        let query = format!(
            "SELECT id, user_name, user_pwd, create_time FROM users WHERE {} ORDER BY id",
            name
        );
        let mut stmt = self.conn.prepare(&query)?;
        let user_iter = stmt.query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                user_name: row.get(1)?,
                user_pwd: row.get(2)?,
                create_time: row.get(3)?,
            })
        })?;
        let mut users = Vec::new();
        for user_result in user_iter {
            users.push(user_result?);
        }
        println!("users: {:?}", users);
        Ok(users)
    }

    fn init_database(conn: &Connection) -> Result<(), Error> {
        // conn.execute("DROP TABLE docs;", [])?;
        // conn.execute("DROP TABLE users;", [])?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS docs (id INTEGER PRIMARY KEY AUTOINCREMENT,name TEXT,sex TEXT,birthday TEXT,candidate_type TEXT,identity_num TEXT,company_name TEXT,company_type TEXT,contract_time TEXT,create_time INTEGER)",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT,user_name TEXT,user_pwd TEXT,create_time INTEGER)",
            [],
        )?;
        // conn.execute("CREATE INDEX IF NOT EXISTS docs_idx ON docs (doc_id)", [])?;
        // conn.execute(
        //     "CREATE INDEX IF NOT EXISTS users_idx ON users (user_id)",
        //     [],
        // )?;
        Ok(())
    }
}
