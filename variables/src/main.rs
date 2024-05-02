fn main() {
    /* shadowing */
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Hello, world! {spaces}");

    {
        let spaces = spaces * 2;
        println!("spaces*2 {spaces}");
    }
}
