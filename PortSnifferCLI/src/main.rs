use ::std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::{Sender, channel};
use std::thread;
use std::{env, process};

const MAX: u16 = 65535;
struct Arguments {
    flag: String,
    ip_addr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(Arguments: &[String]) -> Result<Arguments, &'static str> {
        if Arguments.len() < 2 {
            return Err("Not enough arguments");
        } else if Arguments.len() > 4 {
            return Err("Too many arguments");
        }
        let f = Arguments[1].clone();
        if let Ok(ip_addr) = IpAddr::from_str(&f) {
            return Ok(Arguments {
                flag: String::from(""),
                ip_addr,
                threads: 4,
            });
        } else {
            let flag = Arguments[1].clone();
            if flag.contains("-h") || flag.contains("-help") && Arguments.len() == 2 {
                println!(
                    "Usage: -j to select how many threads to use \n\r -h or -help to display this message"
                );
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("too many arguments");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&Arguments[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("invalid IP, must be in IPv4 format or ipv6 format"),
                };
                let threads = match Arguments[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("invalid thread number"),
                };
                return Ok(Arguments {
                    threads,
                    flag,
                    ip_addr: ipaddr,
                });
            } else {
                return Err("Invalid syntax");
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{}: {}", program, err);
            process::exit(0);
        }
    });

    let num_threads = arguments.threads;
    let addr = arguments.ip_addr;
    let (tx, rs) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, arguments.ip_addr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rs {
        out.push(p);
    }
    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}
