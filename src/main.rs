mod api;

use clap::Parser;

#[derive(Parser, Debug, Default)]
#[clap(author="Lukas Sandman", version="1.0", about="CLI-app to measure internet speed among other things, using data from fast.com.", long_about = None)]
struct Cli {
    /// Display more information such as latency, ip-address, client location, server location, and more.
    #[clap(short, long, takes_value = false)]
    more: bool

}

fn main(){
    let args = Cli::parse();
    let more = args.more;

    match api::fetch_data(more) {
        Ok(()) => (),
        Err(e) => panic!("{}",e),
    };

}
