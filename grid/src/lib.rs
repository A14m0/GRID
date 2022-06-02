/*           LIBGRID
This library implements the GRID protocol. It should eventually have C bindings
but that will be down the road a little bit
*/


use std::io;
use std::sync::Arc;


use rustls::{
    ClientConfig,
    ClientConnection
};

/// Defines the GRID request OPCODES
pub enum GridRequestCode {
    /// Get resource at path
    GET,
    /// Put payload
    PUT,
    /// Set?
    SET,
}

pub enum GridResponseCode {
    /// Response OK
    ROK,
    /// General error 
    GER,
    /// Requested resource not found
    NOF,
    /// Remote is busy
    BSY
}


/// structure defining a GRID client instance
pub struct GridClient {
    remote: ClientConnection,
    tls_config: Arc<ClientConfig>
}


impl GridClient {
    /// Creates a new `GridClient` instance
    /// 
    /// # Params: 
    /// * connection: String formatted as `"grid!domain:port"` or `"grid.ip:port"`
    /// 
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
        let target_domain = string_to_domain(connection)?[..].try_into().unwrap();
        let client = match ClientConnection::new(rc_config.clone(), target_domain){
            Ok(a) => a,
            Err(e) => return Err(format!("{}", e))
        };
            
        // return an instance of the structure
        Ok(GridClient {
            remote: client,
            tls_config: rc_config
        })
    }
}

/// translates a given remote string into a connection target
pub fn string_to_domain(remote: String) -> Result<String, String> {
    Ok(String::new())
}

/// Defines our GRID request header
pub struct GridRequest {
    opcode: u8,             // OPCODE of the request. Translates to one of the enum codes
    path_size: u128,        // Size of the path segment in the payload
    metadata_size: u128,    // Size of the metadata segment in the payload
    payload: Box<[u8]>      // Bytes of the payload
}







#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
