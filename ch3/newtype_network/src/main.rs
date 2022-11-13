struct Hostname(String);

type HostString = String;

fn connect(host: Hostname) {
    println!("connected to: {}", host.0);
}

fn connect_str(host: HostString) {
    println!("connected to string: {}", host);
}

fn main() {
    let ordinary_string = String::from("localhost");
    let host = Hostname(ordinary_string.clone());
    connect(host);
    connect_str(ordinary_string);
}
