use crate::{eth_host::EthernetHost};

pub struct EthernetNetwork {
    pub hosts: Vec<EthernetHost>
}

impl EthernetNetwork {
    pub fn new() -> EthernetNetwork {
        EthernetNetwork {
            hosts: Vec::new()
        }
    }

    pub fn add_host(&mut self, host: EthernetHost) {
        self.hosts.push(host);
    }

    pub fn get_hosts(&mut self) -> &mut Vec<EthernetHost> {
        &mut self.hosts
    }
}