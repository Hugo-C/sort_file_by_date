use sort_file_by_date::{sort_year};
use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1", author = "Hugo-C")]
struct Opts {
    path: String,
    year: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("Starting to sort year {} of path {}", opts.year, opts.path);

    let res = sort_year(opts.path.as_str(), opts.year.as_str());

    match res {
        Ok(_) => { println!("Sorting done!"); }
        Err(_) => { eprintln!("An error occurred during sorting"); }
    }
}
