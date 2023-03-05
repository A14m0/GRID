/*           LIBGRID
This library implements the GRID protocol. It should eventually have C bindings
but that will be down the road a little bit
*/

pub mod client;
pub mod definitions;
pub mod server;

// test cases
mod test {
    use std::string;

    use crate::definitions::ConnectionType;

    use super::*;

    #[test]
    fn server_remote_string_parse() {
        use definitions::string_to_domain;
        use definitions::GRID_DEFAULT_PORT;
        // this function tests the `string_to_domain` function under
        // definitions.rs.
        let test1 = string_to_domain("grid!testdomain"); 
        assert!(test1.is_ok());
        match test1 {
            Ok(a) => {
                assert_eq!(a.0, ConnectionType::Domain);
                assert_eq!(a.1, "testdomain");
                assert_eq!(a.2, GRID_DEFAULT_PORT);
            },
            Err(e) => unreachable!()
        }

        let test2 = string_to_domain("grid!testdomain:1234"); 
        assert!(test2.is_ok());
        match test2 {
            Ok(a) => {
                assert_eq!(a.0, ConnectionType::Domain);
                assert_eq!(a.1, "testdomain");
                assert_eq!(a.2, 1234);
            },
            Err(e) => unreachable!()
        }

        let test3 = string_to_domain("grid.1.2.3.4"); 
        assert!(test3.is_ok());
        match test3 {
            Ok(a) => {
                assert_eq!(a.0, ConnectionType::Address);
                assert_eq!(a.1, "1.2.3.4");
                assert_eq!(a.2, GRID_DEFAULT_PORT);
            },
            Err(e) => unreachable!()
        }

        let test4 = string_to_domain("grid.1.2.3.4:1234"); 
        assert!(test4.is_ok());
        match test4 {
            Ok(a) => {
                assert_eq!(a.0, ConnectionType::Address);
                assert_eq!(a.1, "1.2.3.4");
                assert_eq!(a.2, 1234);
            },
            Err(e) => unreachable!()
        }
        


        // check port bounds
        assert!(string_to_domain("grid!testdomain:1234567").is_err());
        assert!(string_to_domain("grid!testdomain:12345").is_ok());
        
        // check parsing of grid protocol 
        assert!(string_to_domain("grub!testdomain").is_err());
        assert!(string_to_domain("grub.1.2.3.4").is_err());
        assert!(string_to_domain("gri!testdomain:1234").is_err());
        assert!(string_to_domain("gri.1.2.3.4:1234").is_err());
        assert!(string_to_domain("grid 1.2.3.4:1234").is_err());
        assert!(string_to_domain("grid@1.2.3.4:1234").is_err());
    }
}