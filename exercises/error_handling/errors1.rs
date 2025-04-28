// errors1.rs
//
// This function refuses to generate text to be printed on a nametag if you pass
// it an empty string. It'd be nicer if it explained what the problem was,
// instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Result` that can be used to express error conditions. Let's use
// it!
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.

pub fn generate_nametag_text(name: String) -> Result<String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        Err("`name` was empty; it must be nonempty.".into())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            // Ok("Hi! My name is Beyoncé".into())
            Some("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
        //明显需要错误信息，用Result
        //方法1，也就是改变option为Result
        //再用Err  OK就可以了
        //方法2
        //也可以用expect，保持Option，不过得改变以下的
        //保持Some输出
        //Ok("Hi! My name is Beyoncé".into())变为
        //Some("Hi! My name is Beyoncé".into())
        //在加上以下修改这里的改为一句话
        // fn explains_why_generating_nametag_text_fails() {
        //     generate_nametag_text("".into()).expect("empty name");//就是这个
        // }
    }
}
