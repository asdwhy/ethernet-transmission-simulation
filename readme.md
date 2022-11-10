## Ethernet Transmission Simulation

This program will simulate the following situation,

Consider $N$ hosts that are waiting for another packet to finish on an Ethernet network. All the stations (hosts) transmit at once when the packet is finished and collide.

Determine how long it will take for a single host to successfully transmit a frame on the Ethernet network.

The following simplifications are made, ignore interframe spacing, ignore variability in collision times (so that retransmission is always an exact integral multiple of 51.2 $\mu\text{s}$ slot time), and assume that each collision uses up exactly one time slot.

Time is modelled as an integer $T$, in units of time slots of time steps, and collisions take one slot time (so a collision at time $T$ followed by a backoff of $k=0$ will result in a retransmission at $T+1$).


## Compiling
It is recommended to use the rust package manager ```cargo``` to compile the project. See this [tutorial](https://doc.rust-lang.org/book/ch01-01-installation.html) to install cargo. You can simply compile and run the project using ```cargo run``` from the directory where $\verb|cargo.toml|$ is located.

A pre-compiled binary for Ubuntu already included.

## Running the program
```
$ ./eth_transmission_simulation 1 5
Average time for 5 hosts is 4
```

```
$ ./eth_transmission_simulation 1 100000
Average time for 5 hosts is 5.19612
```

Keep in mind these value are in time units, if the time slice is $51.2\mu\text{s}$ then multiply the result by this to get the actual time. In this example the actual time would for 5 hosts would converge to around $5.19612\times 51.2\mu\text{s}=266.041344\mu\text{s}$.
