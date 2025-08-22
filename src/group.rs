type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct CatalogGroup {
    id: u32,
    collection_id: u32,
    name: String,
}

impl CatalogGroup {

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

pub fn get_groups(collection_id: u32) -> Result<Vec<CatalogGroup>> {

    let conn = rusqlite::Connection::open(crate::DATABASE)?;

    let mut stmt = conn.prepare("
        SELECT id, collection_id, name
        FROM groups
        WHERE collection_id = ?1
    ")?;
    let mut rows = stmt.query([collection_id])?;

    let mut groups:Vec<CatalogGroup> = vec![];
    while let Some(row) = rows.next()? {
        groups.push(
            CatalogGroup {
                id: row.get(0)?,
                collection_id: row.get(1)?,
                name: row.get(2)?,
            }
        );
    }
    
    Ok(groups)
}