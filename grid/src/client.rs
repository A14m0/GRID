// Defines all client-related functions and structures
use std::sync::Arc;
use std::net::{SocketAddr, ToSocketAddrs};

use mio::net::TcpStream;

use rustls::{
    ClientConfig,
    ClientConnection
};


use crate::definitions::string_to_domain;


/// structure defining a GRID client instance
pub struct GridClient {
    socket: TcpStream,
    remote: ClientConnection,
    tls_config: Arc<ClientConfig>
}


impl GridClient {
    /// Creates a new `GridClient` instance
    /// 
    /// ## Params: 
    /// * connection: String formatted as `"grid!domain:port"` or `"grid.ip:port"`
    /// 
    /// ## Returns:
    /// Returns either an intsance of the structure or an error string describing the issue encountered
    fn new(connection: String) -> Result<Self, String>{
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
        let client = match ClientConnection::new(rc_config.clone(), remote){
            Ok(a) => a,
            Err(e) => return Err(format!("{}", e))
        };
        
        let tmp = GridClient::build_socket_connect(target_domain.1, target_domain.2)?;
        
        let tcp_conn = match TcpStream::connect(tmp) {
            Ok(a) => a,
            Err(e) => return Err(format!("Connection failed: {}", e))
        };
            
        // return an instance of the structure
        Ok(GridClient {
            socket: tcp_conn,
            remote: client,
            tls_config: rc_config
        })
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


