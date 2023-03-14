use std::io;
fn main() {
    let mut name = String::new();

    println!("Введите имя");

    match io::stdin().read_line(&mut name) {
        Ok(_) => {
            println!("Name: {name}")
        },
        Err(error) => {
            println!("Ошибка программы - {error}")
        }
    }
}
