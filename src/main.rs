use clap::{App, Arg};

fn main() {
    let matches = App::new("revext")
        .version("0.1")
        .author("Oriol Ornaque")
        .about("Reverses the filename extension with the Unicode right-to-left override")
        .arg(Arg::with_name("filename")
            .help("Filename with the real extension included")
            .required(true)
            .multiple(false)
            .index(1)
        )
        .arg(Arg::with_name("extension")
            .help("Fake extension")
            .required(true)
            .multiple(false)
            .index(2)
        )
        .get_matches();

    // filename rlo fake_extension.reverse() dot real_extension
    
    let rlo = '\u{202e}';

    let filename = matches.value_of("filename").unwrap();
    let fake_extension = matches.value_of("extension").unwrap();

    match filename.rsplit_once(".") {
        Some((filename, real_extension)) => {
            let mut filename = filename.to_string();
            filename.push(rlo);
            filename.push_str(&fake_extension.chars().rev().collect::<String>());
            filename.push('.');
            filename.push_str(&real_extension);
            println!("{}", filename);
        },
        None => {
            println!("Filename must include a file extension: filename.extension");
        }
    }
}