fn main() {
    const _MY_VAR: u8 = 8;
    static MY_STATIC_VAR: u8 = 4;
    println!("{}", MY_STATIC_VAR);

    let my_string = String::from("Rohit Gupta");
    let my_str: str = &my_string[4..9];
    println!("{}", my_str[5]);

    let my_arr: [usize;5] = [1, 2, 3, 4, 5];
    let my_arra:[u8;5] = [1, 2];
}
