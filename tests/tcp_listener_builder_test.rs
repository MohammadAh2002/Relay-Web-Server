use relay_web_server::builders::tcp_listener_builder::TcpListenerBuilder;
use std::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successfully_build_tcp_listener(){
        
        let result =  TcpListenerBuilder::build("127.0.0.1", "5000");

        assert!(result.is_ok());
    }
    
    #[test]
    fn test_ports(){

        let result:Result<std::net::TcpListener, Box<dyn Error>> =  TcpListenerBuilder::build("127.0.0.1", "8080");
        
        assert_eq!(result.unwrap().local_addr().unwrap().port().to_string(), "8080");
    }   

    #[test]
    fn test_invalid_hosts(){
        
        // random host name
        assert!(TcpListenerBuilder::build("abc", "5000").is_err(), "Invalid host");
        // IPV6
        assert!(TcpListenerBuilder::build("::1", "5000").is_err(), "Invalid host");
        // out of range
        assert!(TcpListenerBuilder::build("999.999.999.999", "5000").is_err(), "Invalid host");

    }

    #[test]
    fn test_invalid_port(){
        
        assert!(TcpListenerBuilder::build("127.0.0.1", "5").is_err(), "Invalid port number");
        assert!(TcpListenerBuilder::build("127.0.0.1", "65536").is_err(), "Invalid port number");
        
    }

    #[test]
    fn test_random_port(){

        let result1:Result<std::net::TcpListener, Box<dyn Error>> = TcpListenerBuilder::build("127.0.0.1", "0");
        let result2:Result<std::net::TcpListener, Box<dyn Error>> = TcpListenerBuilder::build("127.0.0.1", "0");

        assert_ne!(result1.unwrap().local_addr().unwrap(), result2.unwrap().local_addr().unwrap());

    }

}