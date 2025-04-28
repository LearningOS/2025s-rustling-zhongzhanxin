// errors2.rs
//
// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy, and
// the `total_cost` function will calculate the total cost of the tokens. Since
// the player typed in the quantity, though, we get it as a string-- and they
// might have typed anything, not just numbers!
//
// Right now, this function isn't handling the error case at all (and isn't
// handling the success case properly either). What we want to do is: if we call
// the `parse` function on a string that is not a number, that function will
// return a `ParseIntError`, and in that case, we want to immediately return
// that error from our function and not try to multiply and add.
//
// There are at least two ways to implement this that are both correct-- but one
// is a lot shorter!
//
// Execute `rustlings hint errors2` or use the `hint` watch subcommand for a
// hint.


use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;
    //字符转换成数字太牛了
    //这里不能用expect()，因为当输入无效时会直接panic，无法返回 ParseIntError
    //会改变函数契约（从返回错误变为panic）
    //第二个测试会失败（因为期待返回 Err 但实际panic了）
    //用？的原因如图
// 特性	       ?操作符	            .expect()
// 错误处理	 传播错误（返回Err）	 直接panic（崩溃程序）
// 使用场景	 希望调用者处理错误时	 确定不应该出错或原型开发时
// 错误信息	 保留原始错误	        使用自定义信息
// 返回类型	 保持原Result类型	    成功时解包值，失败时panic
    //而且后面有.unwrap_err().to_string()，就不用检查了
    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
