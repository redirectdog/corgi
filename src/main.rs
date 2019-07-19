fn main() {
    let db = postgres::Connection::connect(
        std::env::var("DATABASE_URL").expect("Missing DATABASE_URL"),
        postgres::TlsMode::None,
    )
    .expect("Failed to connect to database");

    let stmt_visits = db.prepare("UPDATE redirects SET cache_visit_count_total=(SELECT COUNT(*) FROM visits WHERE redirect=redirects.id), cache_visit_count_month=(SELECT COUNT(*) FROM visits WHERE redirect=redirects.id AND tstamp > (localtimestamp - INTERVAL '1 MONTH'))").expect("Failed to prepare statement");

    let stmt_tiers = db.prepare("UPDATE users SET tier=COALESCE((SELECT tier FROM user_subscriptions WHERE user_id=users.id AND start_timestamp < localtimestamp AND (end_timestamp IS NULL OR end_timestamp > localtimestamp) ORDER BY tier DESC), 0)").expect("Failed to prepare statement");

    loop {
        if let Err(err) = stmt_visits.execute(&[]) {
            eprintln!("Error while updating visit caches: {:?}", err);
        } else {
            println!("Updated visits");
        }

        if let Err(err) = stmt_tiers.execute(&[]) {
            eprintln!("Error while updating tier caches: {:?}", err);
        } else {
            println!("Updated tiers");
        }

        std::thread::sleep(std::time::Duration::new(10, 0));
    }
}
