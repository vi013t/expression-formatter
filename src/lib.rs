use proc_macro::TokenStream;
use syn::{Expr, LitStr};

#[proc_macro]
pub fn format(tokens: TokenStream) -> TokenStream {
    let string = syn::parse_macro_input!(tokens as LitStr).value();

    let mut format_string = String::new();
    let mut expressions: Vec<Expr> = Vec::new();
    let mut start_index = 0;

    while start_index < string.len() {
        // Bad opening brace
        if string.chars().nth(start_index).unwrap() == '}' {
            panic!("Unexpected closing brace");
        }

        // Normal character
        if string.chars().nth(start_index).unwrap() != '{' {
            format_string += &string.chars().nth(start_index).unwrap().to_string();
            start_index += 1;
            continue;
        }

        // Opening brace
        start_index += 1;

        // Expression
        let (expression, length) = parse_expr(&string, start_index, string.len() - 1);
        start_index += length;

        while string.chars().nth(start_index).unwrap() != '}' {
            start_index += 1;
        }

        // Closing brace
        if string.chars().nth(start_index).unwrap() != '}' {
            panic!("Missing closing brace after expression");
        }
        start_index += 1;

        // Add expression
        format_string += "{}";
        expressions.push(expression);
    }

    quote::quote! {
        std::format!(#format_string, #(#expressions),*)
    }
    .into()
}

#[proc_macro]
pub fn println(tokens: TokenStream) -> TokenStream {
    let string = syn::parse_macro_input!(tokens as LitStr).value();

    let mut format_string = String::new();
    let mut expressions: Vec<Expr> = Vec::new();
    let mut start_index = 0;

    while start_index < string.len() {
        // Bad opening brace
        if string.chars().nth(start_index).unwrap() == '}' {
            panic!("Unexpected closing brace");
        }

        // Normal character
        if string.chars().nth(start_index).unwrap() != '{' {
            format_string += &string.chars().nth(start_index).unwrap().to_string();
            start_index += 1;
            continue;
        }

        // Opening brace
        start_index += 1;

        // Expression
        let (expression, length) = parse_expr(&string, start_index, string.len() - 1);
        start_index += length;

        while string.chars().nth(start_index).unwrap() != '}' {
            start_index += 1;
        }

        // Closing brace
        if string.chars().nth(start_index).unwrap() != '}' {
            panic!("Missing closing brace after expression");
        }
        start_index += 1;

        // Add expression
        format_string += "{}";
        expressions.push(expression);
    }

    quote::quote! {
        std::println!(#format_string, #(#expressions),*)
    }
    .into()
}

#[proc_macro]
pub fn print(tokens: TokenStream) -> TokenStream {
    let string = syn::parse_macro_input!(tokens as LitStr).value();

    let mut format_string = String::new();
    let mut expressions: Vec<Expr> = Vec::new();
    let mut start_index = 0;

    while start_index < string.len() {
        // Bad opening brace
        if string.chars().nth(start_index).unwrap() == '}' {
            panic!("Unexpected closing brace");
        }

        // Normal character
        if string.chars().nth(start_index).unwrap() != '{' {
            format_string += &string.chars().nth(start_index).unwrap().to_string();
            start_index += 1;
            continue;
        }

        // Opening brace
        start_index += 1;

        // Expression
        let (expression, length) = parse_expr(&string, start_index, string.len() - 1);
        start_index += length;

        while string.chars().nth(start_index).unwrap() != '}' {
            start_index += 1;
        }

        // Closing brace
        if string.chars().nth(start_index).unwrap() != '}' {
            panic!("Missing closing brace after expression");
        }
        start_index += 1;

        // Add expression
        format_string += "{}";
        expressions.push(expression);
    }

    quote::quote! {
        std::print!(#format_string, #(#expressions),*)
    }
    .into()
}

#[proc_macro]
pub fn eprint(tokens: TokenStream) -> TokenStream {
    let string = syn::parse_macro_input!(tokens as LitStr).value();

    let mut format_string = String::new();
    let mut expressions: Vec<Expr> = Vec::new();
    let mut start_index = 0;

    while start_index < string.len() {
        // Bad opening brace
        if string.chars().nth(start_index).unwrap() == '}' {
            panic!("Unexpected closing brace");
        }

        // Normal character
        if string.chars().nth(start_index).unwrap() != '{' {
            format_string += &string.chars().nth(start_index).unwrap().to_string();
            start_index += 1;
            continue;
        }

        // Opening brace
        start_index += 1;

        // Expression
        let (expression, length) = parse_expr(&string, start_index, string.len() - 1);
        start_index += length;

        while string.chars().nth(start_index).unwrap() != '}' {
            start_index += 1;
        }

        // Closing brace
        if string.chars().nth(start_index).unwrap() != '}' {
            panic!("Missing closing brace after expression");
        }
        start_index += 1;

        // Add expression
        format_string += "{}";
        expressions.push(expression);
    }

    quote::quote! {
        std::eprint!(#format_string, #(#expressions),*)
    }
    .into()
}

#[proc_macro]
pub fn eprintln(tokens: TokenStream) -> TokenStream {
    let string = syn::parse_macro_input!(tokens as LitStr).value();

    let mut format_string = String::new();
    let mut expressions: Vec<Expr> = Vec::new();
    let mut start_index = 0;

    while start_index < string.len() {
        // Bad opening brace
        if string.chars().nth(start_index).unwrap() == '}' {
            panic!("Unexpected closing brace");
        }

        // Normal character
        if string.chars().nth(start_index).unwrap() != '{' {
            format_string += &string.chars().nth(start_index).unwrap().to_string();
            start_index += 1;
            continue;
        }

        // Opening brace
        start_index += 1;

        // Expression
        let (expression, length) = parse_expr(&string, start_index, string.len() - 1);
        start_index += length;

        while string.chars().nth(start_index).unwrap() != '}' {
            start_index += 1;
        }

        // Closing brace
        if string.chars().nth(start_index).unwrap() != '}' {
            panic!("Missing closing brace after expression");
        }
        start_index += 1;

        // Add expression
        format_string += "{}";
        expressions.push(expression);
    }

    quote::quote! {
        std::eprintln!(#format_string, #(#expressions),*)
    }
    .into()
}

fn parse_expr(string: &str, start_index: usize, end_index: usize) -> (Expr, usize) {
    if end_index <= start_index {
        panic!("Failed to parse expression");
    }

    let slice = string.get(start_index .. end_index).unwrap();
    let expression_attempt: Result<Expr, _> = syn::parse_str(slice);
    if let Ok(expression) = expression_attempt {
        (expression, end_index - start_index)
    } else {
        parse_expr(string, start_index, end_index - 1)
    }
}