fn main() {
    let db = postgres::Connection::connect(std::env::var("DATABASE_URL").expect("Missing DATABASE_URL"), postgres::TlsMode::None).expect("Failed to connect to database");

    let stmt = db.prepare("UPDATE redirects SET cache_visit_count_total=(SELECT COUNT(*) FROM visits WHERE redirect=redirects.id), cache_visit_count_month=(SELECT COUNT(*) FROM visits WHERE redirect=redirects.id AND tstamp > (localtimestamp - INTERVAL '1 MONTH'))").expect("Failed to prepare statement");

    loop {
        if let Err(err) = stmt.execute(&[]) {
            eprintln!("Error while updating caches: {:?}", err);
        } else {
            println!("Updated");
        }

        std::thread::sleep(std::time::Duration::new(10, 0));
    }
}
