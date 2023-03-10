use std::io::Write;
use std::net::{IpAddr, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::{env, io, process, thread};

const DEFAULT_THREADS_NUM: u16 = 4;
const MAX_PORT: u16 = 65535;

#[derive(Debug)]
struct Arguments {
    flag: String,
    ip_addr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }

        let f = args[1].clone();
        if let Ok(ip_addr) = f.parse::<IpAddr>() {
            return Ok(Arguments {
                flag: String::from(""),
                ip_addr,
                threads: DEFAULT_THREADS_NUM,
            });
        } else {
            let flag = args[1].clone();
            if (flag.contains("-h") || flag.contains("-help")) && args.len() == 2 {
                println!(
                    "Usage: -j to select how many threads you want
                \n\r       -h or -help to show this help message"
                );
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("too many arguments");
            } else if flag.contains("-j") {
                let ip_addr = match args[3].parse::<IpAddr>() {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6"),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread number"),
                };
                return Ok(Arguments {
                    threads,
                    flag,
                    ip_addr,
                });
            } else {
                return Err("invalid syntax");
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0)
        } else {
            eprintln!("{} problem parsing arguments: {}", program, err);
            process::exit(0)
        }
    });

    println!("{:?}", arguments);

    let num_threads = arguments.threads;
    let addr = arguments.ip_addr;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }

    let mut out: Vec<u16> = vec![];
    drop(tx);
    for port in rx {
        out.push(port);
    }

    println!("");

    out.sort();
    for port in out {
        println!("{} is open", port);
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

        if (MAX_PORT - port) <= num_threads {
            break;
        }

        port += num_threads;
    }
}
