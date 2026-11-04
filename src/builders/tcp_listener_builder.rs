use std::{
    net::{TcpListener},
    error::Error,
    net::Ipv4Addr
};

pub struct TcpListenerBuilder{

}

impl TcpListenerBuilder{
    
    pub fn build(host: &str, port: &str)-> Result<TcpListener, Box<dyn Error>>{

        if !Self::is_valid_host(host) {
            return Err("Invalid host".into());
        }
        if !Self::is_valid_port(port.parse().map_err(|_| "Invalid port number")?) {
            return Err("Port must be between 1024 and 65535".into());
        }

        let addr = format!("{host}:{port}");

        let listener: TcpListener = TcpListener::bind(addr).unwrap();

        println!("Server run: {}", listener.local_addr().unwrap());

        Ok(listener)
        
    }

    fn is_valid_port(port: u16) -> bool {
        (1024..=65535).contains(&port) || port == 0
    }
    
    fn is_valid_host(host: &str) -> bool {
        host.parse::<Ipv4Addr>().is_ok()
    }
    
}