use std::io;
// Finds the Odd even number
/* 
In this function first we create on mutable variable named input
and then assign it to a value of type String. 
After that, we use match statement and check if there is any character in string or not.
if not then in number we set that value or else print the statement 
At the end we check the number is odd or even
*/
pub fn is_odd_even(){
    let mut input = String::new();
    println!("Enter a number");
    io::stdin()
        .read_line(&mut input)
        .expect("Faild to read Data");
    
    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a valid number!");
            return; // Exit the function if parsing fails
        }
    };
    if number%2 == 0 {
        println!("Number {} is even",number);
    }
    else {
        println!("Number {} is odd",number);
    }
}

pub fn infinite(){
    let mut counter = 0;

    loop {
        println!("Count is :{}",counter);

        if(counter>=5){
            break;
        }
        counter+=1;
    }
}