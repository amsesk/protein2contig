extern crate bio;
use std::io;

pub fn protein_to_contig<R>(mut phead: Vec<String>, mut gff3: bio::io::gff::Reader<R>)
where
    R: io::Read,
{
    for record in gff3.records().map(|r| r.unwrap()) {
        if let Some(parent) = record.attributes().get("Parent") {
            if phead.iter().any(|h| h == parent) {
                let mut rm = 0;
                for (i, h) in phead.iter().enumerate() {
                    if h == parent {
                        rm = i;
                        println!("{}\t{}", h, record.seqname());
                    }
                }

                phead.remove(rm);
            }
        }
    }

    if !phead.is_empty() {
        println!(
            "Some headers ({} of them) did not have a match in the supplied GFF3:",
            phead.len()
        );
        println!("{}", phead.join("\n"));
    }
}
