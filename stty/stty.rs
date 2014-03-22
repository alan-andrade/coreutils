#[crate_id(name="stty", ver="1.0.0", author="Alan Andrade")];

/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Alan Andrade <alan.andradec@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE file
 * that was distributed with this source code.
 *
 * Synced with: http://src.gnu-darwin.org/src/bin/stty/stty.c.html
 */

#[feature(macro_rules)];
extern crate getopts;

use getopts::{optflag, optopt, getopts, usage};
use std::os;
use std::io::{File,Path};
use std::libc::{c_char, c_int,STDIN_FILENO};
use std::mem::uninit;

#[path = "../common/util.rs"] mod util;

static NAME: &'static str = "stty";
struct winsize;

struct info {
	fd:     int,
	ldisc:  int,
	off:    int,
	set:    int,
	wset:   int,
	arg:    *char,
	t:      termios,
	win:    winsize
}

//typedef unsigned long tcflag_t;
type tcflag_t = u64;
//typedef unsigned char cc_t;
type cc_t = char;
//typedef unsigned long speed_t;
type speed_t = u64;

static NCCS: &'static int = &20;

struct termios {
  c_iflag: tcflag_t,
  c_oflag: tcflag_t,
  c_cflag: tcflag_t,
  c_lflag: tcflag_t,
  //c_cc:  cc_t[NCCS],
  c_ispeed: speed_t,
  c_ospeed: speed_t
}

extern {
    fn tcgetattr(fildes: c_int, termios_p: *termios) -> c_int;
    fn gread(foo: *termios, bar: *c_char);
}

fn main () {
    let args = os::args();

    let options = [
        optflag(
            "a",
            "",
            "Display settings for the terminal as per IEEE Std 1003.2"),
        optflag(
            "e",
            "",
            "Display settings for the terminal in the traditional BSD formats"),
        optopt(
            "f",
            "",
            "Open and use the terminal named by file rather than using standard input.",
            "-f /dev/ttys001"),
        optflag(
            "g",
            "",
            "Display settings to stdout in a form that may be used as argument")
    ];

    let matches = match getopts(args.tail(), options) {
        Ok(m) => { m },
        Err(_) => { crash!(1, "{}", usage(NAME, options)) }
    };

    if matches.opt_present("f") {
        println!("f option");

        let filename = matches.opt_str("f").unwrap();

        match File::open(&Path::new(filename)) {
            Ok(_) => {},
            Err(e) => { crash!(1, "{}", e.desc) }
        }
    }

    let termios = unsafe { uninit::<termios>() };

    let attr = unsafe { tcgetattr(STDIN_FILENO, &termios as *termios) };
    println!("{}", attr);

    if attr < 0 {
        crash!(1, "Bullshit");
    }

    let key = "is_speed";

    unsafe { gread(&termios as *termios, key.as_ptr() as *i8) }

    println!("{}", termios.c_iflag);
}
