type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct CatalogStatus {
    id: u32,
    collection_id: u32,
    name: String,
}

impl CatalogStatus {

    pub fn get_id(&self) -> &u32 {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_collection_id(&self) -> &u32 {
        &self.collection_id
    }

}

pub fn get_statuses(collection_id: u32) -> Result<Vec<CatalogStatus>> {

    let conn = rusqlite::Connection::open(crate::DATABASE)?;

    let mut stmt = conn.prepare("
        SELECT id, collection_id, name
        FROM statuses
        WHERE collection_id = ?1
    ")?;
    let mut rows = stmt.query([collection_id])?;

    let mut statuses:Vec<CatalogStatus> = vec![];
    while let Some(row) = rows.next()? {
        statuses.push(
            CatalogStatus {
                id: row.get(0)?,
                collection_id: row.get(1)?,
                name: row.get(2)?,
            }
        );
    }
    
    Ok(statuses)
}