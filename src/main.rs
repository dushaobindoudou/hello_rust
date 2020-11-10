mod config;

mod routes;

fn main() {
    config::pub_fn();
    config::config::pub_fn();
    config::config::nested::public_function_in_config();
    let homeRoute = "this is home route";
    let rts = routes::Routes::new(homeRoute);
    let rtsi = routes::Routes::intoNew("this is into new".to_string());
    let rtsic = routes::Routes::cIntoNew("c this is into new".to_string());
    println!("Hello, world routes! \n \n {}, \n {}", rts.home, rtsi.home);
    println!("rtsic: {}", rtsi.home);
}
