// defines common definitions and structures 

//////////////////////// DEFAULTS ////////////////////////

/// Default GRID connection port
pub const GRID_DEFAULT_PORT: u16 = 7500;


//////////////////////// REQUESTS ////////////////////////

/// Defines the GRID request OPCODES
#[derive(Debug, Clone, Copy)]
pub enum GridRequestCode {
    /// Get resource at path
    GET=0,
    /// Put payload
    PUT=1,
    /// Set?
    SET=2,
    /// Client error
    CER=3
}



/// Defines the GRID response OPCODES
#[derive(Debug, Clone, Copy)]
pub enum GridResponseCode {
    /// Response OK
    ROK=128,
    /// General error 
    GER=129,
    /// Requested resource not found
    NOF=130,
    /// Remote is busy
    BSY=131
}




/// General enum for different GRID OPCODES
#[derive(Debug, Clone, Copy)]
pub enum GridCode {
    Response(GridResponseCode),
    Request(GridRequestCode)
}

/// implements Into and From traits for `GridRequestCode`
impl From<GridRequestCode> for GridCode {
    fn from(c: GridRequestCode) -> Self {
        GridCode::Request(c)
    } 
}

/// implements Into and From traits for `GridResponseCode`
impl From<GridResponseCode> for GridCode {
    fn from(c: GridResponseCode) -> Self {
        GridCode::Response(c)
    } 
}

impl GridCode {
    /// Converts the GridCode to a vector of bytes
    /// 
    /// ## Params:
    /// None
    /// 
    /// ## Returns:
    /// A byte that represents the code
    pub fn to_byte(
        &self
    ) -> u8 {
        // get the code
        match self {
            GridCode::Request(a) => *a as u8,
            GridCode::Response(a) => *a as u8
        }
    }

    /// Converts the given byte into the appropriate GRID code
    /// 
    /// ## Params:
    /// b: The byte to convert to GRID code
    /// 
    /// ## Returns
    /// Ok: an instance of `GridCode` with the appropriate byte
    /// Err: a string that explains the error encountered
    pub fn from_byte(
        b: u8
    ) -> Result<Self, String> {
        // check what type it should be
        if b < 128 {
            // its a request code
            match b {
                b if b == GridRequestCode::GET as u8 => Ok(GridCode::Request(GridRequestCode::GET)),
                b if b == GridRequestCode::PUT as u8 => Ok(GridCode::Request(GridRequestCode::PUT)),
                b if b == GridRequestCode::SET as u8 => Ok(GridCode::Request(GridRequestCode::SET)),
                b if b == GridRequestCode::CER as u8 => Ok(GridCode::Request(GridRequestCode::CER)),
                _ => Err(format!("Invalid request code {}", b))
            }
        } else {
            // its a response code
            match b {
                b if b == GridResponseCode::ROK as u8 => Ok(GridCode::Response(GridResponseCode::ROK)),
                b if b == GridResponseCode::BSY as u8 => Ok(GridCode::Response(GridResponseCode::BSY)),
                b if b == GridResponseCode::NOF as u8 => Ok(GridCode::Response(GridResponseCode::NOF)),
                b if b == GridResponseCode::GER as u8 => Ok(GridCode::Response(GridResponseCode::GER)),
                _ => Err(format!("Invalid response code {}", b))
            }
        }
    }
}



/// Defines our GRID request header
#[derive(Debug)]
pub struct GridBlock {
    opcode: GridCode,       // OPCODE of the request. Translates to one of the enum codes
    path_size: u128,        // Size of the path segment in the payload
    metadata_size: u128,    // Size of the metadata segment in the payload
    reserved: u128,         // Reserved for future endeavors 
    payload: Vec<u8>        // Bytes of the payload
}

