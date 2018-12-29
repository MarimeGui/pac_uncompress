use std::io::{Read, Result};
use std::mem::transmute;

// Reimplementation of x86 SHLD instruction
pub fn shld(dest: u32, src: u32, count: u8) -> u32 {
    let mut out = dest;
    out <<= count;
    out |= src >> (32 - count);
    out
}

pub fn load_new_data<R: Read>(reader: &mut R, pak_k: &mut u32, pak_m: &mut u32) -> Result<()> {
    let new_data = {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf).unwrap();
        // This is weird, I don't know if the data is read correctly
        u32::from_be(u32::from(unsafe { transmute::<[u8; 2], u16>(buf) }))
    };
    *pak_k = shld(*pak_k, new_data, 16);
    *pak_m += 16;
    Ok(())
}
