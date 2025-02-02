// Loops are used to iterate until a condition is met 
#[allow(dead_code)]
pub fn run(){

    let mut count = 0;

    // infinite loop: 
    loop {
        count += 1;
        println!("Number: {} ", count);

        if count == 20 {
            break;
        }
    }
    
    // While loop: example using the Fizzbuzz quiz: if a number if divisible by 3, print fizz, if divisible by 5 print buzz, if divisible by both print fizzbuzz
    count = 0;
    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        }else if count % 3 == 0 {
            println!("fizz");
        }else if count % 5 == 0 { 
            println!("buzz")
        }else{
            println!("{}", count);
        }
        count+=1;
    } 

    // For Range loop 
    for x in 0..100{
        if x % 15 == 0 {
            println!("fizzbuzz");
        }else if x % 3 == 0 {
            println!("fizz");
        }else if x % 5 == 0 { 
            println!("buzz")
        }else{
            println!("{}", x);
        }
    }

}