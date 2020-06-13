extern crate bio;
extern crate clap;
extern crate csv;
extern crate flate2;
extern crate pretty_env_logger;

#[macro_use]
extern crate log;

use clap::{App, Arg, SubCommand};
use std::error::Error;

mod extract;
mod group;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("signac-kit")
        .version("0.1.0")
        .author("Avi Srivastava & Tim Stuart")
        .about("A set of fast helper functions for signac R package.")
        .subcommand(
            SubCommand::with_name("group")
                .about("A grouping command, analogous to group by command in R.")
                .arg(
                    Arg::with_name("input")
                        .long("input")
                        .short("i")
                        .takes_value(true)
                        .required(true)
                        .help("Input fragment.gz file"),
                )
                .arg(
                    Arg::with_name("column")
                        .long("column")
                        .short("c")
                        .takes_value(true)
                        .default_value("4")
                        .help("Column Index (1-indexing) to use for grouping."),
                )
                .arg(
                    Arg::with_name("output")
                        .long("output")
                        .short("o")
                        .takes_value(true)
                        .required(true)
                        .help("Path to the output file"),
                ),
        )
        .subcommand(
            SubCommand::with_name("extract")
                .about("A barcode extraction command, attaches CB to the header of the reads.")
                .arg(
                    Arg::with_name("1")
                        .long("1")
                        .short("1")
                        .takes_value(true)
                        .required(true)
                        .help("R1_001.fastq.gz file, typically left end of the read"),
                )
                .arg(
                    Arg::with_name("2")
                        .long("2")
                        .short("2")
                        .takes_value(true)
                        .required(true)
                        .help("R2_001.fastq.gz file, typically barcode containing read"),
                )
                .arg(
                    Arg::with_name("3")
                        .long("3")
                        .short("3")
                        .takes_value(true)
                        .required(true)
                        .help("R3_001.fastq.gz file, typically right end of the read"),
                ),
        )
        .get_matches();

    pretty_env_logger::init_timed();

    match matches.subcommand_matches("group") {
        Some(sub_m) => {
            let ret = group::group_command(&sub_m);
            return ret;
        }
        None => (),
    };

    match matches.subcommand_matches("extract") {
        Some(sub_m) => {
            let ret = extract::extract_command(&sub_m);
            return ret;
        }
        None => (),
    };
    Ok(())
}
