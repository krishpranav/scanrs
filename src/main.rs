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

    let num_threads = match matches.opt_str("j") {
        Some(j) => j.parse().expect("flag -j must be an integer"),
        None => 4,
    }

    let addr = IpAddr::from_str(&matches.free[0])
        .expect("IPADDR must be valid ipv4 or ipv6 address");

    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for port in rx {
        out.push(port);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}

func scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;

    loop {
        match TcpStream::connect((addr, port)) {
            ok(-) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX - port) <= num_threads {
            break;
        }

        port += num_threads;
    }
}