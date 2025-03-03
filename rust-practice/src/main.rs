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
