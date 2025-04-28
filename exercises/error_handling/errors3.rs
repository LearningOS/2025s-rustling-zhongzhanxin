// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.


use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError>{
    //在 main() 函数中直接使用了 ? 操作符，但 main() 默认不返回 Result 类型。
    //方法一 让main()函数返回Result
    //fn main() -> Result<(), ParseIntError>{}
    //中间保留？，最后OK(())返回
    //方法二 用match处理Result
    // match total_cost(pretend_user_input) {
    //     Ok(cost) => {
    //         if cost > tokens {
    //             println!("You can't afford that many!");
    //         } else {
    //             tokens -= cost;
    //             println!("You now have {} tokens.", tokens);
    //         }
    //     }
    //     Err(e) => {
    //         println!("Error: {}", e);
    //     }
    // }
    //方法三 使用unwarp或者expect
    //let cost = total_cost(pretend_user_input).expect("输出不了");
    //let cost = total_cost(pretend_user_input).unwarp();
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
