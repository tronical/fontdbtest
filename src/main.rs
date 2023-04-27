fn main() {
    let mut db = fontdb::Database::new();
    db.load_system_fonts();
    let mut query = fontdb::Query::default();
    query.families = &[fontdb::Family::SansSerif];
    match db.query(&query) {
        Some(id) => {
            println!(
                "query for sans-serif returned: {:#?}",
                db.face(id).as_ref().map(|f| &f.source)
            );
        }
        None => println!("Could not locate default sans serif font!"),
    }
}
