use std::collections::HashMap;

enum Task {
    Fn0(Box<dyn Fn()>),
    Fn1(Box<dyn Fn(i32)>),
    Fn2(Box<dyn Fn(i32, i32)>)
}

fn main() {
    // Define the type of the HashMap with trait bounds for the closures
    let mut closures: HashMap<String, Task> = HashMap::new();

    // Insert closures into the HashMap
    closures.insert(
        "greet".to_string(),
        Task::Fn0(Box::new(|| println!("Hello, World!"))),
    );
    closures.insert(
        "count".to_string(),
        Task::Fn0(Box::new(|| {
            for i in 1..=3 {
                println!("{}", i);
            }
        })),
    );
    
    // Insert closure of type `Fn(T) -> Result<T, Error>` into the HashMap
    closures.insert(
        "double".to_string(),
        Task::Fn1(Box::new(|value: i32| {
	    println!("Double: {}", 2 * value)
        })),
    );

    // Invoke the closures by looking them up in the map
    if let Some(Task::Fn0(closure)) = closures.get("greet") {
        closure();
    }
    if let Some(Task::Fn0(closure)) = closures.get("count") {
        closure();
    }

    if let Some(Task::Fn1(closure)) = closures.get("double") {
        closure(10);
        closure(0);
    }
}
