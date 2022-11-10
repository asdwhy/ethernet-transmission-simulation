// use rand::{Rng, SeedableRng, rngs::StdRng};
// use rand_pcg::Pcg32;
use rand::Rng;
use crate::cmd_ln_config::Config;

pub struct EthernetHost {
    identifier: u32,
    collision_count: u32,
    next_time_to_send: u32
}

impl EthernetHost {
    pub fn new(id: u32) -> EthernetHost {
        EthernetHost {
            identifier: id,
            collision_count: 0,
            next_time_to_send: 0,
        }
    }

    pub fn calculate_next_time_to_send(&mut self, time: u32, config: &mut Config) {
        
        let k = if self.collision_count == 0 {0} else {
            match &mut config.rng {
                Some(rng) => rng.gen_range(0..self.collision_count),
                None => rand::thread_rng().gen_range(0..self.collision_count)
            }
        };

        let backoff = u32::pow(2, k as u32) - 1;

        self.next_time_to_send = time + backoff + 1;
        if config.verbose { println!("Host {} next time to send is {}", self.identifier, self.next_time_to_send) };
    }

    pub fn get_next_time_to_send(&self) -> u32 {
        return self.next_time_to_send;
    }

    pub fn increment_collision(&mut self) {
        self.collision_count += 1;
    }

    pub fn get_identifier(&self) -> u32 {
        self.identifier
    }

}