impl GridBlock {
    /// Builds a GRID request structure
    /// 
    /// ## Params:
    /// * opcode: the GRID code to be used for the block
    /// * path: optional argument providing the path of the request
    /// * payload: the data to be sent over with the request
    /// 
    /// ## Returns:
    /// * Ok: Returns a GRID request structure
    /// * Err: Returns a string describing the issue encountered
    pub fn new(
        opcode: impl Into<GridCode>, 
        path: Option<&str>, 
        payload: &mut Vec<u8>
    ) -> Result<Self, String> {
        // build the payload of the request
        // note this is distinct from the provided payload, 
        // as it includes the path provided
        let mut request_payload: Vec<u8> = Vec::new();

        // calculate the path size
        let path_size = match path {
            Some(a) => {
                // given we have a path, 
                // add it to the payload
                let str_bytes = a.as_bytes();
                request_payload.extend_from_slice(&str_bytes); 
                a.len()
            },
            None => 0
        } as u128;

        // now get the payload info we need
        let metadata_size = payload.len() as u128;
        request_payload.append(payload);

        Ok(Self{
            opcode: opcode.into(),
            path_size,
            metadata_size,
            reserved: 0,
            payload: request_payload
        })
    }

    /// Creates a new GRID request block structure from serialized bytes
    /// 
    /// ## Params:
    /// * `bytes`: the vector of bytes to get the GRID request block from
    /// 
    /// ## Returns:
    /// Ok: An instance of `GridBlock`
    /// Err: A string representing the error encountered during deserialization
    pub fn from_bytes(
        bytes: Vec<u8>
    ) -> Result<Self, String> {
        // make sure the length is at least the length of the header
        let header_size = 1+16*3;
        if bytes.len() < header_size {
            return Err(format!("Too few bytes to recreate header: got {}", bytes.len()))
        }

        // parse header bytes
        let opcode = GridCode::from_byte(bytes[0])?;
        let mut u128_buff = [0u8; std::mem::size_of::<u128>()];
        u128_buff.copy_from_slice(&bytes[1..17]);
        let path_size = u128::from_be_bytes(u128_buff);
        u128_buff.copy_from_slice(&bytes[17..33]);
        let metadata_size = u128::from_be_bytes(u128_buff);
        u128_buff.copy_from_slice(&bytes[33..49]);
        let reserved = u128::from_be_bytes(u128_buff);

        // now that we have the rest of the bytes, make sure we got everything
        if bytes.len() != header_size + (path_size + metadata_size) as usize {
            return Err(format!("Incorrect bytes received. Size mismatch. Expected {}, got {}", header_size + (path_size + metadata_size) as usize, bytes.len()))
        }

        // now that the rest is looking OK, lets return the structure
        let mut payload: Vec<u8> = Vec::new();
        payload.extend_from_slice(&bytes[header_size..]);

        Ok(Self {
            opcode,
            path_size,
            metadata_size,
            reserved, 
            payload
        })
    }

    /// Serializes a GRID request block into raw bytes
    /// 
    /// ## Params:
    /// None
    /// 
    /// ## Returns:
    /// A byte array of the GRID request block, ready for network sending
    pub fn serialize(
        &mut self
    ) -> Vec<u8> {
        // create our buffer
        let mut buffer: Vec<u8> = Vec::new();

        // now we serialize all the fields...
        // ... first the opcode...
        buffer.push(self.opcode.to_byte());
        // ... then the path size...
        let mut buff = Vec::from_iter(self.path_size.to_be_bytes()); 
        buffer.append(&mut buff);
        // ... and the metadata size...
        let mut buff = Vec::from_iter(self.metadata_size.to_be_bytes()); 
        buffer.append(&mut buff);
        // ... the reserved flags...
        let mut buff = Vec::from_iter(self.reserved.to_be_bytes()); 
        buffer.append(&mut buff);
        // ... and finally the payload 
        buffer.append(&mut self.payload);

        // return the buffer
        buffer
    }
}




//////////////////////// HELPERS ////////////////////////

/// Defines either IP or domain name connection types
#[derive(Debug, PartialEq)]
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
pub fn string_to_domain(
        remote: impl Into<String>
    ) -> Result<(ConnectionType, String, u16), String> {
    // make sure the string begins with "grid"
    let conn_type: ConnectionType;
    let remote: String = remote.try_into().unwrap();
    if remote.starts_with("grid!") {
        // connecting to domain name
        conn_type = ConnectionType::Domain;
        
    } else if remote.starts_with("grid.") {
        // connecting to IP address
        conn_type = ConnectionType::Address
    } else {
        return Err(format!("No GRID connection specification included in {} (i.e. it is missing 'grid!' or 'grid.'), are you sure you are using the GRID protocol?", remote))
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


