use grid::server::GridServer;

fn main() {
    // build our server instance
    let server = match GridServer::new(1337, None) {
        Ok(a) => a,
        Err(e) => panic!("Failed to create GRID server instance: {}", e)
    };

    // bind to the port
    match server.bind() {
        Ok(_) => (),
        Err(e) => panic!("Failed to bind server to port: {}", e)
    }

    // loop and handle connections
    loop {

    }
}
