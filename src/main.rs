use std::io::stdin;

fn main() {
    let mut nome_do_usuario = String::new();

    // recebendo input do usuário
    stdin().read_line(&mut nome_do_usuario).unwrap();

    println!("--- BEM VINDO ---");
    println!("{nome_do_usuario}")
}
