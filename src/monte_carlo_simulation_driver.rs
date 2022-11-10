use crate::{eth_network_builder::build_network, cmd_ln_config::{Config}, eth_network::EthernetNetwork, eth_host::EthernetHost};

struct NetworkSimulation <'a>{
    time: u32,
    network: EthernetNetwork,
    config: &'a mut Config
}

impl<'a> NetworkSimulation<'a> {
    fn new(config: &'a mut Config, network: EthernetNetwork) -> NetworkSimulation<'a> {
        NetworkSimulation { time: 0, config: config, network: network }
    }

    fn start_simulation(&mut self) -> u32 {
        loop {
            if self.config.verbose { println!("Starting step {}", self.time) };

            match self.step() {
                Some(host) => {
                    if self.config.verbose { println!("Host {} successfully sent", host) };
                    return self.time
                },
                _ => ()
            }

            self.time += 1;
            if self.config.verbose { println!() }
        }
    }

    fn step(&mut self) -> Option<u32> {
        
        // if its time to transmit, then do so
        let mut transmitting: Vec<&mut EthernetHost> = Vec::new();

        for host in self.network.get_hosts().iter_mut() {
            if host.get_next_time_to_send() == self.time {
                if self.config.verbose { println!("Host {} is transmitting", host.get_identifier()) };
                transmitting.push(host);
            }
        }

        match transmitting.len() {
            1 => Some(transmitting.get(0).expect("should exist").get_identifier()),
            0 => {
                if self.config.verbose { println!("Nothing happens") };
                None
            },
            _ => {
                // detect collisions
                for host in transmitting.iter_mut() {
                    if self.config.verbose { println!("Collision w/ Host {}! ", host.get_identifier()) };
                    host.increment_collision();
                    host.calculate_next_time_to_send(self.time, self.config);
                }

                None
            }
        }
    }
}


pub fn run_network_simulation(num_trials: u32, num_hosts: u32, config: &mut Config) -> f32 {

    let mut average_time: f32 = 0.0;

    for _ in 0..num_trials {
        let net = build_network(num_hosts);
        let mut simulation = NetworkSimulation::new(config, net);

        average_time += simulation.start_simulation() as f32;
    }

    average_time / num_trials as f32
}