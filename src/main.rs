mod eth_host;
mod eth_network;
mod eth_network_builder;
mod monte_carlo_simulation_driver;
mod cmd_ln_config;

fn main() {
    let (num_trials, simulations, mut config) = cmd_ln_config::parse_args();
    
    for num_hosts in simulations {
        let average = monte_carlo_simulation_driver::run_network_simulation(num_trials, num_hosts, &mut config);
        
        println!("Average time for {num_hosts} hosts is {average}");
    }
}

