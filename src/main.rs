use structopt::StructOpt;
use std::net::IpAddr;

// rustmap -b banner -a ip_address -c chunk_size -u limit_file_description -v
// -b boolean: show banner
// -a string: ip_address
// -c integer: chunk size, describe how many port will scan at the time
// -u interger: max open file
// -v boolean: verbose

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "b", long = "banner")]
    banner: bool,

    #[structopt(short = "a", long = "address")]
    address: IpAddr,

    #[structopt(short = "c", long = "chunk", default_value = "default 10000")]
    chunk_size: u32,

    #[structopt(short = "u", long = "ulimit", default_value = "default 9999")]
    ulimit: u32,

    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}


fn main() {
    let args = Cli::from_args();
}
