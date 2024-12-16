use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SqlUserCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SqlUser {
    pub username: String,
    pub is_admin: bool,
    pub created_at: String,
}

impl SqlUser {
    pub fn from_row(row: &oracle::Row) -> Result<Self, oracle::Error> {
        Ok(Self {
            username: row.get(0)?,
            is_admin: match row.get(1)? {
                1 => true,
                _ => false,
            },
            created_at: row.get(2)?,
        })
    }
}