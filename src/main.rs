fn main() {
    let mut db = fontdb::Database::new();
    db.load_system_fonts();
    println!("{}", db.family_name(&fontdb::Family::SansSerif));
}
