use std::net::IpAddr;

#[derive(Copy, Clone)]
pub struct Request<'a> {
    pub address: IpAddr,
    pub method: &'a str,
    pub uri: &'a str,
    pub version: &'a str
}
