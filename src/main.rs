use mysql::*;
use mysql::prelude::*;
use std::thread;
use std::time::Duration;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://root:password@localhost:3307/db_name";
    let pool = Pool::new(url)?;

    loop {
        let mut conn = pool.get_conn()?;

        let result: Vec<(String, i32)> = conn.query_map(
            "SHOW STATUS LIKE 'Threads_connected'",
            |(status_var, value)| {
                (status_var, value)
            },
        )?;

        let num_connections = result[0].1;
        println!("Opened connections: {}", num_connections);

        thread::sleep(Duration::from_secs(1));
    }
}
