use clap::{App, Arg};
use num_cpus;
use regex::Regex;

fn validate_hexadecimal(arg: String) -> Result<(), String> {
    let rgx = Regex::new("[0-9a-fA-F]+").unwrap();

    if rgx.is_match(&arg) {
        Ok(())
    } else {
        Err(String::from("search param must be hexadecimal"))
    }
}

fn validate_processors(arg: String) -> Result<(), String> {
    let cpus = num_cpus::get();
    let cpus_arg;

    match arg.parse::<usize>() {
        Ok(result) => cpus_arg = result,
        Err(_) => return Err(String::from("cpus argument must be a number.")),
    }

    match cpus_arg {
        0 => Err(format!(
            "cannot use 0 processors. computer has {} logical processors",
            cpus
        )),
        cpus_arg if cpus_arg > cpus => Err(format!(
            "cannot use more processors than computer has. computer has {} logical processors.",
            cpus
        )),
        _ => Ok(()),
    }
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
        .arg(
            Arg::with_name("cpus")
                .short("c")
                .long("cpus")
                .help("number of logical processors to use")
                .takes_value(true)
                .required(false)
                .validator(validate_processors),
        )
        .get_matches();

    let search = matches.value_of("search").unwrap();
    let cpus = match matches.value_of("cpus") {
        Some(result) => result.parse::<usize>().unwrap(),
        None => num_cpus::get(),
    };

    if let Err(e) = eth_fabulous::run(search, cpus) {
        eprintln!("Application error: {}", e)
    }
}
