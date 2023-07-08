use std::io::{Write, Read, self};
// Defines all client-related functions and structures
use std::sync::Arc;
use std::net::{SocketAddr, ToSocketAddrs};

use mio::net::TcpStream;

use rustls::{
    ClientConfig,
    ClientConnection, 
    Stream
};


use crate::definitions::{
    GridBlock,
    string_to_domain
};


/// structure defining a GRID client instance
pub struct GridClient {
    socket: TcpStream,
    client: ClientConnection,
    tls_config: Arc<ClientConfig>
}


impl GridClient {
    /// Creates a new `GridClient` instance
    /// 
    /// ## Params: 
    /// * connection: String formatted as `"grid!domain:port"` or `"grid.ip:port"`
    /// 
    /// ## Returns:
    /// Returns either an instance of the structure or an error string describing the issue encountered
    pub fn new(
        connection: impl Into<String>
    ) -> Result<Self, String>{
        // make sure we convert the thing into a string
        let connection: String = connection.try_into().unwrap();

        // set up the root TLS store 
        let mut root_store = rustls::RootCertStore::empty();
        root_store.add_server_trust_anchors(
            webpki_roots::TLS_SERVER_ROOTS
                .0
                .iter()
                .map(|ta| {
                    rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                        ta.subject,
                        ta.spki,
                        ta.name_constraints,
                    )
                })
        );

        // build a rustls configuration using the new TLS store
        let config = ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        // set up remote connection to server
        let rc_config = Arc::new(config);
        let target_domain = string_to_domain(connection)?;
        // convert domain into rustls target
        let remote = match target_domain.1[..].try_into() {
            Ok(a) => a,
            Err(e) => return Err(format!("Cannot put remote into TLS target type: {}", e))
        };

        // build client connection
        let mut client = match ClientConnection::new(rc_config.clone(), remote){
            Ok(a) => a,
            Err(e) => return Err(format!("{}", e))
        };
        
        let tmp = GridClient::build_socket_connect(target_domain.1, target_domain.2)?;
        
        let mut tcp_conn = match TcpStream::connect(tmp) {
            Ok(a) => a,
            Err(e) => return Err(format!("Connection failed: {}", e))
        };

        // return an instance of the structure
        Ok(GridClient {
            socket: tcp_conn,
            client,
            tls_config: rc_config
        })
    }

    /// Sends a GridRequest to the remote server 
    /// 
    /// ## Params:
    /// * request: the GridBlock structure to be sent over
    /// 
    /// ## Returns:
    /// * Ok: a response GridBlock structure from the server
    /// * Err: a string describing the issue encountered
    pub fn send(
        &mut self,
        request: &mut GridBlock
    ) -> Result<GridBlock, String> {
        // first we need to serialize the request
        let serialized_request = request.serialize();
        println!("Serialized: {:?}", serialized_request);

        self.client.writer().write(&serialized_request);

        // then we can send it to the connected server
        while self.client.wants_write() {
            match self.client.write_tls(&mut self.socket){//.write_all(&mut serialized_request) {
                Ok(_) => (),
                Err(e) => return Err(format!("{}", e))
            };
        }


        // now we read back from the server
        let mut response_raw: Vec<u8> = Vec::new();
        match self.read_into(&mut response_raw) {
            Ok(a) => println!("Read {}", a),
            Err(e) => return Err(format!("Failed to recieve response from server: {}",e)) 
        };

        println!("Response: {:?}", response_raw);

        // deserialize the bytes and return them
        GridBlock::from_bytes(response_raw)
    }

    /// Helper function to read TLS data into a buffer
    /// 
    /// ## Params:
    /// `buff`: A buffer to write the data into
    /// 
    /// ## Returns:
    /// Ok: Returns the number of bytes read from the connection
    /// Err: Returns a string that describes the error encountered
    pub fn read_into(
        &mut self,
        buff: &mut Vec<u8>,
    ) -> Result<usize, String> {
        // first read TLS data
        if self.client.wants_read() {
            let mut blocked = true;
            
            while blocked {
                let count = match self.client.read_tls(&mut self.socket) {
                    Ok(a) => {blocked = false; a},
                    Err(e) => {
                        if e.kind() != io::ErrorKind::WouldBlock {
                            return Err(format!("Failed to get tls data because {}", e))
                        }
                        0
                    }
                };
                
            }
    
            // next we process the packets
            let io_state = match self.client.process_new_packets() {
                Ok(a) => a,
                Err(e) => return Err(format!("TLS error {}", e))
            };
            
            if io_state.peer_has_closed() {
                return Err(format!("remote closed"))
            }
    
            // and finally we read back from the buffer
            let ret_cnt = match self.client.reader().read(buff){
                Ok(a) => a,
                Err(e) => return Err(format!("TLS read failed {}", e))
            };
            
            return Ok(ret_cnt);
        }
        
        // nothing to read, bail
        Err(format!("no data available for reading"))
    
    }


    /// Helper function to help build a TLS connection target
    /// 
    /// Basically stolen from the rustls tlsclient example. [Link here](https://github.com/rustls/rustls/blob/main/rustls-mio/examples/tlsclient.rs)
    /// 
    /// ## Params:
    /// * domain: the domain string of the remote target
    /// * port: the port used to connect to the remote
    /// 
    /// ## Returns:
    /// * Ok: returns a socket address for use in connections
    /// * Err: returns a string describing the issue encountered
    fn build_socket_connect(domain: String, port: u16) -> Result<SocketAddr, String>{
        let tmp = (&domain[..], port).to_socket_addrs().unwrap();
        for addr in tmp {
            if let SocketAddr::V4(_) = addr {
                return Ok(addr);
            }
        }

        // failed to parse it
        Err(format!("Failed to lookup domain {}", domain))
    }
}


