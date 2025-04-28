// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        //有三种方法
        //方法一
        //将println!("The longest string is '{}'", result);放进里面
        //方法二
        //调整生命周期备注
        // 允许不同生命周期的参数
        // fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str 
        // where
        //     'b: 'a // 要求'b生命周期至少和'a一样长
        // {
        //     if x.len() > y.len() { x } else { y }
        // }
        //方法三
        //使用所有权替代引用
        // fn longest(x: String, y: String) -> String {
        //     if x.len() > y.len() { x } else { y }
        // }
        
        // fn main() {
        //     let string1 = String::from("long string is long");
        //     let result;
        //     {
        //         let string2 = String::from("xyz");
        //         result = longest(string1, string2); // 移交所有权
        //     }
        //     println!("The longest string is '{}'", result);
        // }
        println!("The longest string is '{}'", result);
    }
    
}
