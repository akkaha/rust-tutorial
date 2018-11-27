extern crate clap;

use clap::App;

fn main() {
    App::new("hello clap")
        .version("1.0.0")
        .about("hello clap demo")
        .author("zhengshaodong")
        .get_matches();
}
