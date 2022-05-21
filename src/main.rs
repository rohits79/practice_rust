use rand::{thread_rng, Rng};
use std::io;

fn try_secret_gen() {
    let mut input_str: String = String::new();
    let mut input_num: i32;
    let mut rng = thread_rng();
    let secret_num: i32 = rng.gen_range(1..100);

    println!("secret number is {}", secret_num);
    loop {
        println!("Enter Secret Number");
        input_str.clear();
        match io::stdin().read_line(&mut input_str) {
            Ok(_) => (),
            Err(_) => continue,
        }

        println!("you entered: {}", input_str);
        match input_str.trim().parse() {
            Ok(num) => input_num = num,
            Err(_) => continue,
        }

        if input_num > secret_num {
            println!("input number is bigger")
        } else if input_num < secret_num {
            println!("input number is smaller")
        } else {
            println!("success");
            break;
        }
    }
}

fn try_loop_expression() {
    let mut count = 0;

    //loop returns value - use break to return the value
    let return_val = loop {
        if count < 10 {
            count = count + 1;
        } else {
            break count; //break XXX returns XXX to the outside let
        }
    }; // <---- semi colon is required as loop is an expression

    println!("count = {}", return_val)
}

fn try_range() {
    //type is Range<i32> - range is created simply by start..end no need for brackets
    let range = 1..10;

    for val in range {
        println!("{}", val);
    }
}

fn array_loop() {
    let arr = [3; 5];
    for item in arr.into_iter() {
        println!("{}", item);
    }
}

fn if_expression() {
    let count = 4;
    let return_val = if count > 5 { "it is 5" } else { "it is <= 5" }; // semi colon is necessary when using let-expressions

    println!("{}", return_val);
}

fn mut_reference() {
    let mut s = String::from("hello");

    let s1 = &mut s;
    let s2 = &mut s;

    //OK
    println!("{}", s2);

    // This will throw exception, as s was mut borrowed by s2
    //println!("{}", s1);

    // Likewise this will throw exception as well
    //println!("{} {}", s1, s2);
}

fn immut_iterate() {
    let names = vec!["Rohit", "Sharma"];

    //implicit call to into_iter() causes to "move"
    for name in &names {
        //should call names.iter or &names
        println!("{}", name);
    }

    for name in names {
        println!("{}", name);
    }
}

fn iter_update() {
    let mut names = vec![String::from("Test")];

    for name in &mut names {
        name.push('1');
    }
}

fn first_word(str: &str) -> &str {
    for (index, &item) in str.as_bytes().into_iter().enumerate() {
        if item == b' ' {
            return &str[..index];
        }
    }

    &str[..]
}

fn slice_string() {
    let name = String::from("I am a disco dancer");
    let word = first_word(&name);
    println!("first word '{}'", word);
}

fn main() {
    immut_iterate();
    //slice_string()
    //mut_reference();
    //if_expression();
    //array_loop();
    //try_range();
    //try_loop_expression();
}
