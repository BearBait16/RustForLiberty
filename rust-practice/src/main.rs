fn main() {
    rockstar();
}

fn rockstar() {
    let number = 5;
    println!("The value of numbers is: {number}");

    // Making commments is pretty easy

    let mut x = 10;
    x += 1;
    println!("The value of x is {x}");

    if x > number {
        println!("X is greater then number");
    } else if number < x {
        println!("Number is greater then x");
    } else {
        println!("How the frik did you break this?");
    }

    let awesomeList = [1, 2, 8, 4, 5];
    let thirdItem = awesomeList[2];
    println!("awesomelist item 3 is {thirdItem}");
}

// fn super()
// {
// io::stdin()
//     .read_line(&mut input)
//     .expect("Failed to read line");
// if input.trim() == option1 {
//     display_news().await;
// } else if input.trim() == option2 {
//     display_active_planets().await;
// } else if input.trim() == option3 {
//     println!("AHHHHHHHHHH");
// } else if input.trim() == option4 {
//     quit = true;
// } else {
//     println!("You must be facsist! To prove you're not, try again.")
// }

// input = input.trim();
// match input
// {
//     "1" => {display_news().await;}
//     "2" => {display_active_planets().await;}
//     "3" => {println!("Ahhhhhhh");}
//     "4" => {quit = false;}
// }
// }
