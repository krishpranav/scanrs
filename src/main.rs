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