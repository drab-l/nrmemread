use std::env::Args;
#[cfg(target_os = "linux")]
use std::ffi::*;
#[cfg(target_os = "windows")]
use std::io::Write;

mod dump;

fn print_usage(bin: &str) -> ! {
    println!(
        r#"process memory reader.

Usage:
    {} [Option]
Option:
    -p: target process id, default is self.
"#,
        bin
    );
    std::process::exit(1);
}

struct Config {
    peek: Option<nrpeek::Peek>,
}

impl Config {
    fn new() -> Self {
        Self { peek: None }
    }
}

fn parse_opt_cb<T: Fn(&mut Config, &str)>(config: &mut Config, value: &str, args: &mut Args, expect: &str, cb: T) -> bool
{
    if !value.starts_with(expect) {
        false
    } else if value.len() == expect.len() && args.len() == 0 {
        false
    } else if value.len() > expect.len() {
        cb(config, &value[expect.len()..]);
        true
    } else {
        cb(config, &args.next().unwrap());
        true
    }
}

fn set_pid(config: &mut Config, value: &str) {
    if value == "self" {
        config.peek = Some(nrpeek::Peek::new_with_pid(nrpeek::get_current_id()));
    } else {
        config.peek = Some(nrpeek::Peek::new_with_pid(value.parse::<nrpeek::Pid>().unwrap()));
    }
}

fn parse_opt() -> Config {
    let mut config = Config::new();
    let mut args = std::env::args();
    let bin = args.next().unwrap();
    while args.len() > 0 {
        let head = args.next().unwrap();
        if head == "-h" {
            print_usage(&bin);
        } else if parse_opt_cb(&mut config, &head, &mut args, "-p", set_pid) {
            continue;
        }
        print_usage(&bin);
    }
    config
}

fn set_cbs(peek: nrpeek::Peek, calc: &mut nrmcalc::Calc) {
    let peek = std::rc::Rc::new(peek);
    macro_rules! set_peek_numeric {
        ($name:literal, $t:ty) => {
            let p = std::rc::Rc::clone(&peek);
            calc.set_sqr_bra_cb($name, move |x|
                Some(p.as_ref().peek_data::<$t>(x as usize).ok()? as i64));
        };
    }
    set_peek_numeric!("", u32);
    set_peek_numeric!("b", u8);
    set_peek_numeric!("w", u16);
    set_peek_numeric!("d", u32);
    set_peek_numeric!("q", u64);

    macro_rules! set_dump {
        ($name:literal, $func:ident) => {
            let p = std::rc::Rc::clone(&peek);
            calc.set_custom1_cb($name, move |size,addr| {
                dump::$func(p.as_ref(), addr as usize, size as usize);
                None
            });
        };
    }
    set_dump!("be2", dump_be2);
    set_dump!("le2", dump_le2);
    set_dump!("be4", dump_be4);
    set_dump!("le4", dump_le4);
    set_dump!("be8", dump_be8);
    set_dump!("le8", dump_le8);
    set_dump!("dump", dump_be16);
}

pub fn start() {
    let conf = parse_opt();
    let mut calc = nrmcalc::Calc::new();
    if conf.peek.is_some() {
        set_cbs(conf.peek.unwrap(), &mut calc);
    }
    loop {
        let Some(cmd) = readline() else {
            continue;
        };
        add_history(&cmd);
        if let Some(r) = calc.calc(&cmd) {
            println!("{}", r);
        }
    }
}

#[cfg(target_os = "linux")]
mod c {
    use std::ffi::*;
    extern "C" {
        pub fn free(ptr: *mut c_void);
        pub fn add_history(string: *const c_char);
        pub fn readline(prompt: *const c_char) -> *const c_char;
    }
}

#[cfg(target_os = "linux")]
pub fn readline() -> Option<String> {
    let prompt = "> \0".as_ptr().cast::<c_char>();
    let r = unsafe { c::readline(prompt) };
    if r.is_null() {
        None
    } else {
        let cmd = Some(unsafe { std::ffi::CStr::from_ptr(r) }.to_str().ok()?.to_owned());
        unsafe { c::free(r.cast_mut().cast::<c_void>()) };
        cmd
    }
}

#[cfg(target_os = "linux")]
pub fn add_history(string: &str) {
    let string = string.to_owned() + "\0";
    let string = string.as_ptr().cast::<c_char>();
    unsafe { c::add_history(string) };
}

#[cfg(target_os = "windows")]
fn readline() -> Option<String> {
    std::io::stdout().write(b"> ").ok()?;
    std::io::stdout().flush().unwrap();
    let mut cmd = String::new();
    std::io::stdin().read_line(&mut cmd).unwrap();
    let cmd = cmd.trim_end_matches(['\r', '\n']).to_string();
    Some(cmd)
}

#[cfg(target_os = "windows")]
fn add_history(_string: &str) { }
