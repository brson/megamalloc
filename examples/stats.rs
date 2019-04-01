use megamalloc::A;

fn main() {
    println!("allocator: {}", A.name());
    println!();

    match A.fetch_stats() {
        Ok(Some(stats)) => {
            for (k, v) in stats {
                println!("{} - {}", k, v);
            }
        }
        Ok(None) => {
            println!("no stats reported");
        }
        Err(e) => {
            println!("error: {}", e);
            std::process::exit(1);
        }
    }
}
