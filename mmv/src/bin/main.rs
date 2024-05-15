extern crate clap;

use clap::Parser;
use mass_move_lib::mass_move::mass_move;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "This utility allows you to transfer all files from one place to another using templates."
)]
pub struct Args {
    /// It consists of a path, a name, and a '*' symbol inside the name denoting a
    /// substring of any length (including an empty one).
    /// Example: 'path/to/some_*_filename.*'
    #[arg(verbatim_doc_comment)]
    pub source_pattern: String,

    /// It is formed from ordinary characters, and also uses special markers
    /// like '#1', '#2', and so on. These markers indicate which fragments,
    /// indicated by asterisks in the original template, should be inserted into
    /// the new file name.
    /// Example: 'path2/to/changed_#1_filename.#2'
    #[arg(verbatim_doc_comment)]
    pub target_pattern: String,

    ///If among the names of the resulting files there are names of already existing files,
    /// the program will not throw an error and will overwrite existing files.
    #[arg(short, long, verbatim_doc_comment)]
    pub force: bool,
}

fn main() {
    let args = Args::parse();

    if mass_move(&args.source_pattern, &args.target_pattern, args.force).is_err() {
        std::process::exit(1);
    }
}
