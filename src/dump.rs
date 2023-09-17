use std::cmp::min;

macro_rules! log {
    ($str:expr) => {
        #[cfg(test)]
        {
            let mut s = test::LOG.lock().unwrap();
            *s += $str;
        }
        #[cfg(not(test))]
        print!("{}", $str);
    };
    ($($t:tt)*) => {
        #[cfg(test)]
        {
            let mut s = test::LOG.lock().unwrap();
            *s += &format!($($t)*);
        }
        #[cfg(not(test))]
        print!($($t)*);
    };
}

fn dump_as_ascii(buf: &[u8]) {
    let mut l = Vec::<u8>::with_capacity(17);
    for e in buf {
        if *e >= 0x20 && *e <= 0x7e {
            l.push(*e);
        } else {
            l.push('.' as u8);
        }
    }
    let l = unsafe { std::str::from_utf8_unchecked(l.as_slice()) };
    log!("{}", l);
}

trait Endian {
    const HEADER_32: &'static str;
    const HEADER_64: &'static str;
    type Type;
    const WIDTH: usize = std::mem::size_of::<Self::Type>();
    fn dump_number(x: Self::Type);
    fn dump_empty();
}

struct Be2Byte {}
impl Endian for Be2Byte {
    const HEADER_32: &'static str = "Adress  : +0+1 +2+3 +4+5 +6+7 +8+9 +A+B +C+D +E+F 0123456789ABCDEF\n";
    const HEADER_64: &'static str = "Adress          : +0+1 +2+3 +4+5 +6+7 +8+9 +A+B +C+D +E+F 0123456789ABCDEF\n";
    type Type = u16;
    fn dump_number(x: Self::Type) {
        log!(" {:04x}", x.to_be());
    }
    fn dump_empty() {
        log!("     ");
    }
}

struct Le2Byte {}
impl Endian for Le2Byte {
    const HEADER_32: &'static str = "Adress  : +1+0 +3+2 +5+4 +7+6 +9+8 +B+A +D+C +F+E 0123456789ABCDEF\n";
    const HEADER_64: &'static str = "Adress          : +1+0 +3+2 +5+4 +7+6 +9+8 +B+A +D+C +F+E 0123456789ABCDEF\n";
    type Type = u16;
    fn dump_number(x: Self::Type) {
        log!(" {:04x}", x.to_le());
    }
    fn dump_empty() {
        log!("     ");
    }
}

struct Be4Byte {}
impl Endian for Be4Byte {
    const HEADER_32: &'static str = "Adress  : +0+1+2+3 +4+5+6+7 +8+9+A+B +C+D+E+F 0123456789ABCDEF\n";
    const HEADER_64: &'static str = "Adress          : +0+1+2+3 +4+5+6+7 +8+9+A+B +C+D+E+F 0123456789ABCDEF\n";
    type Type = u32;
    fn dump_number(x: Self::Type) {
        log!(" {:08x}", x.to_be());
    }
    fn dump_empty() {
        log!("         ");
    }
}

struct Le4Byte {}
impl Endian for Le4Byte {
    const HEADER_32: &'static str = "Adress  : +3+2+1+0 +7+6+5+4 +B+A+9+8 +F+E+D+C 0123456789ABCDEF\n";
    const HEADER_64: &'static str = "Adress          : +3+2+1+0 +7+6+5+4 +B+A+9+8 +F+E+D+C 0123456789ABCDEF\n";
    type Type = u32;
    fn dump_number(x: Self::Type) {
        log!(" {:08x}", x.to_le());
    }
    fn dump_empty() {
        log!("         ");
    }
}

struct Be8Byte {}
impl Endian for Be8Byte {
    const HEADER_32: &'static str = "Adress  : +0+1+2+3+4+5+6+7 +8+9+A+B+C+D+E+F 0123456789ABCDEF\n";
    const HEADER_64: &'static str = "Adress          : +0+1+2+3+4+5+6+7 +8+9+A+B+C+D+E+F 0123456789ABCDEF\n";
    type Type = u64;
    fn dump_number(x: Self::Type) {
        log!(" {:016x}", x.to_be());
    }
    fn dump_empty() {
        log!("                 ");
    }
}

