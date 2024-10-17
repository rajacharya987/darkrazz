mod networking;
mod encryption;
mod routing;

fn main() {
    networking::initialize_network();
    encryption::encrypt_message();
    routing::route_message();
    println!("DarkRazz initialized!");
}

