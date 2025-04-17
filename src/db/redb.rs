use redb::{Database, TableDefinition, Error as RedbError};

pub fn open_database(path: &str) -> Result<Database, RedbError> {
    Database::create(path)
}

// Example: Storing draft content
pub fn save_draft(db: &Database, user_id: &str, content: &str) -> Result<(), RedbError> {
    let drafts_table = TableDefinition::<&str, &str>::new("drafts");
    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(drafts_table)?;
        table.insert(user_id, content)?;
    }
    write_txn.commit()?;
    Ok(())
}

pub fn get_draft(db: &Database, user_id: &str) -> Result<Option<String>, RedbError> {
    let drafts_table = TableDefinition::<&str, &str>::new("drafts");
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(drafts_table)?;
    if let Some(val) = table.get(user_id)? {
        Ok(Some(val.value().to_string()))
    } else {
        Ok(None)
    }
}
