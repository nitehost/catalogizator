use chrono::NaiveDateTime;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct CatalogEntry {
    id: u32,
    status_id: u32,
    collection_id: u32,
    group_id: u32,
    flag_favorites: bool,
    flag_deleted: bool,
    flag_archived: bool,
    title: String,
    image: String,
    created_date: NaiveDateTime,
    content: String,
}

impl CatalogEntry {
    pub fn get_id(&self) -> &u32 {
        &self.id
    }

    pub fn get_collection_id(&self) -> &u32 {
        &self.collection_id
    }

    pub fn get_group_id(&self) -> &u32 {
        &self.group_id
    }

    pub fn get_status_id(&self) -> &u32 {
        &self.status_id
    }

    pub fn is_favorites(&self) -> bool {
        self.flag_favorites
    }

    pub fn is_deleted(&self) -> bool {
        self.flag_deleted
    }

    pub fn is_archived(&self) -> bool {
        self.flag_archived
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_image(&self) -> &String {
        &self.image
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn get_created_date(&self) -> &NaiveDateTime {
        &self.created_date
    }
}

pub fn get_entries(collection_id: u32) -> Result<Vec<CatalogEntry>> {
    let conn = rusqlite::Connection::open(crate::DATABASE)?;

    let mut stmt = conn.prepare(
        "
        SELECT id, status_id, collection_id, group_id,
            flag_favorites, flag_deleted, flag_archived,
            title, image, content, created_date
        FROM entries
        WHERE collection_id = ?1
    ",
    )?;
    let mut rows = stmt.query([collection_id])?;
    let mut entries: Vec<CatalogEntry> = vec![];

    while let Some(row) = rows.next()? {
        entries.push(CatalogEntry {
            id: row.get(0)?,
            status_id: row.get(1)?,
            collection_id: row.get(2)?,
            group_id: row.get(3)?,

            flag_favorites: row.get(4)?,
            flag_deleted: row.get(5)?,
            flag_archived: row.get(6)?,

            title: row.get(7)?,
            image: match row.get(8) {
                Ok(img) => img,
                _ => String::new(),
            },
            content: row.get(9)?,

            created_date: NaiveDateTime::parse_from_str(
                row.get::<usize, String>(10)?.as_str(),
                crate::DATEFORMAT,
            )?,
        });
    }

    Ok(entries)
}
