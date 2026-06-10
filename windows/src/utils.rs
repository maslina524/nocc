pub fn utf8_to_utf16le(s: &str, buffer: &mut [u16]) -> usize {
    let mut pos = 0;
    
    for ch in s.chars() {
        let code_point = ch as u32;
        
        if code_point <= 0xFFFF {
            if pos >= buffer.len() { break; }
            buffer[pos] = code_point as u16;
            pos += 1;
        } else {
            if pos + 1 >= buffer.len() { break; }
            let code_point = code_point - 0x10000;
            let high = 0xD800 | ((code_point >> 10) as u16);
            let low = 0xDC00 | ((code_point & 0x3FF) as u16);
            buffer[pos] = high;
            buffer[pos + 1] = low;
            pos += 2;
        }
    }
    
    pos
}

pub fn utf16le_to_utf8(s: &[u16], buffer: &mut [u8]) -> usize {
    let mut i = 0;
    let mut written = 0;

    while i < s.len() {
        let c = s[i];
        i += 1;
        if c == 0 { break; }

        let code_point = if c >= 0xD800 && c <= 0xDBFF {
            if i >= s.len() { break; }
            let low = s[i];
            if low < 0xDC00 || low > 0xDFFF { break; } 
            i += 1;
            0x10000 + ((c as u32 - 0xD800) << 10) + (low as u32 - 0xDC00)
        } else {
            c as u32
        };

        if code_point > 0x10FFFF { break; }

        let len = if code_point < 0x80 {
            1
        } else if code_point < 0x800 {
            2
        } else if code_point < 0x10000 {
            3
        } else {
            4
        };

        if written + len > buffer.len() {
            break;
        }

        // Кодируем в UTF-8
        match len {
            1 => buffer[written] = code_point as u8,
            2 => {
                buffer[written]     = (0xC0 | ((code_point >> 6) & 0x1F)) as u8;
                buffer[written + 1] = (0x80 | (code_point & 0x3F)) as u8;
            }
            3 => {
                buffer[written]     = (0xE0 | ((code_point >> 12) & 0x0F)) as u8;
                buffer[written + 1] = (0x80 | ((code_point >> 6) & 0x3F)) as u8;
                buffer[written + 2] = (0x80 | (code_point & 0x3F)) as u8;
            }
            4 => {
                buffer[written]     = (0xF0 | ((code_point >> 18) & 0x07)) as u8;
                buffer[written + 1] = (0x80 | ((code_point >> 12) & 0x3F)) as u8;
                buffer[written + 2] = (0x80 | ((code_point >> 6) & 0x3F)) as u8;
                buffer[written + 3] = (0x80 | (code_point & 0x3F)) as u8;
            }
            _ => unreachable!(),
        }

        written += len;
    }

    written
}