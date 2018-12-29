use crate::{DICT_LEN, HINT_BITS};

pub fn make_hint_table(dict: &[u16; DICT_LEN * 2], hints: &mut [[u16; 2]; 1 << HINT_BITS]) {
    let mut hint_index = 0usize;

    loop {
        let mut read_value = 256u16;
        let mut bit_index = HINT_BITS as u16;
        loop {
            bit_index -= 1;
            let bit_test = ((hint_index as u16) >> bit_index) & 1; // Gets the bit in hint_id at index bit_index
            let dict_index = 2 * read_value - 512 + bit_test;
            read_value = dict[dict_index as usize];
            if (read_value <= 255) | (bit_index == 0) {
                break;
            }
        }

        let stop = ((1 << bit_index as usize) - 1) | hint_index;

        loop {
            hints[hint_index] = [read_value, bit_index];
            hint_index += 1;
            if hint_index > stop {
                break;
            }
        }

        // Stop looping if hints buffer is filled
        if hint_index >= (1 << HINT_BITS) {
            break;
        }
    }
}
