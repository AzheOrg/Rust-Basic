// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut shopping_list: Vec<String> = Vec::new(); //shopping_list需要指定类型
    shopping_list.push("milk".to_string());
}
