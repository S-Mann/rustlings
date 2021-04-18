fn main() {
    /* Variables can be mutable(the variable can be reassigned with new value) or immutable */
    let x = 5;
    let mut mut_x = 10;
    /*
    You can also declare constants but they can only be immutable and can have global and local scope. const keyword can only be use on experssions that have evaluate to constant values. So no result from functions and other run time values. Constants also need to have explicit data typing.
    */
    const APP_NAME: &str = "my_awesome_app";
    /*
    We can also do something called as variable shadowing, which is like reinitializing a variable in the same scope. This is reminiscent of most interpreted languages where you do
    x = 10;
    x = "string";
    */
    let y = "text";
    let x = y;
    println!("The value of x is: {}", x);
}
