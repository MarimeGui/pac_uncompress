use crate::util::load_new_data;
use crate::DICT_LEN;
use std::io::Read;

pub fn make_dict<R: Read>(
    dict: &mut [u16; DICT_LEN * 2],
    pak_tlen: &mut u32,
    pak_m: &mut u32,
    pak_k: &mut u32,
    reader: &mut R,
) -> u32 {
    if *pak_m <= 8 {
        load_new_data(reader, pak_k, pak_m).unwrap();
    }

    *pak_m -= 1;
    // Test for bit at pak_m (0 = lsb)
    if (*pak_k & (1 << *pak_m)) != (1 << *pak_m) {
        *pak_m -= 8;
        (*pak_k >> (*pak_m & 255)) & 255
    } else {
        *pak_tlen += 1;
        let index = 2 * *pak_tlen as usize - 514;
        dict[index] = make_dict(dict, pak_tlen, pak_m, pak_k, reader) as u16;
        dict[index + 1] = make_dict(dict, pak_tlen, pak_m, pak_k, reader) as u16;
        *pak_tlen - 1
    }
}
