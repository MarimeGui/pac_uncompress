extern crate ez_io;

mod dict;
mod hinttable;
mod uncompress;
mod util;

use crate::uncompress::uncompress;
use std::fs::File;
use std::io::{BufReader, BufWriter, Seek, SeekFrom};

const DICT_LEN: usize = 256;
const HINT_BITS: usize = 10;

fn main() {
    // Open the file directly for testing
    //let mut reader = BufReader::new(File::open("/home/guillaume/.local/share/Steam/steamapps/common/Hyperdimension Neptunia Re;Birth3/data/GAME200000.pac").unwrap());
    let mut reader = BufReader::new(File::open("/home/guillaume/.local/share/Steam/steamapps/common/Hyperdimension Neptunia Re;Birth3/data/MOVIE00000.pac").unwrap());

    // Seek on the correct data
    //reader.seek(SeekFrom::Start(0x30_B7D4)).unwrap();
    reader.seek(SeekFrom::Start(0x1182_8C25)).unwrap();

    // Create output file
    let mut writer = BufWriter::new(File::create("output.bin").unwrap());

    // Uncompress the data
    uncompress(&mut reader, &mut writer).unwrap();
}
