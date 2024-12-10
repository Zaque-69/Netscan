# Netscan

A simple and efficient application written in Rust for scanning ports on various IP addresses. This tool allows you to check which ports are open for a given IP address or hostname and provides flexibility to scan specific ports or ranges of ports.

## Features

- Scan multiple ports for a single IP address or hostname.
- Support for both TCP and UDP protocols.
- Customizable port ranges.
- Clear and concise output of open ports.
- Fast and lightweight implementation using Rust's concurrency model.

## Installation
```bash
git clone https://github.com/Zaque/Netscan.git
```

### Prerequisites
- Ensure you have **Rust** installed on your system. If not, you can install it using **cargo**:


### Clone and Build

1. Clone the repository:
   ```bash
   git clone https://github.com/Zaque/Netscan.git
   cd Netscan-main
   ```

2. Run the binary:
   ```bash
   cargo run #PORT_CATEGORY http://142.250.179.164
   ```

## Usage

### Command-Line Arguments

The application accepts the following arguments:

-  **IP Address or Hostname**: The target to scan.
- **Protocol**: Specify `tcp`, `udp`, and others.

### Examples

#### Scan Common TCP Ports on an IP
```bash
cargo run TCP http://142.250.179.164
```

#### Scan Specific UDP Ports
```bash
cargo run UDP https://142.250.179.164
```

#### Scan a Range of all Ports
```bash
cargo run ALL 142.250.179.164
```

### Output
The application will display open ports in a readable format, e.g.:
```
Scanning http://142.250.179.164...
Successfully connected to http://142.250.179.164:80
Error connecting to http://142.250.179.164:443: error sending request for url (http://142.250.179.164:443/)
...
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.