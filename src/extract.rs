use clap::ArgMatches;
use flate2::read::MultiGzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::error::Error;

use bio::io::fastq;
use std::fs::File;
use std::io::Write;
use std::io;
use std::io::{BufReader, BufWriter};
use std::path::Path;

pub fn extract_command(sub_m: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let one_file_path = Path::new(sub_m.value_of("1").expect("can't find the 1 input file"))
        .canonicalize()
        .expect("can't find absolute path of 1 input file");

    let cb_file_path = Path::new(sub_m.value_of("2").expect("can't find the 2 input file"))
        .canonicalize()
        .expect("can't find absolute path of 2 input file");

    let two_file_path = Path::new(sub_m.value_of("3").expect("can't find the 3 input file"))
        .canonicalize()
        .expect("can't find absolute path of 3 input file");

    info!("Using input barcode files {:?}", cb_file_path);

    let file_o = File::open(&one_file_path).expect("can't read 1 file");
    let file_cb = File::open(&cb_file_path).expect("can't read 2 file");
    let file_t = File::open(&two_file_path).expect("can't read 3 file");

    let file_reader_o = MultiGzDecoder::new(file_o);
    let file_reade_cb = MultiGzDecoder::new(file_cb);
    let file_reader_t = MultiGzDecoder::new(file_t);

    let file_handle_o = BufReader::new(file_reader_o);
    let file_handle_cb = BufReader::new(file_reade_cb);
    let file_handle_t = BufReader::new(file_reader_t);

    let reader_o = fastq::Reader::new(file_handle_o);
    let reader_cb = fastq::Reader::new(file_handle_cb);
    let reader_t = fastq::Reader::new(file_handle_t);

    let mut records_o = reader_o.records().map(|r| r.unwrap())
        .into_iter();
    let mut records_cb = reader_cb.records().map(|r| r.unwrap())
        .into_iter();
    let mut records_t = reader_t.records().map(|r| r.unwrap())
        .into_iter();

    let ofile_path = one_file_path.parent();
    let ofile_1 = File::create(Path::new(ofile_path.unwrap()).join("R1.fq.gz"))
        .expect("can't write 1 file");
    let ofile_2 = File::create(Path::new(ofile_path.unwrap()).join("R2.fq.gz"))
        .expect("can't write 2 file");

    let file_writer_o = GzEncoder::new(ofile_1, Compression::default());
    let file_writer_t = GzEncoder::new(ofile_2, Compression::default());

    let file_handle_wo = BufWriter::new(file_writer_o);
    let file_handle_wt = BufWriter::new(file_writer_t);

    let mut writer_o = fastq::Writer::new(file_handle_wo);
    let mut writer_t = fastq::Writer::new(file_handle_wt);


    let mut index = 0;
    loop {
        let rec_1 = match records_o.next() {
            Some(record) => record,
            None => break,
        };

        let rec_cb = match records_cb.next() {
            Some(record) => record,
            None => panic!("mismatched CB file length"),
        };

        let rec_2 = match records_t.next() {
            Some(record) => record,
            None => panic!("mismatched R2 file length"),
        };

        let seq = String::from_utf8(rec_cb.seq().to_vec())
            .expect("can't unwrap cb sequecne");
        let id = format!("{}_{}", rec_1.id(), seq);

        writer_o.write(&id, None, rec_1.seq(), rec_1.qual())
            .expect("can't write into R1 file");

        writer_t.write(&id, None, rec_2.seq(), rec_2.qual())
            .expect("can't write into R2 file");

        index += 1;
        if index % 1_000_000 == 0 {
            print!("\r Done processing {:?} Million reads", index / 1_000_000);
            io::stdout().flush().unwrap();
        }        
    }

    Ok(())
}
