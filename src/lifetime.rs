
pub fn starting_point(){

  
    example();
  
 
 
 }
// fn longer_error(a: &str, b: &str) -> &str {
//     if a.len() > b.len() {
//         a
//     } else {
//         b
//     }
// }

fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
fn example() {
    let string1 = "This is a long string";
    let string2 = "This is short";

    let result = longer(string1, string2);
    println!("The longer string is: '{}'", result);
}
