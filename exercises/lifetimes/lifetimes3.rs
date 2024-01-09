// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

struct Book<'a> {
    author: &'a mut str,
    title: &'a mut str,
}

fn main() {
    let mut name = String::from("Jill Smith");
    let mut title = String::from("Fish Flying");
    let mut book = Book {
        author: &mut name,
        title: &mut title,
    };

    let mut binding = String::from("Sergio");
    book.author = &mut binding;

    println!("{} by {}", book.title, book.author);
}
