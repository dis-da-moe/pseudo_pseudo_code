use chumsky::prelude::*;

#[derive(Debug)]
enum Expr {
    Num(f64),
    Var(String),

    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>),
    Let {
        name: String,
        rhs: Box<Expr>,
        then: Box<Expr>
    },
    Fn {
        name: String,
        args: Vec<String>,
        body: Box<Expr>,
        then: Box<Expr>
    }
}

fn parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    let ident = text::ident().padded();

    let expression = recursive(|expr| {
        let int = text::int(10)
            .map(|s: String| Expr::Num(s.parse().unwrap()))
            .padded();

        let call = ident.then(expr.clone()
            .separated_by(just(','))
            .allow_trailing()
            .delimited_by(just('('), just(')'))
        ).map(|(f, args)| Expr::Call(f, args));

        let atom = int
            .or(expr.delimited_by(just('('), just(')')))
            .or(call)
            .or(ident.map(Expr::Var));

        let op = |c| just(c).padded();

        let unary = op('-')
            .repeated()
            .then(atom)
            .foldr(|_, right_side| Expr::Neg(Box::new(right_side)));

        let product = unary.clone()
            .then(op('*').to(Expr::Mul as fn(_, _) -> _)
                .or(op('/').to(Expr::Div as fn(_, _) -> _))
                .then(unary)
                .repeated())
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let sum = product.clone()
            .then(op('+').to(Expr::Add as fn(_, _) -> _)
                .or(op('-').to(Expr::Sub as fn(_, _) -> _))
                .then(product)
                .repeated()
            ).foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        sum
    });

    let declare = recursive(|decl| {
        let p_let = text::keyword("let")
            .ignore_then(ident)
            .then_ignore(just('='))
            .then(expression.clone())
            .then_ignore(just(';'))
            .then(decl.clone())
            .map(|((name, rhs), then)| Expr::Let {
                name,
                rhs: Box::new(rhs),
                then: Box::new(then)
            });

        let p_fn = text::keyword("fn")
            .ignore_then(ident)
            .then(ident.repeated())
            .then_ignore(just('='))
            .then(expression.clone())
            .then_ignore(just(';'))
            .then(decl)
            .map(|(((name, args), body), then)| Expr::Fn {
                name,
                args,
                body: Box::new(body),
                then: Box::new(then)
            });

        p_let.or(p_fn).or(expression).padded()
    });

    declare.then_ignore(end())
}

fn eval<'a>(
    expr: &'a Expr,
    vars: &mut Vec<(&'a String, f64)>,
    funcs: &mut Vec<(&'a String, &'a [String], &'a Expr)>,
) -> Result<f64, String> {
    match expr {
        Expr::Num(x) => Ok(*x),
        Expr::Neg(a) => Ok(-eval(a, vars, funcs)?),
        Expr::Add(a, b) => Ok(eval(a, vars, funcs)? + eval(b, vars, funcs)?),
        Expr::Sub(a, b) => Ok(eval(a, vars, funcs)? - eval(b, vars, funcs)?),
        Expr::Mul(a, b) => Ok(eval(a, vars, funcs)? * eval(b, vars, funcs)?),
        Expr::Div(a, b) => Ok(eval(a, vars, funcs)? / eval(b, vars, funcs)?),
        Expr::Var(name) => if let Some((_, val)) = vars.iter().rev().find(|(var, _)| *var == name) {
            Ok(*val)
        } else {
            Err(format!("Cannot find variable `{}` in scope", name))
        },
        Expr::Let { name, rhs, then } => {
            let rhs = eval(rhs, vars, funcs)?;
            vars.push((name, rhs));
            let output = eval(then, vars, funcs);
            vars.pop();
            output
        },
        Expr::Call(name, args) => if let Some((_, arg_names, body)) = funcs
            .iter()
            .rev()
            .find(|(var, _, _)| *var == name)
            .copied()
        {
            if arg_names.len() == args.len() {
                let mut args = args
                    .iter()
                    .map(|arg| eval(arg, vars, funcs))
                    .zip(arg_names.iter())
                    .map(|(val, name)| Ok((name, val?)))
                    .collect::<Result<_, String>>()?;
                vars.append(&mut args);
                let output = eval(body, vars, funcs);
                vars.truncate(vars.len() - args.len());
                output
            } else {
                Err(format!(
                    "Wrong number of arguments for function `{}`: expected {}, found {}",
                    name,
                    arg_names.len(),
                    args.len(),
                ))
            }
        } else {
            Err(format!("Cannot find function `{}` in scope", name))
        },
        Expr::Fn { name, args, body, then } => {
            funcs.push((name, args, body));
            let output = eval(then, vars, funcs);
            funcs.pop();
            output
        },
    }
}

fn main() {
    let source = include_str!("test.foo");

    let parsed = parser().parse(source);
    match parsed {
        Ok(ast) => {
            let evaluated = eval(&ast, &mut Vec::new(), &mut Vec::new());
            match evaluated {
                Ok(output) => println!("{}", output),
                Err(error) => println!("Evaluation error: {}", error)
            }
        },
        Err(error) => error
            .into_iter()
            .for_each(|err| println!("Parse error: {}", err))
    }
}