struct Le8Byte {}
impl Endian for Le8Byte {
    const HEADER_32: &'static str = "Adress  : +7+6+5+4+3+2+1+0 +F+E+D+C+B+A+9+8 0123456789ABCDEF\n";
    const HEADER_64: &'static str = "Adress          : +7+6+5+4+3+2+1+0 +F+E+D+C+B+A+9+8 0123456789ABCDEF\n";
    type Type = u64;
    fn dump_number(x: Self::Type) {
        log!(" {:016x}", x.to_le());
    }
    fn dump_empty() {
        log!("                 ");
    }
}

struct Be16Byte {}
impl Endian for Be16Byte {
    const HEADER_32: &'static str = "Adress  : +0+1+2+3+4+5+6+7+8+9+A+B+C+D+E+F 0123456789ABCDEF\n";
    const HEADER_64: &'static str = "Adress          : +0+1+2+3+4+5+6+7+8+9+A+B+C+D+E+F 0123456789ABCDEF\n";
    type Type = u128;
    fn dump_number(x: Self::Type) {
        log!(" {:032x}", x.to_be());
    }
    fn dump_empty() {
        log!("                                 ");
    }
}

fn dump_data_line<T: Endian>(is64: bool, addr: usize, buf: &[T::Type])
where T::Type: Copy
{
    if is64 {
        log!("{:016x}:", addr);
    } else {
        log!("{:08x}:", addr);
    }
    for i in 0..(0x10 / T::WIDTH) {
        if buf.len() > i {
            T::dump_number(buf[i]);
        } else {
            T::dump_empty();
        }
    }
    let buf = unsafe {
        std::slice::from_raw_parts(buf.as_ptr().cast::<u8>(), buf.len() * T::WIDTH)
    };
    log!(" ");
    dump_as_ascii(buf);
    log!("\n");
}

fn dump_in<T: Endian>(peek: &nrpeek::Peek, addr: usize, size: usize) -> Option<()>
where T::Type: Copy
{
    let mut addr = addr - (addr & (T::WIDTH - 1));
    let mut size = size + (addr & (T::WIDTH - 1));
    size = (size - 1 + T::WIDTH) & !(T::WIDTH - 1);
    size /= T::WIDTH;
    let mut buf = Vec::<T::Type>::with_capacity(size);
    peek.peek_vec2(addr, &mut buf).ok()?;

    let is64_addr = (addr + size * T::WIDTH) > u32::MAX as usize;
    if is64_addr {
        log!("{}", T::HEADER_64);
    } else {
        log!("{}", T::HEADER_32);
    }
    while size > 0 {
        let s = buf.len() - size;
        let l = min(size, 0x10 / T::WIDTH);
        dump_data_line::<T>(is64_addr, addr, &buf[s..(s + l)]);
        addr += l * T::WIDTH;
        size -= l;
    }
    Some(())
}

pub fn dump_be16(peek: &nrpeek::Peek, addr: usize, size: usize) {
    dump_in::<Be16Byte>(peek, addr, size);
}

pub fn dump_be8(peek: &nrpeek::Peek, addr: usize, size: usize) {
    dump_in::<Be8Byte>(peek, addr, size);
}

pub fn dump_le8(peek: &nrpeek::Peek, addr: usize, size: usize) {
    dump_in::<Le8Byte>(peek, addr, size);
}

pub fn dump_be4(peek: &nrpeek::Peek, addr: usize, size: usize) {
    dump_in::<Be4Byte>(peek, addr, size);
}

pub fn dump_le4(peek: &nrpeek::Peek, addr: usize, size: usize) {
    dump_in::<Le4Byte>(peek, addr, size);
}

pub fn dump_be2(peek: &nrpeek::Peek, addr: usize, size: usize) {
    dump_in::<Be2Byte>(peek, addr, size);
}

pub fn dump_le2(peek: &nrpeek::Peek, addr: usize, size: usize) {
    dump_in::<Be2Byte>(peek, addr, size);
}

#[cfg(test)]
mod test {
    use super::*;
    use nrpeek::*;

    use std::sync::Mutex;
    static LOG_TEST: Mutex<()> = Mutex::new(());
    pub static LOG: Mutex<String> = Mutex::new(String::new());

