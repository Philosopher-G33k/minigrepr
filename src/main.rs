
use clap::Parser;
use minigrepr::search;


/*
Options Description
-c : This prints only a count of the lines that match a pattern
-i : Ignores, case for matching
-l : Displays list of a filenames only.
-n : Display the matched lines and their line numbers.
-v : This prints out all the lines that do not matches the pattern
-e exp : Specifies expression with this option. Can use multiple times.
-f file : Takes patterns from file, one per line.
-E : Treats pattern as an extended regular expression (ERE)
-w : Match whole word
-o : Print only the matched parts of a matching line,
 with each such part on a separate output line.
*/

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    pattern: String,
    search_in_text: String,

    /// This prints only a count of the lines that match a pattern 
    #[arg(short='c', action, required = false)]
    print_count: bool,

    /// Ignores, case for matching
    #[arg(short='i', action, required = false)]
    case_insensitive: bool,

    /// Displays list of a filenames only. 
    #[arg(short='l', action, required = false)]
    list_files_only: bool,

    /// Display the matched lines and their line numbers.
    #[arg(short='n', action, required = false)]
    display_only_line_and_numbers: bool,

    /// This prints out all the lines that do not matches the pattern
    #[arg(short='v', action, required = false)]
    display_only_nonmatching_line: bool, 

}

fn main() {
    let args = Args::parse();
    println!("{:#?}", args);
    println!("should print line {}", args.print_count);
    println!("is case insensitive {}", args.case_insensitive);
    search(args.pattern, args.search_in_text);
}
