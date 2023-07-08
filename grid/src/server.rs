// Defines all client-related functions and structures
use std::sync::Arc;
use std::net::{SocketAddr, ToSocketAddrs};

use mio::net::{TcpListener, TcpStream};

use rustls::{
    ServerConfig,
    ServerConnection
};

use rcgen::generate_simple_self_signed;


use crate::definitions::string_to_domain;




/// defines a structure for holding certificates and private keys
#[derive(Clone)]
pub struct CertificateStore {
    priv_key: Vec<u8>,
    ocsp: Vec<u8>,
    domains: Vec<String>,
}

impl CertificateStore {
    /// Creates a new `CertificateStore` instance
    /// 
    /// ## Params:
    /// * priv_key: private key as a vector of bytes in PKCS#8 format  
    /// * ocsp: ???
    /// * domains: vector of all domain names the X.509 certificate is valid for
    /// 
    /// ## Returns:
    /// * instance of the structure
    pub fn new(priv_key: Vec<u8>, ocsp: Vec<u8>, domains: Vec<String>) -> Self {
        CertificateStore{priv_key, ocsp, domains}
    }

    /// Returns the private key of the certificate
    /// 
    /// ## Params:
    /// None
    /// 
    /// ## Returns:
    /// * a rustls::PrivateKey structure
    pub fn get_privkey(self) -> rustls::PrivateKey {
        rustls::PrivateKey(self.priv_key.clone())
    }

    pub fn get_ocsp(self) -> Vec<u8> {
        self.ocsp.clone()
    }

    pub fn get_domains(self) -> Vec<String> {
        self.domains.clone()
    }

    /// Generates a vector of rustls certificates 
    /// 
    /// ## Params: 
    /// None
    /// 
    /// ## Returns:
    /// * Ok: returns a vector of `rustls::Certificate` structures
    /// * Err: returns a string describing the issue encountered
    pub fn get_certificates(self) -> Result<Vec<rustls::Certificate>,String> {
        todo!()
    }
}


/// structure defining a GRID client instance
pub struct GridServer {
    socket: TcpListener,
    remote: ServerConnection,
    tls_config: Arc<ServerConfig>
}

impl GridServer {
    /// Creates a new `GridClient` instance
    /// 
    /// ## Params: 
    /// * port: the port to be listening on
    /// 
    /// ## Returns:
    /// * Ok: an instance of a GridServer structure
    /// * Err: a string describing the issue encountered
    pub fn new(port: u16, certs: Option<CertificateStore>) -> Result<Self, String>{
        // see if we need to load certificates from default location or if they're pre-provided
        let c = match certs {
            Some(a) => a,
            None => gen_certificate(None)?
        };

        let certificates = c.clone().get_certificates()?;
        let privkey = c.clone().get_privkey();
        let ocsp = c.get_ocsp();
        

        // build a rustls configuration using the new TLS store
        let config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert_with_ocsp_and_sct(certificates, privkey, ocsp, vec![])
            .expect("bad certificates/private key");

        // set up remote connection to server
        let rc_config = Arc::new(config);
        
        
        
        ////////////////////////////////////////////////////////////////////////
        // THIS IS WRONG! ServerConnection IS WHAT HANDLES INBOUND CONNECTIONS,
        // NOT FOR BINDING TO AN ADDRESS! MOVE TO DIFFERENT FUNCTION!
        // 
        // for example of how thats supposed to work, see the following:
        // https://github.com/rustls/rustls/blob/main/rustls-mio/examples/tlsserver.rs
        // 
        // REWORK!!!
        ////////////////////////////////////////////////////////////////////////
        
        // build client connection
        let client = match ServerConnection::new(rc_config.clone()){
            Ok(a) => a,
            Err(e) => return Err(format!("{}", e))
        };
        
            
        // return an instance of the structure
        Ok(GridServer {
            socket: todo!(),
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







//////////////////////// MISC HELPERS ///////////////////////////

/// Generates a self-signed X.509 certificate for use in the server structure
/// 
/// ## Params:
/// * names: optional vector of domain names this certificate should be valid for. If not provided, defaults to "localhost"
/// 
/// ## Returns:
/// * Ok: returns a CertificateStore structure for use
/// * Err: returns a string describing the issue encountered
pub fn gen_certificate(names: Option<Vec<String>>) -> Result<CertificateStore, String> {
    // see if we have any names available currently, otherwise 'localhost'
    let domains = match names {
        Some(a) => a,
        None => vec!["localhost".to_string()]
    };

    // build the certificate
    let cert = match generate_simple_self_signed(domains.clone()) {
        Ok(a) => a,
        Err(e) => return Err(format!("Certificate generation failed: {}", e))
    };

    
    Ok(CertificateStore::new(cert.serialize_private_key_der(), vec![], domains))
}
