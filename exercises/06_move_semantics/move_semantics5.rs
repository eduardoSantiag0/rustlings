#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
//* O & indica que a função recebe uma referência, e não assume a posse da String.
fn get_char(data: &str) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
//* mut data: String
//* String: recebe o valor por ownership, transferindo a posse.
//* mut: permite modificar a variável local data.
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    //* Empresta o valor mas main continua como dono
    get_char(&data);

    string_uppercase(data);
    //* Data é apagada, pois transferiu a ownsership para a função acima
}
