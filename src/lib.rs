extern crate bio;
use std::io;

pub fn protein_to_contig<R>(
    mut phead: Vec<String>,
    mut gff3: bio::io::gff::Reader<R>,
) -> Result<String, &'static str>
where
    R: io::Read,
{
    for record in gff3.records().map(|r| r.unwrap()) {
        if let Some(parent) = record.attributes().get("Parent") {
            let spl: Vec<&str> = parent.split('_').collect();
            if phead.iter().any(|h| *h == spl[1]) {
                let mut rm = 0;
                for (i, h) in phead.iter().enumerate() {
                    if *h == spl[1] {
                        rm = i;
                        println!("{}\t{}", h, record.seqname());
                    }
                }

                phead.remove(rm);
            }
        }
    }
    Err("No match")
}
