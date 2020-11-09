pub mod config {
  // 一个公有的结构体，带有一个公有的字段（类型为泛型 `T`）
  pub struct OpenBox<T> {
    pub contents: T,
  }

  // 一个公有的结构体，带有一个私有的字段（类型为泛型 `T`）    
  #[allow(dead_code)]
  pub struct ClosedBox<T> {
    contents: T,
  }

  impl<T> ClosedBox<T> {
    // 一个公有的构造器方法
    pub fn new(contents: T) -> ClosedBox<T> {
      ClosedBox {
        contents: contents,
      }
    }
  }
  // 私有函数
  fn private_fn() {
    println!("called `my_mod::private_function()`");
  }
  // 公共函数
  pub fn pub_fn() {
    println!("show pub_fn")
  }
  // 模块也可以嵌套
  pub mod nested {
    pub fn function() {
      println!("called `my_mod::nested::function()`");
    }

    #[allow(dead_code)]
    fn private_function() {
      println!("called `my_mod::nested::private_function()`");
    }

    // 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见。
    // `path` 必须是父模块（parent module）或祖先模块（ancestor module）
    // 真奇怪
    pub fn public_function_in_config() {
      print!("called `my_mod::nested::public_function_in_my_mod()`, that\n > ");
      public_function_in_nested()
    }

    // 使用 `pub(self)` 语法定义的函数则只在当前模块中可见。
    pub(self) fn public_function_in_nested() {
      println!("called `my_mod::nested::public_function_in_nested");
    }

    // 使用 `pub(super)` 语法定义的函数只在父模块中可见。
    pub(super) fn public_function_in_super_mod() {
      println!("called my_mod::nested::public_function_in_super_mod");
    }
  }
}


// 公共函数
pub fn pub_fn() {
  println!("show pub_fn")
}