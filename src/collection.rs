use gtk4 as gtk;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct CatalogCollection {
    id: u32,
    name: String,
}

impl CatalogCollection {

    pub fn get_id(&self) -> &u32 {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

}

pub fn get_collections() -> Result<Vec<CatalogCollection>> {

    let conn = rusqlite::Connection::open(crate::DATABASE)?;
    
    let mut stmt = conn.prepare("SELECT id, name FROM collections")?;
    let mut rows = stmt.query([])?;
        
    let mut collections:Vec<CatalogCollection> = vec![];
    while let Some(row) = rows.next()? {
        collections.push(
            CatalogCollection {
                id: row.get(0)?,
                name: row.get(1)?,
            }
        );
    }

    Ok(collections)
}

pub fn get_collections_model() -> Result<gtk::StringList> {

    let stringlist = gtk::StringList::new(&["-- Не выбрано --"]);
    let collections = get_collections()?;

    for collection in collections {
        stringlist.append(collection.get_name());
    }

    Ok(stringlist)
}