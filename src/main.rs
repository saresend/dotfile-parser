use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Dotfile Parser")
        .version("1.0")
        .author("Samuel Resendez")
        .arg(
            Arg::with_name("INPUT_FILE")
                .short("f")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let filename = matches.value_of("INPUT_FILE").unwrap();
    
}
