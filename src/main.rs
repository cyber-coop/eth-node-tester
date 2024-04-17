use postgres::{Client, NoTls};
use std::net::TcpStream;
use core::time::Duration;

fn main() {
    let mut client = Client::connect("host=localhost user=postgres password=wow dbname=blockchains", NoTls).unwrap();

    let timeout_duration = Duration::from_millis(3000);

    for row in client.query("SELECT * FROM discv4.nodes", &[]).unwrap() {
        let address: String = row.get(0);
        let tcp_port: i32 = row.get(1);
        let _udp_port: i32 = row.get(2);
        let _node_id: Vec<u8> = row.get(3);
        
        let socket_address = format!("{}:{}", address, tcp_port).parse().unwrap();

        match TcpStream::connect_timeout(&socket_address, timeout_duration) {
            Ok(_) => {
                println!("{} on port {} is working", address, tcp_port);
            }
            Err(_) => {
                println!("{} on port {} is NOT WORKING, deleting from database...", address, tcp_port);
                if let Err(err) = client.execute("DELETE FROM discv4.nodes WHERE address = $1 AND tcp_port = $2", &[&address, &tcp_port]) {
                    eprintln!("Failed to delete row: {}", err);
                }
            }
        }
    }
}