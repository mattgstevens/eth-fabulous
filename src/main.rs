use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Eth Fabulous")
        .version("0.1.0")
        .author("Cody Lamson <tovarishfin@gmail.com>")
        .about("ethereum vanity address generator")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@thing.com>")
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
        .get_matches();

    // gets a value for config if supplied by user or default to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    // because input is required we can simply unwrap here... clap enforces...
    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("no verbose info"),
        1 => println!("some verbose info"),
        2 => println!("tons of verbose info"),
        3 | _ => println!("insane amounts of verbose info"),
    }

    // You can handle information about subcommands by requesting their matches by name
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("printing debug info...");
        } else {
            println!("printing normally");
        }
    }

    // run the app...
    if let Err(e) = eth_fabulous::run() {
        eprintln!("Application error: {}", e)
    }
}
