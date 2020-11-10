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
