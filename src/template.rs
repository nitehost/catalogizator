type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct CatalogTemplate {
    id: u32,
    name: String,
    content: String,
}

impl CatalogTemplate {

    pub fn get_id(&self) -> &u32 {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

}

pub fn get_templates() -> Result<Vec<CatalogTemplate>> {

    let conn = rusqlite::Connection::open(crate::DATABASE)?;
    
    let mut stmt = conn.prepare("SELECT id, name FROM templates")?;
    let mut rows = stmt.query([])?;
        
    let mut templates:Vec<CatalogTemplate> = vec![];
    while let Some(row) = rows.next()? {
        templates.push(
            CatalogTemplate {
                id: row.get(0)?,
                name: row.get(1)?,
                content: String::new(),
            }
        );
    }

    Ok(templates)
}


pub fn get_template(id: u32) -> Result<CatalogTemplate> {

    let conn = rusqlite::Connection::open(crate::DATABASE)?;
    
    let mut stmt = conn.prepare("
        SELECT id, name, content
        FROM templates
        WHERE id = ?1
    ")?;
    let mut rows = stmt.query([id])?;
        
    if let Some(row) = rows.next()? {
        return Ok(CatalogTemplate {
            id: row.get(0)?,
            name: row.get(1)?,
            content: row.get(2)?,
        });
    }

    Err("Something wrong!".into())
}