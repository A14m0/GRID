// defines common definitions and structures 

//////////////////////// DEFAULTS ////////////////////////

/// Default GRID connection port
const GRID_DEFAULT_PORT: u16 = 7500;


//////////////////////// REQUESTS ////////////////////////

/// Defines the GRID request OPCODES
pub enum GridRequestCode {
    /// Get resource at path
    GET,
    /// Put payload
    PUT,
    /// Set?
    SET,
}

/// Defines the GRID response OPCODES
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

/// General enum for different GRID OPCODES
pub enum GridCode {
    Response(GridResponseCode),
    Request(GridRequestCode)
}


/// Defines our GRID request header
pub struct GridRequest {
    opcode: GridCode,             // OPCODE of the request. Translates to one of the enum codes
    path_size: u128,        // Size of the path segment in the payload
    metadata_size: u128,    // Size of the metadata segment in the payload
    payload: Box<[u8]>      // Bytes of the payload
}




//////////////////////// HELPERS ////////////////////////

/// Defines either IP or domain name connection types
pub enum ConnectionType {
    Address,
    Domain
}

/// Translates a given remote string into a connection target
/// 
/// ## Params:
///  * remote: String formatted as `"grid!domain:port"` or `"grid.ip:port"`. Port is optional
/// 
/// ## Returns:
///  * Ok: returns a tuple of the type of connection, the domain/IP of the connection, and the port
///  * Err: returns a string describing the issue encountered
pub fn string_to_domain(remote: String) -> Result<(ConnectionType, String, u16), String> {
    // make sure the string begins with "grid"
    let conn_type: ConnectionType;
    if remote.starts_with("grid!") {
        // connecting to domain name
        conn_type = ConnectionType::Domain;
        
    } else if remote.starts_with("grid.") {
        // connecting to IP address
        conn_type = ConnectionType::Address
    } else {
        return Err(format!("No GRID connection specification included in {} (i.e. it is missing 'grid!' or 'grid.')", remote))
    }

    // now that we know what kind of connection type we have, build the other structures
    let port: u16;
    let domain: String;

    if remote.contains(":") {
        // non-default port
        let splits: Vec<&str> = remote.split_terminator(':').collect();
        let split_lengths = splits.len();
        domain = splits[0][5..].to_string();
        
        // make sure we only have two different items from the split
        if split_lengths != 2 {
            // if we only have one, we can assume that the port should be default
            if split_lengths == 1 {
                port = GRID_DEFAULT_PORT;
            } else {
                // not valid -> Err out
                return Err(format!("Illegal remote {}: Only one defintion of port allowed in definition!", remote));
            }
            
        } else {
            // correctly formatted statement, parse it
            port = match splits[1].parse() {
                Ok(a) => a,
                Err(e) => return Err(format!("Failed to parse non-standard port: {}", e))
            }
        }
    } else {
        // default port, build structure
        domain = remote[5..].to_string();
        port = GRID_DEFAULT_PORT;
    }

    // return the stuff 
    Ok((conn_type, domain, port))
}