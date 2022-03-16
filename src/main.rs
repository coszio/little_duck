
mod ast;
#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub little_duck); // synthesized by LALRPOP

fn main() {
    println!("Hello, world!");
}


#[test]
fn hello_world() {
    let good_str = "program myProgram; 
    { 
        print(cte.string); 
    }";
    let bad_str1 = "program myProgram { print(cte.string); }";
    let bad_str2 = "program myProgram; { print(cte.string) }";
    
    assert!(little_duck::PROGRAMAParser::new().parse(good_str).is_ok());
    assert!(little_duck::PROGRAMAParser::new().parse(bad_str1).is_err());
    assert!(little_duck::PROGRAMAParser::new().parse(bad_str2).is_err());
}

#[test]
fn condicion() {
    let test_str = "program myProgram;
    {
        if (a > b) {
            a = b;
        } else {
            a = c;
        };
    }";

    let bad_str1 = "program myProgram;
    {
        if (a > b) {
            a = b;
        } else {
            a = c;
        }
    }";

    let bad_str2 = "program myProgram;
    {
        if a > b {
            a = b;
        } else {
            a = c;
        };
    }";
    assert!(little_duck::PROGRAMAParser::new().parse(test_str).is_ok());
    assert!(little_duck::PROGRAMAParser::new().parse(bad_str1).is_err());
    assert!(little_duck::PROGRAMAParser::new().parse(bad_str2).is_err());
}


#[test]
fn instanciar_variables() {
    let good_str1 = "program myProgram; 
    var a: int;
    { 
        print(cte.string); 
    }";
    
    let good_str2 = "program myProgram; 
    var b: float;
    { 
        print(cte.string); 
    }";

    
    
    let bad_str1 = "program myProgram { print(cte.string); }";
    let bad_str2 = "program myProgram: { print(cte.string) }";
    
    assert!(little_duck::PROGRAMAParser::new().parse(good_str1).is_ok());
    assert!(little_duck::PROGRAMAParser::new().parse(good_str2).is_ok());
    assert!(little_duck::PROGRAMAParser::new().parse(bad_str1).is_err());
    assert!(little_duck::PROGRAMAParser::new().parse(bad_str2).is_err());
}