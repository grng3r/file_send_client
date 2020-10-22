A client to send the contents of a file over TCP or UDP in rust. This implementation works for an local test. To use it in production the loopback address should be changed to the local ip address, 
also there should be a specific console.log (not provided) in the same location as main.rs, that can be changed according to requirements. 
Build with cargo build --release.
Usage: a) TCP: <executablename> <t(tcp)> <portnumber>
       b) UDP: <executablename> <u(udp)> <portnumber> <receiver IP>

