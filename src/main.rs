extern crate bio;
mod lib;
use clap::{App, Arg};
use std::fs::File;
use std::io::BufRead;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let matches = App::new("protien2contig")
        .version("0.1")
        .author("Kevin Amses")
        .about("Takes a list of protein headers, a gff3, and a fasta and matches the each protein with its originating contig.")
        .arg(Arg::with_name("gff3")
            .long("gff3")
            .value_name("FILE")
            .about("Path to GFF3.")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("fasta")
            .long("fasta")
            .value_name("FILE")
            .about("Path to assembly fasta.")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("headers")
            .long("headers")
            .value_name("FILE")
            .about("Path to \n-separated list of protein headers.")
            .takes_value(true)
            .required(true))
        .get_matches();

    let gff3_path = matches.value_of("gff3").unwrap();
    let _fasta_path = matches.value_of("fasta").unwrap();
    let headers_path = matches.value_of("headers").unwrap();

    let gff3 = bio::io::gff::Reader::from_file(&gff3_path, bio::io::gff::GffType::GFF3).unwrap();

    let headers = File::open(headers_path)?;
    let headers = BufReader::new(headers);
    let headers: Vec<String> = headers
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.split('_').collect::<Vec<&str>>()[1].to_owned())
        .collect();
    let _pairs = lib::protein_to_contig(headers, gff3);
    Ok(())
}
