use clap::{App, Arg};
use regex::Regex;

fn validate_hexadecimal(arg: String) -> Result<(), String> {
    let rgx = Regex::new("[0-9a-fA-F]+").unwrap();
    assert!(rgx.is_match(&arg));

    Ok(())
}

fn main() {
    let matches = App::new("Eth Fabulous")
        .version("0.1.0")
        .author("Cody Lamson <tovarishfin@gmail.com>")
        .about("ethereum vanity address generator")
        .arg(
            Arg::with_name("search")
                .short("s")
                .long("search")
                .help("value to search for when trying addresses")
                .takes_value(true)
                .required(true)
                .validator(validate_hexadecimal),
        )
        .get_matches();

    let search = matches.value_of("search").unwrap();

    if let Err(e) = eth_fabulous::run(search) {
        eprintln!("Application error: {}", e)
    }
}
