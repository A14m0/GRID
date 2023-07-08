use grid::client::GridClient;
use grid::definitions::{GridBlock, GridRequestCode};

use clap::Parser;

#[derive(Parser, Debug)] // requires `derive` feature
#[command(term_width = 0)] // Just to make testing across clap features easier
struct Arguments {
    /// The GRID remote address. Ex: grid!localhost:1337
    #[arg(short='r', long="remote")]
    remote: String
}


fn main() {
    let args = Arguments::parse();

    // build our client (DEBUG: CONNECTING TO LOCALHOST 1337)
    let mut client = match GridClient::new(&args.remote) {
        Ok(a) => a,
        Err(e) => panic!("Failed to initialize GRID client: {}", e)
    };

    // build the GridBlock with a GET request
    let mut request = match GridBlock::new(GridRequestCode::GET, None, &mut Vec::new()) {
        Ok(a) => a,
        Err(e) => panic!("Failed to create new GRID request structure: {}", e)
    };

    // send it to the server and print what we got
    let response = match client.send(&mut request) {
        Ok(a) => a,
        Err(e) => panic!("Failed to send GRID Block: {}", e)
    };

    println!("Response: {:?}", response);
}
