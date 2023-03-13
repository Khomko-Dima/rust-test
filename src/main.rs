fn main() {
    let mut age: i8 = 28;
    let name = String::from("Dima");
    let symbol: char = 'F';
    let boolean: bool = true;

    println!("My name {name}, my age {age}");
    println!("{}, {}", symbol, boolean);

    if age >= 18 {
        println!("Have 18")
    } else {
        println!("Don't have 18")
    }

    let num = if boolean { 1 } else { 0 };
    println!("num {}", num);

    let mut counter = 0;
    loop {
        if counter == 3 {
            break;
        }

        println!("loop: {}", counter);
        counter += 1;
    }
    let mut counter2 = 0;
    while counter2 <= 3 {
        println!("while: {}", counter2);
        counter2 += 1;
    }

    for i in 0..5 {
        println!("for: {}", i);
    }
}
//integer, 1 5 7 9
//i8 i16 i32 i64 i128 isize