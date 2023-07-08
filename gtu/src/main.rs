use grid::client::GridClient;
use grid::definitions::{GridBlock, GridRequestCode};

fn main() {
    // build our client (DEBUG: CONNECTING TO LOCALHOST 1337)
    let mut client = match GridClient::new("grid.127.0.0.1:1337") {
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
