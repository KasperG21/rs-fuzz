use clap::{Arg, ArgMatches, Command};

pub fn args() {
    set_vars(collect_matches());
}

fn collect_matches() -> ArgMatches {
    let matches = Command::new("rs-fuzz")
        .author("Gudo_")
        .bin_name("rs-fuzz")
        .before_help(r"
                             /$$$$$$                              
                            /$$__  $$                             
  /$$$$$$   /$$$$$$$       | $$  \__//$$   /$$ /$$$$$$$$ /$$$$$$$$
 /$$__  $$ /$$_____//$$$$$$| $$$$   | $$  | $$|____ /$$/|____ /$$/
| $$  \__/|  $$$$$$|______/| $$_/   | $$  | $$   /$$$$/    /$$$$/ 
| $$       \____  $$       | $$     | $$  | $$  /$$__/    /$$__/  
| $$       /$$$$$$$/       | $$     |  $$$$$$/ /$$$$$$$$ /$$$$$$$$
|__/      |_______/        |__/      \______/ |________/|________/

By Gudo_
                                                                  
")
        .arg(
            Arg::new("url")
            .short('u')
            .long("url")
            .value_name("URL")
            .required(true)
            .help("This sets the url to fuzz to, specifically the 'FUZZ' part of the url will be replaced by the fuzzing keywords.")
        )
        .arg(
            Arg::new("wordlist")
            .short('w')
            .long("wordlist")
            .value_name("PATH-TO-WORDLIST")
            .required(true)
            .help("This sets the path to the wordlist containing the fuzzing keywoards. (The wordlist needs to be encoded in plaintext)")
        )
        .arg(
            Arg::new("threads")
                .short('t')
                .long("threads")
                .value_name("AMOUNT-OF-THREADS")
                .default_value("16")
                .help("Specifies how many threads rs-fuzz can use."),
        )
        .get_matches();

    matches
}

fn set_vars(matches: ArgMatches) -> Arguments {
    // Unwrapping is safe, because those are a required argument and the program won't run without them.
    let url: &String = matches.get_one("url").unwrap();
    let wordlist: &String = matches.get_one("wordlist").unwrap();

    Arguments {
        url: url.to_owned(),
        wordlist: wordlist.to_owned(),
    }
}

struct Arguments {
    url: String,
    wordlist: String,
    threads: usize,
}
