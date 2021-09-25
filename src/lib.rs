pub mod builtin_functions;
pub mod default_env;
pub mod eval;
pub mod parser;
pub mod runner;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::{default_env::default_env, eval::parse_and_eval};

    #[test]
    fn iile() {
        /*
         * Immediately invoked lambda expression
         */
        let mut env = default_env();
        let expr = "((lambda (x) (+ x 10)) 10)".to_string();
        let eval_expr = parse_and_eval(expr, &mut env).unwrap();
        assert_eq!(format!("{}", eval_expr), "20");
    }

    #[test]
    fn list_test() {
        let mut env = default_env();
        let list_fn = "(list 1 2 3)".to_string();
        let ans = parse_and_eval(list_fn, &mut env).unwrap();
        assert_eq!(format!("{}", ans), "(1 2 3)");

        let mapper = "(map (lambda (x) (+ x x)) '(1 2 3 4))".to_string();
        let ans = parse_and_eval(mapper, &mut env).unwrap();
        assert_eq!(format!("{}", ans), "(2 4 6 8)");
    }

    #[test]
    fn square() {
        let square_fn = "(define square (lambda (x) (* x x)))";
        let mut env = default_env();
        parse_and_eval(square_fn.to_string(), &mut env).unwrap();
        let sq_10 = parse_and_eval("(square 10)".to_string(), &mut env).unwrap();
        assert_eq!(format!("{}", sq_10), "100");
        let sq = parse_and_eval("(square 5.5)".to_string(), &mut env).unwrap();
        assert_eq!(format!("{}", sq), "30.25");
    }

    #[test]
    fn fact() {
        let mut env = default_env();
        let fact_fn = "(define fact (lambda (n) (if (<= n 1) 1 (* n (fact (- n 1))))))".to_string();
        parse_and_eval(fact_fn, &mut env).unwrap();
        let fact_10 = parse_and_eval("(fact 10)".to_string(), &mut env).unwrap();
        assert_eq!(format!("{}", fact_10), "3628800");
        let fact_0 = parse_and_eval("(fact 0)".to_string(), &mut env).unwrap();
        assert_eq!(format!("{}", fact_0), "1");
        let fact_20 = parse_and_eval("(fact 20)".to_string(), &mut env).unwrap();
        assert_eq!(format!("{}", fact_20), "2432902008176640000");
    }

    #[test]
    fn fib() {
        let mut env = default_env();
        let fib_fn = "(define (fib x) (
                if (<= x 1) x  
                (+ (fib (- x 1)) (fib (- x 2)))
                ))"
        .to_string();
        parse_and_eval(fib_fn, &mut env).unwrap();
        let fib_2 = parse_and_eval("(fib 2)".to_string(), &mut env).unwrap();
        assert_eq!(format!("{}", fib_2), "1");
        let fib_10 = parse_and_eval("(fib 10)".to_string(), &mut env).unwrap();
        assert_eq!(format!("{}", fib_10), "55");
    }

    #[test]
    fn test() {}
}
