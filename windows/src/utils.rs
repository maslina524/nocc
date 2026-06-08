pub fn utf8_to_utf16le(s: &str, buffer: &mut [u16]) -> usize {
    let mut pos = 0;
    
    for ch in s.chars() {
        let code_point = ch as u32;
        
        if code_point <= 0xFFFF {
            if pos >= buffer.len() { break; }
            buffer[pos] = code_point as u16; // напрямую u16
            pos += 1;
        } else {
            // суррогатная пара
            if pos + 1 >= buffer.len() { break; }
            let code_point = code_point - 0x10000;
            let high = 0xD800 | ((code_point >> 10) as u16);
            let low = 0xDC00 | ((code_point & 0x3FF) as u16);
            buffer[pos] = high;
            buffer[pos + 1] = low;
            pos += 2;
        }
    }
    
    pos // количество записанных u16
}