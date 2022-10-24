struct Number {
    odd: bool,
    value: i32
}


fn main() {
    let one = Number { odd: true, value: 1 };
    let two = Number { odd: false, value: 2 };

    print_number(one);
    print_number(two);
    println!("#####");

    for i in 0..10 {
        let num = Number{odd: i%2 == 0, value: i};
        print_number(num);
    }
}

fn print_number(n: Number) {
    match n {
        Number {odd: true, value} => println!("Odd number: {}", value),
        Number {odd: false, value} => println!("Even number: {}", value)
    }
}
