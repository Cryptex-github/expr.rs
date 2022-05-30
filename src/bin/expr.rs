use expr_rs::eval;

fn main() {
    let src = std::env::args().skip(1).collect::<String>();

    println!(
        "{}",
        match eval(&src) {
            Ok(res) => res.to_string(),
            Err(e) => e,
        }
    );
}
