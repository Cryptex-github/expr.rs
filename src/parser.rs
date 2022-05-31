use crate::ast::Token;
use chumsky::prelude::*;

pub fn parser() -> impl Parser<char, Token, Error = Simple<char>> {
    recursive(|expr| {
        let int = text::int(10).map(Token::Number).padded();

        let float = text::int::<_, Simple<char>>(10)
            .then_ignore(just('.'))
            .then(text::int(10))
            .map(|(s, d)| Token::Float(format!("{}.{}", s, d)));

        let fact = float
            .or(int)
            .then_ignore(just('!'))
            .map(|n| Token::Factorial(Box::new(n)));

        let atom = fact
            .or(float)
            .or(int)
            .or(expr.delimited_by(just('('), just(')')))
            .padded();

        let op = |c| just(c).padded();

        let unary = op('-')
            .repeated()
            .then(atom)
            .foldr(|_op, rhs| Token::Neg(Box::new(rhs)));

        let exp = unary
            .clone()
            .then(
                op('^')
                    .to(Token::Pow as fn(_, _) -> _)
                    .or(op('%').to(Token::Mod as fn(_, _) -> _))
                    .then(unary)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let product = exp
            .clone()
            .then(
                op('*')
                    .to(Token::Mul as fn(_, _) -> _)
                    .or(op('/').to(Token::Div as fn(_, _) -> _))
                    .then(exp)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        product
            .clone()
            .then(
                op('+')
                    .to(Token::Add as fn(_, _) -> _)
                    .or(op('-').to(Token::Sub as fn(_, _) -> _))
                    .then(product)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
    })
    .then_ignore(end())
}
