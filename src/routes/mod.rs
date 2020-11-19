pub struct Routes {
  pub home: String,
}

pub struct Keys {
}

impl Keys {
  pub fn all() -> String {
    let my = "all in one";
    my.to_string()
  }
}

impl Routes {
  pub fn new(home: &str) -> Routes {
    Routes {
      home: home.to_string(),
    }
  }

  pub fn intoNew<T: Into<String>>(into_home: T) -> Routes {
    Routes {
      home: into_home.into(),
    }
  }


  pub fn cIntoNew<T>(c_home:T) -> Routes where T:Into<String> {
    Routes {
      home: c_home.into(),
    }
  }
}


pub struct Word<'a> {
  pub name: &'a str,
}

impl<'b> Word<'b> {
  pub fn show(&self) {
    println!("打印当前的名称：{}", self.name);
  }
}


pub fn setRoute<T: Into<String> + std::fmt::Display>(route: T) -> Routes {
  println!("current set route: {}", route);
  Routes::cIntoNew(route)
}

