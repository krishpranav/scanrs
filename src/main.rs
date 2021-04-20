// port scanner

extern crate getopts;

use getopts::Options;
use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::{Sender, channel};
use std::thread;

// max possible port

const MAX: u16 = 65535

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("j",
    "",
    "number of threads to use for concurrent scanning",
    "THREADS");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            println!("error parsing options: {}", e.to_string())
            return;
        }
    };

    if matches.opt_present("h") || matches.free.is_empty() {
        let brief = format!("Usage: {} -j THREADS IPADDR", program);
        print!("{}", opts.usage(&brief));
        return;
    }
}