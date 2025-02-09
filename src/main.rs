mod lexer;
use lexer::functions;

fn main() {
    let f = functions::Functions::new(true);
    // Functions構造体を使用できることを確認
    println!("Function created successfully!");
}