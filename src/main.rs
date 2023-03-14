use std::io;

fn main() {
    let mut a_str = String::new();
    let mut b_str = String::new();
    let mut c_str = String::new();

    println!("Решить квадратное уравнение");

    println!("Введите а:");
    match io::stdin().read_line(&mut a_str) {
        Ok(_) => {},
        Err(error) => {
            println!("Ошибка программы - {error}")
        }
    }

    println!("Введите b:");
    match io::stdin().read_line(&mut b_str) {
        Ok(_) => {},
        Err(error) => {
            println!("Ошибка программы - {error}")
        }
    }

    println!("Введите c:");
    match io::stdin().read_line(&mut c_str) {
        Ok(_) => {},
        Err(error) => {
            println!("Ошибка программы - {error}")
        }
    }

    let a: f64 = a_str.trim().parse().unwrap();
    let b: f64 = b_str.trim().parse().unwrap();
    let c: f64 = c_str.trim().parse().unwrap();

    let d: f64 = (b*b) - 4.0 * (a * c);

    if d > 0.0 {
        let x1 = ((-b) + d.sqrt()) / (2.0 * a);
        let x2 = ((-b) - d.sqrt()) / (2.0 * a);
        println!("Результат: \n Корень 1: {x1}, корень 2: {x2} от {d}")
    }

    if d == 0.0 {
        let x = (-b) / (2.0 * a);
        println!("Результат: \n Корень: {x} от {d}")
    }

    if d < 0.0 {
        println!("Результат: корней нет {d}")
    }

    println!("Результат: {d}")
}
