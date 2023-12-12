fn frobnicator() -> Option<i32> {
    Some(3)
}

fn main() {
    // Panic if the option is equal to None
    let a = frobnicator().unwrap();
    
    // Panic with a nice error message if the option is equal to None
    let a = frobnicator().expect("The frobnicator is broken!");

    // Just check to see if we got a value at all
    let a = frobnicator();
    if a.is_some() { 
        // Do something
    }
    if a.is_none() {
        // Do something
    }

    // Use "match" for pattern matching
    let a = frobnicator();
    match a {
        None => {
            // Do Something
        }
        Some(a) => {
            // a now refers to the contents of a
            // Do something
        }
    }

    // Use "if let" for single-arm pattern matching
    let a = frobnicator();
    if let Some(a) = a {
        // Do something
    }
}