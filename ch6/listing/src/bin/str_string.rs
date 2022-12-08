// fn is_string(password: String) -> bool {
//     password.len() > 5
// }

fn is_string<T: AsRef<str>>(password: T) -> bool {
    password.as_ref().len() > 5
}

fn main() {
    let pw = "justok";
    is_string(pw);
    let str2 = "abcdefg".to_string();
    is_string(str2);
}
