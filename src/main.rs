mod config;

mod routes;

mod cal;

mod zero;



fn main() {
    config::pub_fn();
    config::config::pub_fn();
    config::config::nested::public_function_in_config();
    let homeRoute = "this is home route";
    let rts = routes::Routes::new(homeRoute);
    let rtsi = routes::Routes::intoNew("this is into new".to_string());
    let rtsic = routes::Routes::cIntoNew("c this is into new".to_string());

    let word = routes::Word{name: "i am the new best"};

    // let mut count = 0;
    // let total = 4999999;

    // let mut nums = vec![false; total];

    // // nums.push(1);
    // // nums.push(2);
    // // nums.push(3);
    // let mut i = 2;

    // while i < total {
    //     if !nums[i] {
    //         count += 1;
    //         for j in (i * 2..total).step_by(i) {
    //             nums[j] = true;
    //         }
    //     }
    //     i += 1;
    // }

    // for num in 2..total {
    //     // let mut isPrems = true;
    //     if !num {
    //         count += 1;

    //     }
    //     for val in nums.iter() {
    //     //     if num % val == 0 {
    //     //         isPrems = false;
    //     //         // println!("这不是一个素数：{}/{}", num, val);
    //     //         break;
    //     //     }
    //     }
    //     if isPrems {
    //         nums.push(num);
    //         count += 1;
    //     }
    // }
    // println!("当前的素数: {}", count as i32);
    // let num = -405220;
    // let resNum = cal::t_t_s(num, 7);

    // println!("把数字：{} 转换为7进制: {}", num, resNum);

    let rt = routes::setRoute("basic/home");

    // println!("set route: {:?}", nums);
    println!("set route: {}", rt.home);

    let fc = zero::how_much(101);

    println!("total have: {} 5", fc);

    println!("Hello, world routes! \n \n {}, \n {}", rts.home, rtsi.home);
    println!("rtsic: {}", rtsi.home);
    word.show();
}
