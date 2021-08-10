// using regex and clap to build our command line tool
use regex::Regex;
use clap::{App, Arg};

fn main() {
// defining the clap App
    let args = App::new("grep-lite")
    .version("0.3")
    .about("search for patterns in text")
    .arg(Arg::with_name("pattern")
    .help("The pattern to search for")
    .takes_value(true)
    .required(true))
    .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();
    let quote = "\
    Every face, every shop,
    bedroom window, public-house, and 
    dark square is a picture 
    feverishly turned--in search of what?
    It is the same with books.
    What do we seek through 
    millions of pages?";

    for line in quote.lines(){
        match re.find(line){
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
