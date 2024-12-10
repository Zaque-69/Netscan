use reqwest::blocking::{ Response, Client };
use std::env;
use std::error::Error;

// Collecting the argument and return the URL
fn arguments() -> Result<Vec<String>, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err("You don't have 3 arguments!".into());
    }

    Ok(args)
}

// Checking if the URL have "http://"
fn check_http_url(url : String) -> String{
    let mut address = url.to_string();
    if ! url.contains("http") {
        address = format!("http://{}", url);
    }
    if url.contains("https") { 
        address = url.replace("https", "http");
    }
    address
}

// Sending the request
fn send_request(url: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let http_client: Client = Client::new();
    let request = http_client.get(url).send()?;
    
    Ok(request)
}
// -> Result<(), Box<dyn std::error::Error>>
fn specific_port_request(url : &str, port_list : Vec<&str>)  {
    for port in port_list{
        let address = check_http_url(format!("{}:{}", url, port));
        let request = send_request(&address);
        
        match request {
            Ok(_) => println!("Successfully connected to {}", address),
            Err(ref e) => eprintln!("Error connecting to {}: {}", address, e),
        }
    }
}
fn main(){
    let args : Vec<String> = arguments().unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        std::process::exit(1); 
    });
    let url: &_ = &args[2];
    
    println!("Scanning {}...", {url});
    
    match args[1].as_str() {
        "N" => {
            let n_ports: [&str; 5] = ["80", "443", "22", "8080", "3306"];
            specific_port_request(url, n_ports.to_vec());
        },
        "UDP" => {
            let udp_ports: [&str; 5] = ["53", "67", "111", "24", "162"];
            specific_port_request(url, udp_ports.to_vec());
        },
        "TCP" => {
            let tcp_ports: [&str; 7] = ["22", "25", "53", "70", "80", "113", "443"];
            specific_port_request(url, tcp_ports.to_vec());
        },
        "ALL" => {
            let all_ports: [&str; 18] = ["80", "443", "22", "8080", "3306", "22", "53", "67", "111", "24", "162", "22", "25", "53", "70", "80", "113", "443"];
            specific_port_request(url, all_ports.to_vec());
        },
        _ => {
            let ports: [&str; 5] = ["443", "443", "22", "8080", "3306"];
            specific_port_request(url, ports.to_vec());
        }
    }
}