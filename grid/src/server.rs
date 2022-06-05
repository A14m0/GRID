// Defines all client-related functions and structures
use std::sync::Arc;
use std::net::{SocketAddr, ToSocketAddrs};

use mio::net::{TcpListener, TcpStream};

use rustls::{
    ServerConfig,
    ServerConnection
};


use crate::definitions::string_to_domain;


/// structure defining a GRID client instance
pub struct GridServer {
    socket: TcpListener,
    remote: ServerConnection,
    tls_config: Arc<ServerConfig>
}


/// defines a structure for holding certificates and private keys
pub struct CertificateStore {
    
}

impl GridServer {
    /// Creates a new `GridClient` instance
    /// 
    /// ## Params: 
    /// * port: the port to be listening on
    /// 
    /// ## Returns:
    /// Returns either an intsance of the structure or an error string describing the issue encountered
    fn new(port: u16, certs: Option<CertificateStore>) -> Result<Self, String>{
        // see if we need to load certificates from default location or if they're pre-provided
        if let c = Some(certs) {

        }
        let privkey = ;
        let ocsp = ;
        

        // build a rustls configuration using the new TLS store
        let config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert_with_ocsp_and_sct(certs, privkey, ocsp, vec![])
            .expect("bad certificates/private key");;

        // set up remote connection to server
        let rc_config = Arc::new(config);
        
        // build client connection
        let client = match ServerConnection::new(rc_config.clone()){
            Ok(a) => a,
            Err(e) => return Err(format!("{}", e))
        };
        
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


