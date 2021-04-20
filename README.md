# scanrs
A simple port scanner made in rust

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)

# Installation
```
git clone https://github.com/krishpranav/scanrs
cd scanrs
chmod +x *
cargo install
rust src/main.rs
```


# Example
```rust

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;

    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                // Found open port, indicate progress and send to main thread
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        // Break loop when out of ports
        if (MAX - port) <= num_threads {
            break;
        }

        port += num_threads;
    }
}

```