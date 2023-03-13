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

    let value = 23;
    match value {
        10 => println!("Num is 10"),
        23 => {
            println!("Num is 23");
        },
        10..=50 => {
            println!("Num is between"); //не выполнится т.к совпадение выше
        },
        _ => {
            println!("Error");
        }
    }
    let value2 = match value {
        2 => 1,
        3 => 10,
        _ => 0
    };
    println!("value2 {}", value2)
}
//integer, 1 5 7 9
//i8 i16 i32 i64 i128 isize