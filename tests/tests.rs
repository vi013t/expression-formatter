use expression_formatter::println;

#[test]
fn test() {
    let value = println!(r#"{if 1 == 2 { 1 } else { 2 }}"#);
}
