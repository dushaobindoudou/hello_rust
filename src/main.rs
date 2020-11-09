mod config;

mod routes;

fn main() {
    config::pub_fn();
    config::config::pub_fn();
    config::config::nested::public_function_in_config();
    println!("Hello, world routes! {}", routes::home);
}
