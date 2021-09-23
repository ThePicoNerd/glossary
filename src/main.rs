use std::{io, path::PathBuf};

use csv::WriterBuilder;
use structopt::StructOpt;
use walkdir::WalkDir;

use crate::term::{Dictionary, Entry};

mod term;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Sub,
}

#[derive(Debug, StructOpt)]
enum Sub {
    #[structopt(about = "Combines multiple dictionaries")]
    Merge {
        #[structopt(parse(from_os_str))]
        root: PathBuf,

        #[structopt(long)]
        sort: bool,
    },
}

pub fn main() {
    let opt = Opt::from_args();

    match opt.cmd {
        Sub::Merge { root, sort } => {
            let mut terms: Vec<Entry> = WalkDir::new(root)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
                .flat_map(|e| {
                    let dict = Dictionary::from_file(e.path()).unwrap();

                    dict.terms
                })
                .collect();

            if sort {
                terms.sort_unstable_by(|a, b| a.term.cmp(&b.term));
            }

            let mut wtr = WriterBuilder::new()
                .has_headers(false)
                .from_writer(io::stdout());

            for term in terms {
                wtr.serialize(term).unwrap();
            }
        }
    }
}
