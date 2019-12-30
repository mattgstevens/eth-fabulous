use clap::{App, Arg};

fn main() {
    let matches = App::new("Eth Fabulous")
        .version("0.1.0")
        .author("Cody Lamson <tovarishfin@gmail.com>")
        .about("ethereum vanity address generator")
        .arg(
            Arg::with_name("regex")
                .short("r")
                .long("regex")
                .help("regex to search for when trying addresses")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let regex = matches.value_of("regex").unwrap();

    // run the app...
    if let Err(e) = eth_fabulous::run(regex) {
        eprintln!("Application error: {}", e)
    }
}
