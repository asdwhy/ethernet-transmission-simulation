use crate::eth_network::EthernetNetwork;
use crate::eth_host::EthernetHost;

pub fn build_network(size: u32) -> EthernetNetwork {
    let mut eth_net = EthernetNetwork::new();

    for i in 0..size {
        let host = EthernetHost::new(i);
        eth_net.add_host(host);
    }

    eth_net
}
