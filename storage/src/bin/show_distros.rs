use storage::{establish_connection, distro::get_distros};
use libapt::Distro;


fn main() {
    let conn = &mut establish_connection();

    let results: Vec<Distro> = get_distros(conn, None);

    println!("Displaying {} distros", results.len());

    for distro in results {
        println!("Distro: {:?}", distro);
    }
}