    #[test]
    fn test_dump_be4_short() {
        let _lock =  LOG_TEST.lock();
        let p = Peek::new_with_pid(get_current_id());
        let s: [u32; 2] = [0x34333231, 0x38373635];
        let ss = s.as_ptr() as usize;
        let e = if ss > u32::MAX as usize {
            format!(r#"Adress          : +0+1+2+3 +4+5+6+7 +8+9+A+B +C+D+E+F 0123456789ABCDEF
{:016x}: 31323334 35363738                   12345678
"#, ss)
        } else {
            format!(r#"Adress          : +0+1+2+3 +4+5+6+7 +8+9+A+B +C+D+E+F 0123456789ABCDEF
{:08x}: 31323334 35363738                   12345678
"#, ss)
        };
        *LOG.lock().unwrap() = "".to_string();
        dump_be4(&p, ss, s.len() * 4);
        let r = LOG.lock().unwrap().to_string();
        assert_eq!(r, e);
        *LOG.lock().unwrap() = "".to_string();
    }

    #[test]
    fn test_dump_be16_long() {
        let _lock =  LOG_TEST.lock();
        let p = Peek::new_with_pid(get_current_id());
        let s: [u32; 8] = [0x34333231, 0x38373635, 0x64636261, 0x68676665, 0x34333231, 0x38373635, 0x34333231, 0x38373635];
        let ss = s.as_ptr() as usize;
        let e = if ss > u32::MAX as usize {
            format!(r#"Adress          : +0+1+2+3+4+5+6+7+8+9+A+B+C+D+E+F 0123456789ABCDEF
{:016x}: 31323334353637386162636465666768 12345678abcdefgh
{:016x}: 31323334353637383132333435363738 1234567812345678
"#, ss, ss + 16)
        } else {
            format!(r#"Adress          : +0+1+2+3+4+5+6+7+8+9+A+B+C+D+E+F 0123456789ABCDEF
{:08x}: 31323334353637386162636465666768 12345678abcdefgh
{:08x}: 31323334353637383132333435363738 1234567812345678
"#, ss, ss + 16)
        };
        *LOG.lock().unwrap() = "".to_string();
        dump_be16(&p, ss, s.len() * 4);
        let r = LOG.lock().unwrap().to_string();
        assert_eq!(r, e);
        println!("{}", *LOG.lock().unwrap());
    }

    #[test]
    fn test_dump_be8_long() {
        let _lock =  LOG_TEST.lock();
        let p = Peek::new_with_pid(get_current_id());
        let s: [u32; 6] = [0x34333231, 0x38373635, 0x64636261, 0x68676665, 0x34333231, 0x38373635];
        let ss = s.as_ptr() as usize;
        let e = if ss > u32::MAX as usize {
            format!(r#"Adress          : +0+1+2+3+4+5+6+7 +8+9+A+B+C+D+E+F 0123456789ABCDEF
{:016x}: 3132333435363738 6162636465666768 12345678abcdefgh
{:016x}: 3132333435363738                  12345678
"#, ss, ss + 16)
        } else {
            format!(r#"Adress          : +0+1+2+3+4+5+6+7 +8+9+A+B+C+D+E+F 0123456789ABCDEF
{:08x}: 3132333435363738 6162636465666768 12345678abcdefgh
{:08x}: 3132333435363738                  12345678
"#, ss, ss + 16)
        };
        *LOG.lock().unwrap() = "".to_string();
        dump_be8(&p, ss, s.len() * 4);
        let r = LOG.lock().unwrap().to_string();
        assert_eq!(r, e);
        println!("{}", *LOG.lock().unwrap());
    }

    #[test]
    fn test_dump_le8_long() {
        let _lock =  LOG_TEST.lock();
        let p = Peek::new_with_pid(get_current_id());
        let s: [u32; 6] = [u32::swap_bytes(0x34333231), u32::swap_bytes(0x38373635), u32::swap_bytes(0x64636261), u32::swap_bytes(0x68676665), u32::swap_bytes(0x34333231), u32::swap_bytes(0x38373635)];
        let ss = s.as_ptr() as usize;
        let e = if ss > u32::MAX as usize {
            format!(r#"Adress          : +7+6+5+4+3+2+1+0 +F+E+D+C+B+A+9+8 0123456789ABCDEF
{:016x}: 3536373831323334 6566676861626364 43218765dcbahgfe
{:016x}: 3536373831323334                  43218765
"#, ss, ss + 16)
        } else {
            format!(r#"Adress  : +7+6+5+4+3+2+1+0 +F+E+D+C+B+A+9+8 0123456789ABCDEF
{:08x}: 3536373831323334 6566676861626364 43218765dcbahgfe
{:08x}: 3536373831323334                  43218765
"#, ss, ss + 16)
        };
        *LOG.lock().unwrap() = "".to_string();
        dump_le8(&p, ss, s.len() * 4);
        let r = LOG.lock().unwrap().to_string();
        assert_eq!(r, e);
        println!("{}", *LOG.lock().unwrap());
    }

    #[test]
    fn test_dump_be4_long() {
        let _lock =  LOG_TEST.lock();
        let p = Peek::new_with_pid(get_current_id());
        let s: [u32; 6] = [0x34333231, 0x38373635, 0x64636261, 0x68676665, 0x34333231, 0x38373635];
        let ss = s.as_ptr() as usize;
        let e = if ss > u32::MAX as usize {
            format!(r#"Adress          : +0+1+2+3 +4+5+6+7 +8+9+A+B +C+D+E+F 0123456789ABCDEF
{:016x}: 31323334 35363738 61626364 65666768 12345678abcdefgh
{:016x}: 31323334 35363738                   12345678
"#, ss, ss + 16)
        } else {
            format!(r#"Adress          : +0+1+2+3 +4+5+6+7 +8+9+A+B +C+D+E+F 0123456789ABCDEF
{:08x}: 31323334 35363738 61626364 65666768 12345678abcdefgh
{:08x}: 31323334 35363738                   12345678
"#, ss, ss + 16)
        };
        *LOG.lock().unwrap() = "".to_string();
        dump_be4(&p, ss, s.len() * 4);
        let r = LOG.lock().unwrap().to_string();
        assert_eq!(r, e);
        println!("{}", *LOG.lock().unwrap());
    }

    #[test]
    fn test_dump_le4_long() {
        let _lock =  LOG_TEST.lock();
        let p = Peek::new_with_pid(get_current_id());
        let s: [u32; 6] = [u32::swap_bytes(0x34333231), u32::swap_bytes(0x38373635), u32::swap_bytes(0x64636261), u32::swap_bytes(0x68676665), u32::swap_bytes(0x34333231), u32::swap_bytes(0x38373635)];
        let ss = s.as_ptr() as usize;
        let e = if ss > u32::MAX as usize {
            format!(r#"Adress          : +3+2+1+0 +7+6+5+4 +B+A+9+8 +F+E+D+C 0123456789ABCDEF
{:016x}: 31323334 35363738 61626364 65666768 43218765dcbahgfe
{:016x}: 31323334 35363738                   43218765
"#, ss, ss + 16)
        } else {
            format!(r#"Adress  : +3+2+1+0 +7+6+5+4 +B+A+9+8 +F+E+D+C 0123456789ABCDEF
{:08x}: 31323334 35363738 61626364 65666768 43218765dcbahgfe
{:08x}: 31323334 35363738                   43218765
"#, ss, ss + 16)
        };
        *LOG.lock().unwrap() = "".to_string();
        dump_le4(&p, ss, s.len() * 4);
        let r = LOG.lock().unwrap().to_string();
        assert_eq!(r, e);
        println!("{}", *LOG.lock().unwrap());
    }

    #[test]
    fn test_dump_be2_long() {
        let _lock =  LOG_TEST.lock();
        let p = Peek::new_with_pid(get_current_id());
        let s: [u32; 6] = [0x34333231, 0x38373635, 0x64636261, 0x68676665, 0x34333231, 0x38373635];
        let ss = s.as_ptr() as usize;
        let e = if ss > u32::MAX as usize {
            format!(r#"Adress          : +0+1 +2+3 +4+5 +6+7 +8+9 +A+B +C+D +E+F 0123456789ABCDEF
{:016x}: 3132 3334 3536 3738 6162 6364 6566 6768 12345678abcdefgh
{:016x}: 3132 3334 3536 3738                     12345678
"#, ss, ss + 16)
        } else {
            format!(r#"Adress          : +0+1 +2+3 +4+5 +6+7 +8+9 +A+B +C+D +E+F 0123456789ABCDEF
{:08x}: 3132 3334 3536 3738 6162 6364 6566 6768 12345678abcdefgh
{:08x}: 3132 3334 3536 3738                     12345678
"#, ss, ss + 16)
        };
        *LOG.lock().unwrap() = "".to_string();
        dump_be2(&p, ss, s.len() * 4);
        let r = LOG.lock().unwrap().to_string();
        assert_eq!(r, e);
        println!("{}", *LOG.lock().unwrap());
    }
}
