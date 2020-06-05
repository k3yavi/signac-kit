extern crate csv;
extern crate clap;
extern crate flate2;
extern crate pretty_env_logger;

#[macro_use]
extern crate log;

use std::io;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::error::Error;
use std::io::BufReader;
use std::collections::HashMap;

use flate2::read::MultiGzDecoder;
use clap::{App, Arg, ArgMatches, SubCommand};

fn group_command(sub_m: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let ifile_path = Path::new(sub_m.value_of("input")
                       .expect("can't find the input file"))
        .canonicalize()
        .expect("can't find absolute path of input file");

    let ofile_path = sub_m.value_of("output")
        .unwrap();

    let mut column_index = match sub_m.value_of("column") {
        Some(x) => x.parse::<usize>().expect("can't parse the column index"),
        None => panic!("Can't find what column index to use."),
    };

    info!("Using input file {:?} and extracting {:?} column", ifile_path, column_index);
    column_index -= 1;

    let mut group_hash = HashMap::<String, usize>::new();
    let file = File::open(&ifile_path).expect("can't read file");
    let file_reader = MultiGzDecoder::new(file);
    let file_handle = BufReader::new(file_reader);

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_reader(file_handle);

    for (index, result) in rdr.records().enumerate() {
        let record = result?;
        let cid = record.get(column_index).expect("can't extract the column");
        *group_hash.entry(cid.to_string()).or_insert(0) += 1;

        if index % 1_000_000 == 0 {
            print!("\r Done processing {:?} Million lines", index / 1_000_000);
            io::stdout().flush().unwrap();
        }
    }
    println!();
    info!("Done Parsing the input file, writing output");

    let mut ofile = File::create(&ofile_path).expect("can't create output file");
    for (k,v) in group_hash.into_iter() {
        writeln!(&mut ofile, "{}\t{}", k, v)?;
    }

    info!("All Done!");
    Ok(())
}

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
                )
        )
        .get_matches();

    pretty_env_logger::init_timed();

    match matches.subcommand_matches("group") {
        Some(sub_m) =>  {
            let ret = group_command(&sub_m);
            return ret;
        }
        None => ()
    };

    Ok(())
}
