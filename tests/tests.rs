use expression_formatter::format;

#[test]
fn test() {
    let value = format!("expressions: {1 + 2} and {3 + 4} wow!");
    println!("{}", value);
}
