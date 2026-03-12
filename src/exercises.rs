#[derive(Clone, Debug)]
pub struct ExerciseStep {
    pub label: &'static str,
    pub code: &'static str,
    /// Pretty-printed version for display. Falls back to `code` if None.
    pub display: Option<&'static str>,
    /// Optional hint shown when user clicks "Hint".
    pub hint: Option<&'static str>,
}

#[derive(Clone, Debug)]
pub struct Exercise {
    pub title: &'static str,
    pub description: &'static str,
    pub theory: &'static str,
    pub category: &'static str,
    pub steps: &'static [ExerciseStep],
}

pub static EXERCISES: &[Exercise] = &[
    // 0: Hello World
    Exercise {
        title: "Hello, World!",
        description: "Every journey starts with Hello World. Rust uses macros (ending with !) for formatted output. println! writes to stdout with a newline.",
        theory: "In Rust, println! is a macro, not a function — notice the !. Macros can take a variable number of arguments and are expanded at compile time. The string inside is a format string, similar to printf in C.",
        category: "Basics",
        steps: &[ExerciseStep {
            label: "Print \"Hello, world!\" to the console",
            code: r#"println!("Hello, world!");"#,
            display: None,
            hint: None,
        }],
    },
    // 1: Variables & Types
    Exercise {
        title: "Variables & Types",
        description: "Rust is statically typed. Variables are immutable by default. Use `let` to declare, and add type annotations when needed. Rust has integers (i32, u64, usize...), floats (f32, f64), booleans, and chars.",
        theory: "Variables are declared with let and are immutable by default. Rust is statically typed — the compiler must know every variable's type at compile time. Type annotations use : Type syntax. Common types: i32, f64, bool, char (Unicode scalar).",
        category: "Basics",
        steps: &[
            ExerciseStep {
                label: "Declare an integer",
                code: "let age: u32 = 25;",
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Declare a float",
                code: "let pi: f64 = 3.14159;",
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Declare a boolean",
                code: "let is_rustacean: bool = true;",
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Declare a character",
                code: "let ferris: char = '🦀';",
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Print all values",
                code: r#"println!("age={}, pi={:.2}, rustacean={}, mascot={}", age, pi, is_rustacean, ferris);"#,
                display: Some("println!(\n  \"age={}, pi={:.2}, rustacean={}, mascot={}\",\n  age, pi, is_rustacean, ferris\n);"),
                hint: None,
            },
        ],
    },
    // 2: Mutability
    Exercise {
        title: "Mutability",
        description: "Variables are immutable by default in Rust. Use `mut` to allow changes. This design choice helps prevent bugs — you only opt into mutability when you actually need it.",
        theory: "By default, variables in Rust are immutable — you cannot change their value after binding. This prevents bugs and makes code easier to reason about. Add mut after let to make a variable mutable. The compiler warns if mut is unused.",
        category: "Basics",
        steps: &[
            ExerciseStep {
                label: "Declare a mutable variable",
                code: "let mut score = 0;",
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Increase the score",
                code: "score += 10;",
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Increase again",
                code: "score += 25;",
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Print the final score",
                code: r#"println!("Final score: {}", score);"#,
                display: None,
                hint: None,
            },
        ],
    },
    // 3: Strings
    Exercise {
        title: "Strings",
        description: "Rust has two main string types: `String` (owned, heap-allocated, growable) and `&str` (borrowed string slice). String literals are `&str`. Use `.to_string()` or `String::from()` to create owned strings.",
        theory: "Rust has two string types: String (owned, growable, heap) and &str (borrowed slice). Both are UTF-8. Use .to_string() or String::from() to create a String. Use format!() for concatenation. Direct byte indexing is not allowed.",
        category: "Basics",
        steps: &[
            ExerciseStep {
                label: "Create an owned String",
                code: r#"let mut greeting = String::from("Hello");"#,
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Append to the string",
                code: r#"greeting.push_str(", Rust!");"#,
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Get the length",
                code: "let len = greeting.len();",
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Check if it contains a word",
                code: r#"let has_rust = greeting.contains("Rust");"#,
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Print the results",
                code: r#"println!("'{}' (len={}, has_rust={})", greeting, len, has_rust);"#,
                display: Some("println!(\n  \"'{}' (len={}, has_rust={})\",\n  greeting, len, has_rust\n);"),
                hint: None,
            },
        ],
    },
    // 4: Tuples & Destructuring
    Exercise {
        title: "Tuples & Destructuring",
        description: "Tuples group values of different types into one compound type. Access elements by index (tuple.0) or destructure them with pattern matching. Tuples are useful for returning multiple values from a function.",
        theory: "Tuples group values of different types: (i32, f64, bool). Access with .0, .1. Arrays hold same-type values with fixed length: [i32; 5]. Indexing uses []. Rust panics on out-of-bounds access at runtime.",
        category: "Basics",
        steps: &[
            ExerciseStep {
                label: "Create a tuple",
                code: r#"let person = ("Alice", 30, true);"#,
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Access by index",
                code: r#"println!("Name: {}", person.0);"#,
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Destructure the tuple",
                code: "let (name, age, active) = person;",
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Print destructured values",
                code: r#"println!("{} is {} years old, active: {}", name, age, active);"#,
                display: Some("println!(\n  \"{} is {} years old, active: {}\",\n  name, age, active\n);"),
                hint: None,
            },
        ],
    },
    // 5: Arrays & Iteration
    Exercise {
        title: "Arrays & Iteration",
        description: "Arrays have a fixed size and contain elements of the same type. Use `[T; N]` syntax. Iterate with `for..in` or use iterator methods like `.iter()`, `.map()`, `.filter()`.",
        theory: "Arrays have a fixed size known at compile time: [T; N]. Access elements with [] indexing. Use .iter() to iterate over elements. Rust checks array bounds at runtime and panics on out-of-bounds access. For growable collections, use Vec instead.",
        category: "Basics",
        steps: &[
            ExerciseStep {
                label: "Create an array",
                code: "let numbers = [1, 2, 3, 4, 5];",
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Access by index",
                code: r#"println!("First: {}, Last: {}", numbers[0], numbers[4]);"#,
                display: Some("println!(\n  \"First: {}, Last: {}\",\n  numbers[0], numbers[4]\n);"),
                hint: None,
            },
            ExerciseStep {
                label: "Calculate the sum",
                code: "let sum: i32 = numbers.iter().sum();",
                display: None,
                hint: Some("Use .iter() to get an iterator, then chain .sum() to add up all elements."),
            },
            ExerciseStep {
                label: "Find the max",
                code: "let max = numbers.iter().max().unwrap();",
                display: None,
                hint: Some("Chain .iter().max() to find the largest element. It returns an Option, so use .unwrap()."),
            },
            ExerciseStep {
                label: "Print results",
                code: r#"println!("Sum: {}, Max: {}, Len: {}", sum, max, numbers.len());"#,
                display: Some("println!(\n  \"Sum: {}, Max: {}, Len: {}\",\n  sum, max, numbers.len()\n);"),
                hint: None,
            },
        ],
    },
    // 6: Vectors
    Exercise {
        title: "Vectors",
        description: "Vec<T> is Rust's growable array type. Unlike arrays, vectors can change size at runtime. Use `vec![]` macro or `Vec::new()` to create them. Vectors are one of the most used collections in Rust.",
        theory: "Vec<T> is a growable array on the heap. Create with Vec::new() or vec![]. push() adds elements, [] or get() accesses them. get() returns Option<&T> (safe), while [] panics on out-of-bounds.",
        category: "Basics",
        steps: &[
            ExerciseStep {
                label: "Create a vector with the macro",
                code: "let mut fruits = vec![\"apple\", \"banana\"];",
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Add elements",
                code: r#"fruits.push("cherry"); fruits.push("date");"#,
                display: Some("fruits.push(\"cherry\");\nfruits.push(\"date\");"),
                hint: None,
            },
            ExerciseStep {
                label: "Remove the last element",
                code: "let last = fruits.pop().unwrap();",
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Print the vector and removed item",
                code: r#"println!("Fruits: {:?}, removed: {}", fruits, last);"#,
                display: Some("println!(\n  \"Fruits: {:?}, removed: {}\",\n  fruits, last\n);"),
                hint: None,
            },
        ],
    },
    // 7: Control Flow
    Exercise {
        title: "Control Flow",
        description: "Rust has `if/else`, `loop`, `while`, and `for`. Notably, `if` is an expression — it returns a value. There's no ternary operator because `if/else` already fills that role.",
        theory: "Rust has if/else (which is an expression — it returns a value), loop (infinite loop), while (conditional loop), and for..in (iterator loop). There is no ternary operator; use if/else instead. break exits loops and can return a value from loop.",
        category: "Basics",
        steps: &[
            ExerciseStep {
                label: "Use if as an expression",
                code: r#"let temp = 22; let weather = if temp > 30 { "hot" } else if temp > 15 { "nice" } else { "cold" };"#,
                display: Some("let temp = 22;\nlet weather = if temp > 30 {\n  \"hot\"\n} else if temp > 15 {\n  \"nice\"\n} else {\n  \"cold\"\n};"),
                hint: None,
            },
            ExerciseStep {
                label: "Print the weather",
                code: r#"println!("{}°C is {}", temp, weather);"#,
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Use a for loop with range",
                code: r#"for i in 1..=5 { print!("{} ", i * i); }"#,
                display: Some("for i in 1..=5 {\n  print!(\"{} \", i * i);\n}"),
                hint: None,
            },
            ExerciseStep {
                label: "Print newline after squares",
                code: r#"println!();"#,
                display: None,
                hint: None,
            },
        ],
    },
    // 8: Functions
    Exercise {
        title: "Functions",
        description: "Functions are declared with `fn`. Parameters need type annotations. The return type follows `->`. Rust functions return the last expression implicitly (no semicolon = return value).",
        theory: "Functions are declared with fn. Parameters require type annotations. The return type follows ->. Rust is expression-based: the last expression without ; becomes the return value. Adding ; turns it into a statement returning ().",
        category: "Intermediate",
        steps: &[
            ExerciseStep {
                label: "Define a function",
                code: "fn square(n: i32) -> i32 { n * n };",
                display: Some("fn square(n: i32) -> i32 {\n  n * n\n};"),
                hint: Some("Use fn name(param: Type) -> ReturnType { body }. The last expression without a semicolon is the return value."),
            },
            ExerciseStep {
                label: "Define a function with multiple params",
                code: r#"fn greet(name: &str, times: u32) { for _ in 0..times { println!("Hello, {}!", name); } };"#,
                display: Some("fn greet(name: &str, times: u32) {\n  for _ in 0..times {\n    println!(\"Hello, {}!\", name);\n  }\n};"),
                hint: Some("Functions with no return value omit the -> ReturnType. Use &str for borrowed string parameters."),
            },
            ExerciseStep {
                label: "Call square",
                code: r#"println!("7² = {}", square(7));"#,
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Call greet",
                code: r#"greet("Ferris", 2);"#,
                display: None,
                hint: None,
            },
        ],
    },
    // 9: Closures
    Exercise {
        title: "Closures",
        description: "Closures are anonymous functions that can capture variables from their environment. They use `|params|` syntax. Closures are widely used with iterator methods like `.map()`, `.filter()`, and `.for_each()`.",
        theory: "Closures are anonymous functions that capture their environment. Syntax: |x, y| x + y. They implement Fn (borrow), FnMut (borrow mutably), or FnOnce (take ownership). The compiler infers which trait based on how captured values are used.",
        category: "Intermediate",
        steps: &[
            ExerciseStep {
                label: "Create a simple closure",
                code: "let double = |x: i32| x * 2;",
                display: None,
                hint: Some("Closures use |params| body syntax. Type annotations are optional: |x: i32| x * 2."),
            },
            ExerciseStep {
                label: "Use it",
                code: r#"println!("double(5) = {}", double(5));"#,
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Use closures with iterators",
                code: "let nums = vec![1, 2, 3, 4, 5]; let evens: Vec<i32> = nums.iter().filter(|&&x| x % 2 == 0).cloned().collect();",
                display: Some("let nums = vec![1, 2, 3, 4, 5];\nlet evens: Vec<i32> = nums\n  .iter()\n  .filter(|&&x| x % 2 == 0)\n  .cloned()\n  .collect();"),
                hint: Some("Chain .iter().filter(|&&x| condition).cloned().collect() to filter elements. The &&x pattern dereferences the double reference from iter + filter."),
            },
            ExerciseStep {
                label: "Print filtered results",
                code: r#"println!("Evens: {:?}", evens);"#,
                display: None,
                hint: None,
            },
        ],
    },
    // 10: Ownership
    Exercise {
        title: "Ownership",
        description: "Ownership is Rust's most unique feature. Each value has exactly one owner. When the owner goes out of scope, the value is dropped. Assigning or passing a heap value moves it — the original variable becomes invalid.",
        theory: "Ownership is Rust's key feature. Rules: each value has one owner, when the owner goes out of scope the value is dropped, assigning moves the value (for heap data). This eliminates garbage collection while preventing memory leaks.",
        category: "Intermediate",
        steps: &[
            ExerciseStep {
                label: "Create an owned string",
                code: r#"let s1 = String::from("hello");"#,
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Clone instead of move",
                code: "let s2 = s1.clone();",
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Both are still valid after clone",
                code: r#"println!("s1={}, s2={}", s1, s2);"#,
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Integers implement Copy (no move)",
                code: "let a = 42; let b = a;",
                display: Some("let a = 42;\nlet b = a;"),
                hint: None,
            },
            ExerciseStep {
                label: "Both ints are still valid",
                code: r#"println!("a={}, b={}", a, b);"#,
                display: None,
                hint: None,
            },
        ],
    },
    // 11: References & Borrowing
    Exercise {
        title: "References & Borrowing",
        description: "References let you use a value without taking ownership. `&T` is an immutable reference, `&mut T` is a mutable one. Rule: you can have many &T OR one &mut T, never both at the same time.",
        theory: "References let you use a value without taking ownership. &x creates an immutable reference, &mut x a mutable one. Rule: you can have one mutable reference OR any number of immutable references. This prevents data races at compile time.",
        category: "Intermediate",
        steps: &[
            ExerciseStep {
                label: "Create a string",
                code: r#"let mut text = String::from("hello");"#,
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Define a function that borrows",
                code: "fn count_chars(s: &str) -> usize { s.len() };",
                display: Some("fn count_chars(s: &str) -> usize {\n  s.len()\n};"),
                hint: Some("Use &str as the parameter type to borrow a string slice. The function signature is fn name(s: &str) -> ReturnType."),
            },
            ExerciseStep {
                label: "Borrow immutably",
                code: r#"let len = count_chars(&text); println!("Length: {}", len);"#,
                display: Some("let len = count_chars(&text);\nprintln!(\"Length: {}\", len);"),
                hint: None,
            },
            ExerciseStep {
                label: "Borrow mutably and modify",
                code: r#"text.push_str(" world"); println!("Modified: {}", text);"#,
                display: Some("text.push_str(\" world\");\nprintln!(\"Modified: {}\", text);"),
                hint: None,
            },
        ],
    },
    // 12: Structs
    Exercise {
        title: "Structs",
        description: "Structs let you create custom data types by grouping related values. Define methods and associated functions with `impl` blocks. Derive macros like `#[derive(Debug)]` add common trait implementations automatically.",
        theory: "Structs create custom types with named fields. Define with struct Name { field: Type }. Add methods with impl Name { fn method(&self) {} }. The &self parameter borrows the struct. Use &mut self for methods that modify fields.",
        category: "Intermediate",
        steps: &[
            ExerciseStep {
                label: "Define a struct",
                code: "#[derive(Debug)] struct Rectangle { width: f64, height: f64 };",
                display: Some("#[derive(Debug)]\nstruct Rectangle {\n  width: f64,\n  height: f64,\n};"),
                hint: Some("Use struct Name { field: Type, ... } to define a struct. Add #[derive(Debug)] to enable {:?} printing."),
            },
            ExerciseStep {
                label: "Add methods with impl",
                code: "impl Rectangle { fn area(&self) -> f64 { self.width * self.height } fn is_square(&self) -> bool { (self.width - self.height).abs() < f64::EPSILON } };",
                display: Some("impl Rectangle {\n  fn area(&self) -> f64 {\n    self.width * self.height\n  }\n  fn is_square(&self) -> bool {\n    (self.width - self.height).abs()\n      < f64::EPSILON\n  }\n};"),
                hint: Some("Use impl StructName { fn method(&self) -> Type { ... } } to add methods. &self borrows the struct immutably."),
            },
            ExerciseStep {
                label: "Create an instance",
                code: "let rect = Rectangle { width: 10.0, height: 5.0 };",
                display: Some("let rect = Rectangle {\n  width: 10.0,\n  height: 5.0,\n};"),
                hint: None,
            },
            ExerciseStep {
                label: "Call methods and print",
                code: r#"println!("{:?} area={}, square={}", rect, rect.area(), rect.is_square());"#,
                display: Some("println!(\n  \"{:?} area={}, square={}\",\n  rect, rect.area(), rect.is_square()\n);"),
                hint: None,
            },
        ],
    },
    // 13: Enums & Pattern Matching
    Exercise {
        title: "Enums & Pattern Matching",
        description: "Enums define a type with multiple variants. Combined with `match`, they're incredibly powerful. Rust's `match` must be exhaustive — you must handle every possible variant, making your code safer.",
        theory: "Enums define a type with several variants. Unlike C, Rust enum variants can hold data: enum Shape { Circle(f64), Rect(f64, f64) }. match is exhaustive — you must handle every variant. The compiler catches missing cases.",
        category: "Intermediate",
        steps: &[
            ExerciseStep {
                label: "Define an enum with data",
                code: "#[derive(Debug)] enum Shape { Circle(f64), Square(f64), Rect(f64, f64) };",
                display: Some("#[derive(Debug)]\nenum Shape {\n  Circle(f64),\n  Square(f64),\n  Rect(f64, f64),\n};"),
                hint: Some("Use enum Name { Variant(Type), ... } to define variants that hold data."),
            },
            ExerciseStep {
                label: "Define an area function using match",
                code: "fn area(s: &Shape) -> f64 { match s { Shape::Circle(r) => std::f64::consts::PI * r * r, Shape::Square(s) => s * s, Shape::Rect(w, h) => w * h } };",
                display: Some("fn area(s: &Shape) -> f64 {\n  match s {\n    Shape::Circle(r) =>\n      std::f64::consts::PI * r * r,\n    Shape::Square(s) => s * s,\n    Shape::Rect(w, h) => w * h,\n  }\n};"),
                hint: Some("match must cover every variant. Use Pattern => expression for each arm. Destructure enum data with Variant(binding)."),
            },
            ExerciseStep {
                label: "Create shapes and calculate areas",
                code: r#"let shapes = vec![Shape::Circle(5.0), Shape::Square(4.0), Shape::Rect(3.0, 7.0)];"#,
                display: Some("let shapes = vec![\n  Shape::Circle(5.0),\n  Shape::Square(4.0),\n  Shape::Rect(3.0, 7.0),\n];"),
                hint: None,
            },
            ExerciseStep {
                label: "Print all areas",
                code: r#"for s in &shapes { println!("{:?} -> area = {:.2}", s, area(s)); }"#,
                display: Some("for s in &shapes {\n  println!(\n    \"{:?} -> area = {:.2}\",\n    s, area(s)\n  );\n}"),
                hint: None,
            },
        ],
    },
    // 14: Option & Result
    Exercise {
        title: "Option & Result",
        description: "Rust has no null. Instead, `Option<T>` represents an optional value (Some or None) and `Result<T, E>` represents success or failure. Use pattern matching or methods like `.unwrap_or()` to handle them.",
        theory: "Rust has no null. Option<T> is either Some(value) or None — used when a value might be absent. Result<T, E> is either Ok(value) or Err(error) — used for operations that can fail. Both force you to handle all cases, preventing null pointer bugs.",
        category: "Intermediate",
        steps: &[
            ExerciseStep {
                label: "Define a safe division function",
                code: r#"fn safe_div(a: f64, b: f64) -> Option<f64> { if b == 0.0 { None } else { Some(a / b) } };"#,
                display: Some("fn safe_div(a: f64, b: f64) -> Option<f64> {\n  if b == 0.0 {\n    None\n  } else {\n    Some(a / b)\n  }\n};"),
                hint: Some("Return Option<T> from a function: use Some(value) for success and None for absence. The return type is Option<f64>."),
            },
            ExerciseStep {
                label: "Handle the Option",
                code: r#"match safe_div(10.0, 3.0) { Some(v) => println!("10/3 = {:.2}", v), None => println!("Cannot divide by zero!") }"#,
                display: Some("match safe_div(10.0, 3.0) {\n  Some(v) => println!(\"10/3 = {:.2}\", v),\n  None => println!(\"Cannot divide by zero!\"),\n}"),
                hint: Some("Match on Option with: Some(val) => ... and None => ... to handle both cases exhaustively."),
            },
            ExerciseStep {
                label: "Handle the None case",
                code: r#"match safe_div(10.0, 0.0) { Some(v) => println!("10/0 = {:.2}", v), None => println!("Cannot divide by zero!") }"#,
                display: Some("match safe_div(10.0, 0.0) {\n  Some(v) => println!(\"10/0 = {:.2}\", v),\n  None => println!(\"Cannot divide by zero!\"),\n}"),
                hint: Some("Same pattern as before: match on Some(v) and None. This time the None arm will execute."),
            },
            ExerciseStep {
                label: "Use unwrap_or with a default",
                code: r#"let result = safe_div(10.0, 0.0).unwrap_or(0.0); println!("With default: {}", result);"#,
                display: Some("let result = safe_div(10.0, 0.0)\n  .unwrap_or(0.0);\nprintln!(\"With default: {}\", result);"),
                hint: None,
            },
        ],
    },
    // 15: Error Handling
    Exercise {
        title: "Error Handling",
        description: "Rust encourages explicit error handling with Result<T, E>. The `?` operator propagates errors automatically. Custom error types help create clear, maintainable error handling throughout your application.",
        theory: "Rust has no exceptions. Use Result<T, E> for recoverable errors and panic! for unrecoverable ones. Result has Ok(value) and Err(error). The ? operator propagates errors. unwrap() gets the value but panics on Err.",
        category: "Intermediate",
        steps: &[
            ExerciseStep {
                label: "Parse a string to a number",
                code: r#"let good: Result<i32, _> = "42".parse(); let bad: Result<i32, _> = "abc".parse();"#,
                display: Some("let good: Result<i32, _> = \"42\".parse();\nlet bad: Result<i32, _> = \"abc\".parse();"),
                hint: None,
            },
            ExerciseStep {
                label: "Handle with match",
                code: r#"match good { Ok(n) => println!("Parsed: {}", n), Err(e) => println!("Error: {}", e) }"#,
                display: Some("match good {\n  Ok(n) => println!(\"Parsed: {}\", n),\n  Err(e) => println!(\"Error: {}\", e),\n}"),
                hint: Some("Match on Result with Ok(value) => ... and Err(e) => ... to handle success and failure."),
            },
            ExerciseStep {
                label: "Handle the error case",
                code: r#"match bad { Ok(n) => println!("Parsed: {}", n), Err(e) => println!("Error: {}", e) }"#,
                display: Some("match bad {\n  Ok(n) => println!(\"Parsed: {}\", n),\n  Err(e) => println!(\"Error: {}\", e),\n}"),
                hint: Some("Same match pattern: Ok(n) and Err(e). This time the Err arm will execute since \"abc\" is not a number."),
            },
            ExerciseStep {
                label: "Use map and unwrap_or_else",
                code: r#"let doubled = "21".parse::<i32>().map(|n| n * 2).unwrap_or_else(|e| { println!("Error: {}", e); 0 }); println!("Doubled: {}", doubled);"#,
                display: Some("let doubled = \"21\".parse::<i32>()\n  .map(|n| n * 2)\n  .unwrap_or_else(|e| {\n    println!(\"Error: {}\", e);\n    0\n  });\nprintln!(\"Doubled: {}\", doubled);"),
                hint: Some("Chain .map(|n| transform) to transform the Ok value, and .unwrap_or_else(|e| fallback) to handle errors with a closure."),
            },
        ],
    },
    // 16: Traits
    Exercise {
        title: "Traits",
        description: "Traits define shared behavior — similar to interfaces in other languages. Types implement traits to provide specific behavior. Trait bounds let you write generic code that works with any type implementing required traits.",
        theory: "Traits define shared behavior — similar to interfaces. Define with trait Name { fn method(&self); }. Implement with impl Trait for Type. Traits can have default implementations. Use trait bounds to constrain generic types.",
        category: "Advanced",
        steps: &[
            ExerciseStep {
                label: "Define a trait",
                code: "trait Greetable { fn greet(&self) -> String; };",
                display: Some("trait Greetable {\n  fn greet(&self) -> String;\n};"),
                hint: Some("Use trait Name { fn method(&self) -> Type; } to declare a trait with required methods."),
            },
            ExerciseStep {
                label: "Define structs",
                code: "struct Dog { name: String }; struct Cat { name: String };",
                display: Some("struct Dog { name: String };\nstruct Cat { name: String };"),
                hint: None,
            },
            ExerciseStep {
                label: "Implement trait for Dog",
                code: r#"impl Greetable for Dog { fn greet(&self) -> String { format!("Woof! I'm {}", self.name) } };"#,
                display: Some("impl Greetable for Dog {\n  fn greet(&self) -> String {\n    format!(\"Woof! I'm {}\", self.name)\n  }\n};"),
                hint: Some("Use impl TraitName for TypeName { fn method(&self) -> Type { ... } } to implement a trait for a specific type."),
            },
            ExerciseStep {
                label: "Implement trait for Cat",
                code: r#"impl Greetable for Cat { fn greet(&self) -> String { format!("Meow! I'm {}", self.name) } };"#,
                display: Some("impl Greetable for Cat {\n  fn greet(&self) -> String {\n    format!(\"Meow! I'm {}\", self.name)\n  }\n};"),
                hint: Some("Same pattern: impl TraitName for TypeName. Each type provides its own implementation of the trait methods."),
            },
            ExerciseStep {
                label: "Use the trait",
                code: r#"let dog = Dog { name: "Rex".into() }; let cat = Cat { name: "Whiskers".into() }; println!("{}", dog.greet()); println!("{}", cat.greet());"#,
                display: Some("let dog = Dog { name: \"Rex\".into() };\nlet cat = Cat { name: \"Whiskers\".into() };\nprintln!(\"{}\", dog.greet());\nprintln!(\"{}\", cat.greet());"),
                hint: None,
            },
        ],
    },
    // 17: Generics
    Exercise {
        title: "Generics",
        description: "Generics let you write code that works with multiple types. Combine with trait bounds to constrain what types are accepted. Rust monomorphizes generics at compile time — zero runtime cost!",
        theory: "Generics let you write code for multiple types. Use <T> as type parameter: fn largest<T>(list: &[T]) -> &T. Add bounds to constrain types: <T: PartialOrd>. Rust monomorphizes generics at compile time — zero runtime cost.",
        category: "Advanced",
        steps: &[
            ExerciseStep {
                label: "Define a generic function",
                code: "fn largest<T: PartialOrd>(list: &[T]) -> &T { let mut biggest = &list[0]; for item in &list[1..] { if item > biggest { biggest = item; } } biggest };",
                display: Some("fn largest<T: PartialOrd>(list: &[T]) -> &T {\n  let mut biggest = &list[0];\n  for item in &list[1..] {\n    if item > biggest {\n      biggest = item;\n    }\n  }\n  biggest\n};"),
                hint: Some("Use fn name<T: TraitBound>(param: &[T]) -> &T to define a generic function. The trait bound constrains which types are accepted."),
            },
            ExerciseStep {
                label: "Use with integers",
                code: r#"let numbers = vec![34, 50, 25, 100, 65]; println!("Largest number: {}", largest(&numbers));"#,
                display: Some("let numbers = vec![34, 50, 25, 100, 65];\nprintln!(\n  \"Largest number: {}\",\n  largest(&numbers)\n);"),
                hint: None,
            },
            ExerciseStep {
                label: "Use with strings",
                code: r#"let words = vec!["rust", "is", "awesome"]; println!("Largest word: {}", largest(&words));"#,
                display: Some("let words = vec![\"rust\", \"is\", \"awesome\"];\nprintln!(\n  \"Largest word: {}\",\n  largest(&words)\n);"),
                hint: None,
            },
        ],
    },
    // 18: Iterators
    Exercise {
        title: "Iterator Magic",
        description: "Rust's iterators are lazy and composable. Chain methods like .map(), .filter(), .fold() to build powerful data pipelines. The compiler optimizes them into efficient loops — zero-cost abstractions!",
        theory: "Iterators produce values lazily. Call .iter() on collections. Chain adapters: .map(), .filter(), .enumerate(). Consume with .collect(), .sum(), .for_each(). Iterators are zero-cost abstractions — same performance as hand-written loops.",
        category: "Advanced",
        steps: &[
            ExerciseStep {
                label: "Create a range and transform it",
                code: "let squares: Vec<i32> = (1..=10).map(|x| x * x).collect();",
                display: Some("let squares: Vec<i32> = (1..=10)\n  .map(|x| x * x)\n  .collect();"),
                hint: Some("Chain .map(|x| transform) to transform each element, then .collect() to gather results into a Vec."),
            },
            ExerciseStep {
                label: "Print squares",
                code: r#"println!("Squares: {:?}", squares);"#,
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Chain filter and sum",
                code: "let even_sum: i32 = (1..=20).filter(|x| x % 2 == 0).sum();",
                display: Some("let even_sum: i32 = (1..=20)\n  .filter(|x| x % 2 == 0)\n  .sum();"),
                hint: Some("Chain .filter(|x| condition) to keep matching elements, then .sum() to add them up."),
            },
            ExerciseStep {
                label: "Use fold to build a string",
                code: r#"let csv = vec!["a", "b", "c"].iter().enumerate().fold(String::new(), |acc, (i, s)| { if i > 0 { format!("{},{}", acc, s) } else { s.to_string() } }); println!("Even sum: {}, CSV: {}", even_sum, csv);"#,
                display: Some("let csv = vec![\"a\", \"b\", \"c\"]\n  .iter()\n  .enumerate()\n  .fold(String::new(), |acc, (i, s)| {\n    if i > 0 {\n      format!(\"{},{}\", acc, s)\n    } else {\n      s.to_string()\n    }\n  });\nprintln!(\n  \"Even sum: {}, CSV: {}\",\n  even_sum, csv\n);"),
                hint: Some("Use .enumerate() to get (index, value) pairs, then .fold(initial, |acc, item| ...) to accumulate a result across all elements."),
            },
        ],
    },
    // 19: HashMap
    Exercise {
        title: "HashMaps",
        description: "HashMap<K, V> stores key-value pairs with O(1) average lookup. Import from std::collections. Use .entry() API for elegant insert-or-update patterns.",
        theory: "HashMap<K, V> stores key-value pairs with O(1) lookup. Import from std::collections. insert() adds entries, get() returns Option<&V>. The entry() API handles insert-or-update patterns elegantly.",
        category: "Advanced",
        steps: &[
            ExerciseStep {
                label: "Import and create a HashMap",
                code: r#"use std::collections::HashMap; let mut scores: HashMap<&str, i32> = HashMap::new();"#,
                display: Some("use std::collections::HashMap;\nlet mut scores: HashMap<&str, i32> =\n  HashMap::new();"),
                hint: None,
            },
            ExerciseStep {
                label: "Insert entries",
                code: r#"scores.insert("Alice", 95); scores.insert("Bob", 87); scores.insert("Carol", 92);"#,
                display: Some("scores.insert(\"Alice\", 95);\nscores.insert(\"Bob\", 87);\nscores.insert(\"Carol\", 92);"),
                hint: None,
            },
            ExerciseStep {
                label: "Use entry API",
                code: r#"scores.entry("Dave").or_insert(78); scores.entry("Alice").and_modify(|s| *s += 5);"#,
                display: Some("scores.entry(\"Dave\").or_insert(78);\nscores.entry(\"Alice\")\n  .and_modify(|s| *s += 5);"),
                hint: Some("Use .entry(key).or_insert(default) to insert if absent. Use .and_modify(|v| *v += n) to update an existing value with a closure."),
            },
            ExerciseStep {
                label: "Query and iterate",
                code: r#"println!("Alice: {}", scores["Alice"]); for (name, score) in &scores { print!("{}: {} ", name, score); } println!();"#,
                display: Some("println!(\"Alice: {}\", scores[\"Alice\"]);\nfor (name, score) in &scores {\n  print!(\"{}: {} \", name, score);\n}\nprintln!();"),
                hint: None,
            },
        ],
    },
    // Operator Overloading
    Exercise {
        title: "Operator Overloading",
        description: "Rust lets you overload operators by implementing traits from std::ops. Add custom + - * behavior to your types. The Debug trait with #[derive] gives you printable output.",
        theory: "Overload operators by implementing std::ops traits: Add for +, Mul for *, Index for []. The Display trait (std::fmt::Display) defines how a type prints with {} in format strings. Derive Debug for developer-facing output with {:?}.",
        category: "Advanced",
        steps: &[
            ExerciseStep {
                label: "Define a Point struct",
                code: "#[derive(Debug, Clone, Copy)] struct Point { x: f64, y: f64 };",
                display: Some("#[derive(Debug, Clone, Copy)]\nstruct Point {\n  x: f64,\n  y: f64,\n};"),
                hint: Some("Derive Clone and Copy so the struct can be used after being passed by value (e.g., after using +)."),
            },
            ExerciseStep {
                label: "Implement Add",
                code: "impl std::ops::Add for Point { type Output = Self; fn add(self, other: Self) -> Self { Point { x: self.x + other.x, y: self.y + other.y } } };",
                display: Some("impl std::ops::Add for Point {\n  type Output = Self;\n  fn add(self, other: Self) -> Self {\n    Point {\n      x: self.x + other.x,\n      y: self.y + other.y,\n    }\n  }\n};"),
                hint: Some("Use impl std::ops::Add for Type { type Output = Self; fn add(self, other: Self) -> Self { ... } } to overload the + operator."),
            },
            ExerciseStep {
                label: "Add a distance method",
                code: "impl Point { fn distance(&self, other: &Point) -> f64 { ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt() } };",
                display: Some("impl Point {\n  fn distance(&self, other: &Point) -> f64 {\n    ((self.x - other.x).powi(2)\n      + (self.y - other.y).powi(2))\n    .sqrt()\n  }\n};"),
                hint: Some("Add methods with impl Type { fn method(&self, ...) -> ReturnType { ... } }. Use .powi(2) for squaring and .sqrt() for square root."),
            },
            ExerciseStep {
                label: "Create points and use them",
                code: r#"let p1 = Point { x: 1.0, y: 2.0 }; let p2 = Point { x: 4.0, y: 6.0 }; let p3 = p1 + p2; println!("{:?} + {:?} = {:?}, distance = {:.2}", p1, p2, p3, p1.distance(&p2));"#,
                display: Some("let p1 = Point { x: 1.0, y: 2.0 };\nlet p2 = Point { x: 4.0, y: 6.0 };\nlet p3 = p1 + p2;\nprintln!(\n  \"{:?} + {:?} = {:?}, distance = {:.2}\",\n  p1, p2, p3, p1.distance(&p2)\n);"),
                hint: None,
            },
        ],
    },
    // Mini Project - Temperature Converter
    Exercise {
        title: "Mini Project: Temp Converter",
        description: "Build a temperature converter! This mini project combines functions, formatting, and f64 arithmetic. You'll create functions to convert between Celsius and Fahrenheit.",
        theory: "Convert between Celsius and Fahrenheit. Practice f64 arithmetic and formatted output with precision control ({:.1}). Formula: °F = °C × 9/5 + 32.",
        category: "Projects",
        steps: &[
            ExerciseStep {
                label: "Define Celsius to Fahrenheit",
                code: "fn c_to_f(c: f64) -> f64 { c * 9.0 / 5.0 + 32.0 };",
                display: Some("fn c_to_f(c: f64) -> f64 {\n  c * 9.0 / 5.0 + 32.0\n};"),
                hint: Some("Use fn name(param: f64) -> f64 { expression } to define a function. The last expression is returned implicitly."),
            },
            ExerciseStep {
                label: "Define Fahrenheit to Celsius",
                code: "fn f_to_c(f: f64) -> f64 { (f - 32.0) * 5.0 / 9.0 };",
                display: Some("fn f_to_c(f: f64) -> f64 {\n  (f - 32.0) * 5.0 / 9.0\n};"),
                hint: Some("Same pattern: fn name(param: f64) -> f64 { body }. The formula is (f - 32) * 5 / 9."),
            },
            ExerciseStep {
                label: "Convert body temperature",
                code: r#"println!("37°C = {:.1}°F", c_to_f(37.0));"#,
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Convert boiling point",
                code: r#"println!("212°F = {:.1}°C", f_to_c(212.0));"#,
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Print a conversion table",
                code: r#"for c in (0..=100).step_by(25) { println!("{}°C = {:.1}°F", c, c_to_f(c as f64)); }"#,
                display: Some("for c in (0..=100).step_by(25) {\n  println!(\n    \"{}°C = {:.1}°F\",\n    c, c_to_f(c as f64)\n  );\n}"),
                hint: None,
            },
        ],
    },
    // 21: Mini Project - FizzBuzz
    Exercise {
        title: "Mini Project: FizzBuzz",
        description: "The classic FizzBuzz challenge in Rust! Print numbers 1-20, but replace multiples of 3 with 'Fizz', multiples of 5 with 'Buzz', and multiples of both with 'FizzBuzz'.",
        theory: "FizzBuzz is a classic programming exercise. Practice for loops, the modulo operator (%), and conditional logic. In Rust, use if/else if/else chains or match to handle the three cases: divisible by 3, by 5, or by both.",
        category: "Projects",
        steps: &[
            ExerciseStep {
                label: "Define the fizzbuzz function",
                code: r#"fn fizzbuzz(n: u32) -> String { match (n % 3, n % 5) { (0, 0) => "FizzBuzz".into(), (0, _) => "Fizz".into(), (_, 0) => "Buzz".into(), _ => n.to_string() } };"#,
                display: Some("fn fizzbuzz(n: u32) -> String {\n  match (n % 3, n % 5) {\n    (0, 0) => \"FizzBuzz\".into(),\n    (0, _) => \"Fizz\".into(),\n    (_, 0) => \"Buzz\".into(),\n    _ => n.to_string(),\n  }\n};"),
                hint: Some("Match on a tuple (n % 3, n % 5) to check divisibility. Use _ as a wildcard pattern and .into() to convert &str to String."),
            },
            ExerciseStep {
                label: "Run FizzBuzz for 1 to 20",
                code: r#"for i in 1..=20 { print!("{} ", fizzbuzz(i)); } println!();"#,
                display: Some("for i in 1..=20 {\n  print!(\"{} \", fizzbuzz(i));\n}\nprintln!();"),
                hint: None,
            },
        ],
    },
    // 22: Mini Project - Word Counter
    Exercise {
        title: "Mini Project: Word Counter",
        description: "Build a word frequency counter using HashMap! This project practices string manipulation, HashMaps, iterators, and sorting — common patterns in real Rust programs.",
        theory: "Count word frequencies using HashMap. Split strings, normalize with to_lowercase(), and use the entry() API for counting. Exercises iterators, string processing, and collections.",
        category: "Projects",
        steps: &[
            ExerciseStep {
                label: "Define the input text",
                code: r#"let text = "the quick brown fox jumps over the lazy dog the fox";"#,
                display: None,
                hint: None,
            },
            ExerciseStep {
                label: "Count word frequencies",
                code: "use std::collections::HashMap; let mut counts: HashMap<&str, u32> = HashMap::new(); for word in text.split_whitespace() { *counts.entry(word).or_insert(0) += 1; }",
                display: Some("use std::collections::HashMap;\nlet mut counts: HashMap<&str, u32> =\n  HashMap::new();\nfor word in text.split_whitespace() {\n  *counts.entry(word).or_insert(0) += 1;\n}"),
                hint: Some("Use .entry(key).or_insert(0) to get a mutable reference to the count, then dereference with * and increment with += 1."),
            },
            ExerciseStep {
                label: "Sort by frequency (descending)",
                code: "let mut sorted: Vec<_> = counts.iter().collect(); sorted.sort_by(|a, b| b.1.cmp(a.1));",
                display: Some("let mut sorted: Vec<_> =\n  counts.iter().collect();\nsorted.sort_by(|a, b| b.1.cmp(a.1));"),
                hint: Some("Collect iterator into a Vec, then use .sort_by(|a, b| b.1.cmp(a.1)) with a closure to sort by the second tuple element in descending order."),
            },
            ExerciseStep {
                label: "Print the results",
                code: r#"for (word, count) in &sorted { println!("{:>6}: {}", word, count); }"#,
                display: Some("for (word, count) in &sorted {\n  println!(\"{:>6}: {}\", word, count);\n}"),
                hint: None,
            },
        ],
    },
    // 23: Mini Project - Fibonacci
    Exercise {
        title: "Mini Project: Fibonacci",
        description: "Implement Fibonacci numbers using both iterative and functional approaches. Compare how Rust lets you write the same algorithm in different styles, all with the same performance.",
        theory: "The Fibonacci sequence (0, 1, 1, 2, 3, 5, 8...) is great for practicing loops and recursion. In Rust, you can implement it iteratively with a loop and tuple swapping, or functionally using iterators and .take().collect().",
        category: "Projects",
        steps: &[
            ExerciseStep {
                label: "Iterative Fibonacci function",
                code: "fn fib(n: u64) -> u64 { let (mut a, mut b) = (0, 1); for _ in 0..n { let tmp = a; a = b; b = tmp + b; } a };",
                display: Some("fn fib(n: u64) -> u64 {\n  let (mut a, mut b) = (0, 1);\n  for _ in 0..n {\n    let tmp = a;\n    a = b;\n    b = tmp + b;\n  }\n  a\n};"),
                hint: Some("Define fn fib(n: u64) -> u64 with mutable variables. Swap values in a loop: save a, set a = b, set b = saved + b."),
            },
            ExerciseStep {
                label: "Print first 15 Fibonacci numbers",
                code: r#"let fibs: Vec<u64> = (0..15).map(fib).collect(); println!("Fibonacci: {:?}", fibs);"#,
                display: Some("let fibs: Vec<u64> = (0..15)\n  .map(fib)\n  .collect();\nprintln!(\"Fibonacci: {:?}\", fibs);"),
                hint: Some("Chain (0..15).map(fib).collect() to apply the fib function to each number and gather results into a Vec."),
            },
            ExerciseStep {
                label: "Find first Fibonacci number over 1000",
                code: r#"let big = (0..).map(fib).find(|&x| x > 1000).unwrap(); println!("First fib > 1000: {}", big);"#,
                display: Some("let big = (0..)\n  .map(fib)\n  .find(|&x| x > 1000)\n  .unwrap();\nprintln!(\"First fib > 1000: {}\", big);"),
                hint: Some("Use an infinite range (0..) with .map(fib).find(|&x| condition) to lazily search. The iterator stops at the first match."),
            },
        ],
    },
    // Mini Project - Simple Calculator
    Exercise {
        title: "Mini Project: Calculator",
        description: "Build a calculator using enums and pattern matching! This project demonstrates how Rust's type system lets you model operations cleanly and handle all cases exhaustively.",
        theory: "This project combines parsing, error handling, and pattern matching. Parse input into numbers and operators, use match for the operation, and handle edge cases like division by zero.",
        category: "Projects",
        steps: &[
            ExerciseStep {
                label: "Define operations as an enum",
                code: "#[derive(Debug)] enum Op { Add(f64, f64), Sub(f64, f64), Mul(f64, f64), Div(f64, f64) };",
                display: Some("#[derive(Debug)]\nenum Op {\n  Add(f64, f64),\n  Sub(f64, f64),\n  Mul(f64, f64),\n  Div(f64, f64),\n};"),
                hint: Some("Define enum variants that hold data: enum Name { Variant(Type, Type), ... }. Each variant can carry different data."),
            },
            ExerciseStep {
                label: "Implement calculate function",
                code: r#"fn calc(op: &Op) -> Result<f64, &'static str> { match op { Op::Add(a, b) => Ok(a + b), Op::Sub(a, b) => Ok(a - b), Op::Mul(a, b) => Ok(a * b), Op::Div(a, b) => if *b == 0.0 { Err("Division by zero") } else { Ok(a / b) } } };"#,
                display: Some("fn calc(op: &Op) -> Result<f64, &'static str> {\n  match op {\n    Op::Add(a, b) => Ok(a + b),\n    Op::Sub(a, b) => Ok(a - b),\n    Op::Mul(a, b) => Ok(a * b),\n    Op::Div(a, b) => if *b == 0.0 {\n      Err(\"Division by zero\")\n    } else {\n      Ok(a / b)\n    },\n  }\n};"),
                hint: Some("Return Result<f64, &str> and use match to handle each enum variant. Return Ok(value) for success and Err(msg) for errors."),
            },
            ExerciseStep {
                label: "Run calculations",
                code: r#"let ops = vec![Op::Add(10.0, 5.0), Op::Sub(10.0, 5.0), Op::Mul(10.0, 5.0), Op::Div(10.0, 5.0), Op::Div(1.0, 0.0)];"#,
                display: Some("let ops = vec![\n  Op::Add(10.0, 5.0),\n  Op::Sub(10.0, 5.0),\n  Op::Mul(10.0, 5.0),\n  Op::Div(10.0, 5.0),\n  Op::Div(1.0, 0.0),\n];"),
                hint: None,
            },
            ExerciseStep {
                label: "Print all results",
                code: r#"for op in &ops { match calc(op) { Ok(r) => println!("{:?} = {}", op, r), Err(e) => println!("{:?} = ERROR: {}", op, e) } }"#,
                display: Some("for op in &ops {\n  match calc(op) {\n    Ok(r) => println!(\"{:?} = {}\", op, r),\n    Err(e) => println!(\n      \"{:?} = ERROR: {}\", op, e\n    ),\n  }\n}"),
                hint: Some("Match on the Result from calc(): Ok(r) for the success value, Err(e) for the error message."),
            },
        ],
    },
    // 26: Mini Project - Stack
    Exercise {
        title: "Mini Project: Stack",
        description: "Build a generic Stack data structure from scratch! This project combines generics, Option, Vec, and methods to create a reusable collection type — a common pattern in real Rust libraries.",
        theory: "Implement a generic stack using Vec<T>. Practice generics, Option<T> returns, and method design. A stack is LIFO — push adds to top, pop removes from top.",
        category: "Projects",
        steps: &[
            ExerciseStep {
                label: "Define a generic Stack struct",
                code: "#[derive(Debug)] struct Stack<T> { elements: Vec<T> };",
                display: Some("#[derive(Debug)]\nstruct Stack<T> {\n  elements: Vec<T>,\n};"),
                hint: Some("Use struct Name<T> { field: Vec<T> } to define a generic struct. The <T> makes it work with any type."),
            },
            ExerciseStep {
                label: "Implement Stack methods",
                code: "impl<T> Stack<T> { fn new() -> Self { Stack { elements: Vec::new() } } fn push(&mut self, item: T) { self.elements.push(item); } fn pop(&mut self) -> Option<T> { self.elements.pop() } fn peek(&self) -> Option<&T> { self.elements.last() } fn is_empty(&self) -> bool { self.elements.is_empty() } fn size(&self) -> usize { self.elements.len() } };",
                display: Some("impl<T> Stack<T> {\n  fn new() -> Self {\n    Stack { elements: Vec::new() }\n  }\n  fn push(&mut self, item: T) {\n    self.elements.push(item);\n  }\n  fn pop(&mut self) -> Option<T> {\n    self.elements.pop()\n  }\n  fn peek(&self) -> Option<&T> {\n    self.elements.last()\n  }\n  fn is_empty(&self) -> bool {\n    self.elements.is_empty()\n  }\n  fn size(&self) -> usize {\n    self.elements.len()\n  }\n};"),
                hint: Some("Use impl<T> Stack<T> { ... } to add methods. Use &self for read-only and &mut self for mutating methods. Return Option<T> for operations that may fail."),
            },
            ExerciseStep {
                label: "Create a stack and push items",
                code: r#"let mut stack: Stack<i32> = Stack::new(); stack.push(10); stack.push(20); stack.push(30); println!("Stack: {:?}, size: {}", stack, stack.size());"#,
                display: Some("let mut stack: Stack<i32> = Stack::new();\nstack.push(10);\nstack.push(20);\nstack.push(30);\nprintln!(\n  \"Stack: {:?}, size: {}\",\n  stack, stack.size()\n);"),
                hint: None,
            },
            ExerciseStep {
                label: "Peek and pop",
                code: r#"println!("Peek: {:?}", stack.peek()); let popped = stack.pop(); println!("Popped: {:?}, remaining: {:?}", popped, stack);"#,
                display: Some("println!(\"Peek: {:?}\", stack.peek());\nlet popped = stack.pop();\nprintln!(\n  \"Popped: {:?}, remaining: {:?}\",\n  popped, stack\n);"),
                hint: None,
            },
            ExerciseStep {
                label: "Use with strings too",
                code: r#"let mut names: Stack<&str> = Stack::new(); names.push("Alice"); names.push("Bob"); println!("Names: {:?}, empty: {}", names, names.is_empty());"#,
                display: Some("let mut names: Stack<&str> = Stack::new();\nnames.push(\"Alice\");\nnames.push(\"Bob\");\nprintln!(\n  \"Names: {:?}, empty: {}\",\n  names, names.is_empty()\n);"),
                hint: None,
            },
        ],
    },
];
