use scripture::run;

fn main() {
    let matches = clap::App::new("Scripture")
        .version("0.1.0")
        .author("Mike Rosinsky")
        .arg(clap::Arg::with_name("file")
            .help("Input file")
            .required(true)
            .index(1))
        .get_matches_safe();

    // Retreive the value of the input file.
    match matches {
        Ok(matches) => {
            if let Some(input_file) = matches.value_of("file") {
                match run(input_file) {
                    Ok(_) => {},
                    Err(error) => {
                        error.report();
                        std::process::exit(1);
                    }
                }
            }
        },
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
