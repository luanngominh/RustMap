use structopt::StructOpt;
use std::net::IpAddr;
use colored::Colorize;
use rlimit::{Resource,setrlimit,getrlimit};
use log::*;

// rustmap -b banner -a ip_address -c chunk_size -u limit_file_description -v
// -b boolean: show banner
// -a string: ip_address
// -c integer: chunk size, describe how many port will scan at the time
// -u interger: max open file
// -v boolean: verbose

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short = "b", long = "banner")]
    banner: bool,

    #[structopt(short = "a", long = "address")]
    address: IpAddr,

    #[structopt(short = "c", long = "chunk", default_value = "10000")]
    chunk_size: u64,

    // Auto increase ulimit with value provied
    #[structopt(short = "u", long = "ulimit", default_value = "1024")]
    ulimit: u64,

    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}


fn main() {
    let args = Cli::from_args();
    println!("{:?}", args);
    if args.banner == true {
        banner();
    }

    adjust_ulimit(&args);
}


fn banner() {
    println!("{}", "banner".green());
}

fn adjust_ulimit(cli: &Cli) -> rlimit::rlim {
    match setrlimit(Resource::NOFILE, cli.ulimit, cli.ulimit) {
        Ok(_) => info!("set ulimit to {}", cli.ulimit),
        Err(_) => error!("fail to set ulimit to {}", cli.ulimit),
    }

    let (rlim, _) = getrlimit(Resource::NOFILE).unwrap_or_else(|e| {
        panic!("get rlimit error with message");
    });

    rlim
}
