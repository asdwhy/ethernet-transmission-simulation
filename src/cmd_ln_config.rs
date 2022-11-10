use std::env;
use std::cmp::Ordering;
use rand_pcg::{Pcg32, Lcg64Xsh32};
use rand::{SeedableRng};

pub struct Config {
    pub verbose: bool,
    pub rng: Option<Lcg64Xsh32> // seeded random number generator
}

// gets the number of trials from the command line arguments
pub fn parse_args() -> (u32, Vec<u32>, Config) {
    fn usage(message: &str) {
        if message.len() > 0 { eprintln!("{message}") };
        eprintln!("Usage: ./eth_transmission_simulation [OPTIONS...] <num_trials> <num_hosts>...");
        eprintln!("example: ./eth_transmission_simulation -s 0 100 20 60 100");
        eprintln!("\tindicates 100 trials of 20 host network, 60 host network, and 100 host network. PRNG with seed 0.");
        eprintln!("Options:");
        eprintln!("\t-v\t\tverbose");
        eprintln!("\t-s <seed>\tunsigned 64 bit seed");
        std::process::exit(1);
    }

    let mut args: Vec<String> = env::args().collect();

    let mut config = Config {
        verbose: false,
        rng: None
    };
    
    // extract options
    let mut flagged = Vec::<usize>::new();
    for i in 0..args.len() {
        if let Ordering::Equal = &args[i].cmp(&String::from("-v")) {
            config.verbose = true;
            flagged.push(i);
        } else if let Ordering::Equal = &args[i].cmp(&String::from("-s")) {
            if i >= args.len() -1 { usage("Error: Seed flag present but seed not specified") };

            match &args[i+1].parse::<u64>() {
                Ok(seed) => {
                    config.rng = Some(Pcg32::seed_from_u64(*seed));
                    flagged.push(i);
                    flagged.push(i + 1);
                },
                Err(_) => usage("Error: Could not parse seed as unsigned 64 bit integer")
            }
        }
    }

    // remove flags so parsing is easier
    for i in 0..flagged.len() {
        args.remove(flagged[i]-i);
    }

    // extract arguments
    if args.len() < 3 {
        usage("");
    }

    let num_trials = *(&args[1].parse::<u32>().unwrap_or(0));
    if num_trials == 0 {
        usage("Error: Invalid arguments");
    }

    let mut simulation_configs = Vec::<u32>::new();

    for i in 2..(args.len()) {
        let num_hosts = *(&args[i].parse::<u32>().unwrap_or(0));

        if num_hosts == 0 {
            usage("Error: Invalid arguments");
        }

        simulation_configs.push(num_hosts);
    }

    return (num_trials, simulation_configs, config);
}