mod bantam;

use crate::bantam::lexer::Lexer;
use crate::bantam::bantam_parser::BantamParser;

fn main() {
    // Function call.
    let mut passed: i32 = 0;
    let mut failed: i32 = 0;

    test("a()", "a()", &mut passed, &mut failed);
    test("a(b)", "a(b)", &mut passed, &mut failed);
    test("a(b, c)", "a(b, c)", &mut passed, &mut failed);
    test("a(b)(c)", "a(b)(c)", &mut passed, &mut failed);
    test("a(b) + c(d)", "(a(b) + c(d))", &mut passed, &mut failed);
    test("a(b ? c : d, e + f)", "a((b ? c : d), (e + f))", &mut passed, &mut failed);

    // Unary precedence.
    test("~!-+a", "(~(!(-(+a))))", &mut passed, &mut failed);
    test("a!!!", "(((a!)!)!)", &mut passed, &mut failed);

    // Unary and binary predecence.
    test("-a * b", "((-a) * b)", &mut passed, &mut failed);
    test("!a + b", "((!a) + b)", &mut passed, &mut failed);
    test("~a ^ b", "((~a) ^ b)", &mut passed, &mut failed);
    test("-a!",    "(-(a!))", &mut passed, &mut failed);
    test("!a!",    "(!(a!))", &mut passed, &mut failed);

    // Binary precedence.
    test("a = b + c * d ^ e - f / g", "(a = ((b + (c * (d ^ e))) - (f / g)))", &mut passed, &mut failed);

    // Binary associativity.
    test("a = b = c", "(a = (b = c))", &mut passed, &mut failed);
    test("a + b - c", "((a + b) - c)", &mut passed, &mut failed);
    test("a * b / c", "((a * b) / c)", &mut passed, &mut failed);
    test("a ^ b ^ c", "(a ^ (b ^ c))", &mut passed, &mut failed);

    // Conditional operator.
    test("a ? b : c ? d : e", "(a ? b : (c ? d : e))", &mut passed, &mut failed);
    test("a ? b ? c : d : e", "(a ? (b ? c : d) : e)", &mut passed, &mut failed);
    test("a + b ? c * d : e / f", "((a + b) ? (c * d) : (e / f))", &mut passed, &mut failed);

    // Grouping.
    test("a + (b + c) + d", "((a + (b + c)) + d)", &mut passed, &mut failed);
    test("a ^ (b + c)", "(a ^ (b + c))", &mut passed, &mut failed);
    test("(!a)!",    "((!a)!)", &mut passed, &mut failed);

    // Show the results.
    if failed == 0 {
        println!("Passed all {} tests.", passed);
    } else {
        println!("----");
        println!("Failed {} out of {} tests.", failed, (failed + passed));
    }
}

fn test(source: &str, expected: &str, passed: &mut i32, failed: &mut i32) {
    let lexer: Box<Lexer> = Box::new(Lexer::new(source.to_string()));
    let mut parser: BantamParser = BantamParser::new(lexer);

    let result = parser.parse_expression();
    match result {
        Ok(expr) => {
            let mut actual: String = String::new();
            expr.print(&mut actual);

            if actual == expected {
                *passed += 1;
                println!("[OK]: {}   ==>   result: {}", source, expected);
            }
            else {
                *failed += 1;
                println!("[FAIL] Expected: {}", expected);
                println!("         Actual: {}", actual);
            }
        }
        Err(err) => {
            *failed += 1;
            println!("[FAIL] Expected: {}", expected);
            println!("          Error: {}", err);
        }
    }
}
