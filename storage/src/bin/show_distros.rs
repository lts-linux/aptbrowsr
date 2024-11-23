use std::process::exit;

use storage::{establish_connection, distro::get_distros};
use common::distro::Distro;


fn main() {
    let conn = &mut establish_connection();

    let results: Vec<Distro> = match get_distros(conn, None) {
        Ok(ds) => ds,
        Err(e) => {
            println!("Error: {}", e);
            exit(1);
        }
    };

    println!("Displaying {} distros", results.len());

    for distro in results {
        println!("Distro: {:?}", distro);
    }
}
