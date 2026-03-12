#[derive(Clone, Debug)]
pub struct ExerciseStep {
    pub label: &'static str,
    pub code: &'static str,
    /// Pretty-printed version for display. Falls back to `code` if None.
    pub display: Option<&'static str>,
}

#[derive(Clone, Debug)]
pub struct Exercise {
    pub title: &'static str,
    pub description: &'static str,
    pub category: &'static str,
    pub steps: &'static [ExerciseStep],
}

pub static EXERCISES: &[Exercise] = &[
    // 0: Hello World
    Exercise {
        title: "Hello, World!",
        description: "Every journey starts with Hello World. Rust uses macros (ending with !) for formatted output. println! writes to stdout with a newline.",
        category: "Basics",
        steps: &[ExerciseStep {
            label: "Print \"Hello, world!\" to the console",
            code: r#"println!("Hello, world!");"#,
            display: None,
        }],
    },
    // 1: Variables & Types
    Exercise {
        title: "Variables & Types",
        description: "Rust is statically typed. Variables are immutable by default. Use `let` to declare, and add type annotations when needed. Rust has integers (i32, u64, usize...), floats (f32, f64), booleans, and chars.",
        category: "Basics",
        steps: &[
            ExerciseStep {
                label: "Declare an integer",
                code: "let age: u32 = 25;",
                display: None,
            },
            ExerciseStep {
                label: "Declare a float",
                code: "let pi: f64 = 3.14159;",
                display: None,
            },
            ExerciseStep {
                label: "Declare a boolean",
                code: "let is_rustacean: bool = true;",
                display: None,
            },
            ExerciseStep {
                label: "Declare a character",
                code: "let ferris: char = '🦀';",
                display: None,
            },
            ExerciseStep {
                label: "Print all values",
                code: r#"println!("age={}, pi={:.2}, rustacean={}, mascot={}", age, pi, is_rustacean, ferris);"#,
                display: Some("println!(\n  \"age={}, pi={:.2}, rustacean={}, mascot={}\",\n  age, pi, is_rustacean, ferris\n);"),
            },
        ],
    },
    // 2: Mutability
    Exercise {
        title: "Mutability",
        description: "Variables are immutable by default in Rust. Use `mut` to allow changes. This design choice helps prevent bugs — you only opt into mutability when you actually need it.",
        category: "Basics",
        steps: &[
            ExerciseStep {
                label: "Declare a mutable variable",
                code: "let mut score = 0;",
                display: None,
            },
            ExerciseStep {
                label: "Increase the score",
                code: "score += 10;",
                display: None,
            },
            ExerciseStep {
                label: "Increase again",
                code: "score += 25;",
                display: None,
            },
            ExerciseStep {
                label: "Print the final score",
                code: r#"println!("Final score: {}", score);"#,
                display: None,
            },
        ],
    },
    // 3: Strings
    Exercise {
        title: "Strings",
        description: "Rust has two main string types: `String` (owned, heap-allocated, growable) and `&str` (borrowed string slice). String literals are `&str`. Use `.to_string()` or `String::from()` to create owned strings.",
        category: "Basics",
        steps: &[
            ExerciseStep {
                label: "Create an owned String",
                code: r#"let mut greeting = String::from("Hello");"#,
                display: None,
            },
            ExerciseStep {
                label: "Append to the string",
                code: r#"greeting.push_str(", Rust!");"#,
                display: None,
            },
            ExerciseStep {
                label: "Get the length",
                code: "let len = greeting.len();",
                display: None,
            },
            ExerciseStep {
                label: "Check if it contains a word",
                code: r#"let has_rust = greeting.contains("Rust");"#,
                display: None,
            },
            ExerciseStep {
                label: "Print the results",
                code: r#"println!("'{}' (len={}, has_rust={})", greeting, len, has_rust);"#,
                display: Some("println!(\n  \"'{}' (len={}, has_rust={})\",\n  greeting, len, has_rust\n);"),
            },
        ],
    },
    // 4: Tuples & Destructuring
    Exercise {
        title: "Tuples & Destructuring",
        description: "Tuples group values of different types into one compound type. Access elements by index (tuple.0) or destructure them with pattern matching. Tuples are useful for returning multiple values from a function.",
        category: "Basics",
        steps: &[
            ExerciseStep {
                label: "Create a tuple",
                code: r#"let person = ("Alice", 30, true);"#,
                display: None,
            },
            ExerciseStep {
                label: "Access by index",
                code: r#"println!("Name: {}", person.0);"#,
                display: None,
            },
            ExerciseStep {
                label: "Destructure the tuple",
                code: "let (name, age, active) = person;",
                display: None,
            },
            ExerciseStep {
                label: "Print destructured values",
                code: r#"println!("{} is {} years old, active: {}", name, age, active);"#,
                display: Some("println!(\n  \"{} is {} years old, active: {}\",\n  name, age, active\n);"),
            },
        ],
    },
    // 5: Arrays & Iteration
    Exercise {
        title: "Arrays & Iteration",
        description: "Arrays have a fixed size and contain elements of the same type. Use `[T; N]` syntax. Iterate with `for..in` or use iterator methods like `.iter()`, `.map()`, `.filter()`.",
        category: "Basics",
        steps: &[
            ExerciseStep {
                label: "Create an array",
                code: "let numbers = [1, 2, 3, 4, 5];",
                display: None,
            },
            ExerciseStep {
                label: "Access by index",
                code: r#"println!("First: {}, Last: {}", numbers[0], numbers[4]);"#,
                display: Some("println!(\n  \"First: {}, Last: {}\",\n  numbers[0], numbers[4]\n);"),
            },
            ExerciseStep {
                label: "Calculate the sum",
                code: "let sum: i32 = numbers.iter().sum();",
                display: None,
            },
            ExerciseStep {
                label: "Find the max",
                code: "let max = numbers.iter().max().unwrap();",
                display: None,
            },
            ExerciseStep {
                label: "Print results",
                code: r#"println!("Sum: {}, Max: {}, Len: {}", sum, max, numbers.len());"#,
                display: Some("println!(\n  \"Sum: {}, Max: {}, Len: {}\",\n  sum, max, numbers.len()\n);"),
            },
        ],
    },
    // 6: Vectors
    Exercise {
        title: "Vectors",
        description: "Vec<T> is Rust's growable array type. Unlike arrays, vectors can change size at runtime. Use `vec![]` macro or `Vec::new()` to create them. Vectors are one of the most used collections in Rust.",
        category: "Basics",
        steps: &[
            ExerciseStep {
                label: "Create a vector with the macro",
                code: "let mut fruits = vec![\"apple\", \"banana\"];",
                display: None,
            },
            ExerciseStep {
                label: "Add elements",
                code: r#"fruits.push("cherry"); fruits.push("date");"#,
                display: Some("fruits.push(\"cherry\");\nfruits.push(\"date\");"),
            },
            ExerciseStep {
                label: "Remove the last element",
                code: "let last = fruits.pop().unwrap();",
                display: None,
            },
            ExerciseStep {
                label: "Print the vector and removed item",
                code: r#"println!("Fruits: {:?}, removed: {}", fruits, last);"#,
                display: Some("println!(\n  \"Fruits: {:?}, removed: {}\",\n  fruits, last\n);"),
            },
        ],
    },
    // 7: Control Flow
    Exercise {
        title: "Control Flow",
        description: "Rust has `if/else`, `loop`, `while`, and `for`. Notably, `if` is an expression — it returns a value. There's no ternary operator because `if/else` already fills that role.",
        category: "Basics",
        steps: &[
            ExerciseStep {
                label: "Use if as an expression",
                code: r#"let temp = 22; let weather = if temp > 30 { "hot" } else if temp > 15 { "nice" } else { "cold" };"#,
                display: Some("let temp = 22;\nlet weather = if temp > 30 {\n  \"hot\"\n} else if temp > 15 {\n  \"nice\"\n} else {\n  \"cold\"\n};"),
            },
            ExerciseStep {
                label: "Print the weather",
                code: r#"println!("{}°C is {}", temp, weather);"#,
                display: None,
            },
            ExerciseStep {
                label: "Use a for loop with range",
                code: r#"for i in 1..=5 { print!("{} ", i * i); }"#,
                display: Some("for i in 1..=5 {\n  print!(\"{} \", i * i);\n}"),
            },
            ExerciseStep {
                label: "Print newline after squares",
                code: r#"println!();"#,
                display: None,
            },
        ],
    },
    // 8: Functions
    Exercise {
        title: "Functions",
        description: "Functions are declared with `fn`. Parameters need type annotations. The return type follows `->`. Rust functions return the last expression implicitly (no semicolon = return value).",
        category: "Intermediate",
        steps: &[
            ExerciseStep {
                label: "Define a function",
                code: "fn square(n: i32) -> i32 { n * n };",
                display: Some("fn square(n: i32) -> i32 {\n  n * n\n};"),
            },
            ExerciseStep {
                label: "Define a function with multiple params",
                code: r#"fn greet(name: &str, times: u32) { for _ in 0..times { println!("Hello, {}!", name); } };"#,
                display: Some("fn greet(name: &str, times: u32) {\n  for _ in 0..times {\n    println!(\"Hello, {}!\", name);\n  }\n};"),
            },
            ExerciseStep {
                label: "Call square",
                code: r#"println!("7² = {}", square(7));"#,
                display: None,
            },
            ExerciseStep {
                label: "Call greet",
                code: r#"greet("Ferris", 2);"#,
                display: None,
            },
        ],
    },
    // 9: Closures
    Exercise {
        title: "Closures",
        description: "Closures are anonymous functions that can capture variables from their environment. They use `|params|` syntax. Closures are widely used with iterator methods like `.map()`, `.filter()`, and `.for_each()`.",
        category: "Intermediate",
        steps: &[
            ExerciseStep {
                label: "Create a simple closure",
                code: "let double = |x: i32| x * 2;",
                display: None,
            },
            ExerciseStep {
                label: "Use it",
                code: r#"println!("double(5) = {}", double(5));"#,
                display: None,
            },
            ExerciseStep {
                label: "Use closures with iterators",
                code: "let nums = vec![1, 2, 3, 4, 5]; let evens: Vec<i32> = nums.iter().filter(|&&x| x % 2 == 0).cloned().collect();",
                display: Some("let nums = vec![1, 2, 3, 4, 5];\nlet evens: Vec<i32> = nums\n  .iter()\n  .filter(|&&x| x % 2 == 0)\n  .cloned()\n  .collect();"),
            },
            ExerciseStep {
                label: "Print filtered results",
                code: r#"println!("Evens: {:?}", evens);"#,
                display: None,
            },
        ],
    },
    // 10: Ownership
    Exercise {
        title: "Ownership",
        description: "Ownership is Rust's most unique feature. Each value has exactly one owner. When the owner goes out of scope, the value is dropped. Assigning or passing a heap value moves it — the original variable becomes invalid.",
        category: "Intermediate",
        steps: &[
            ExerciseStep {
                label: "Create an owned string",
                code: r#"let s1 = String::from("hello");"#,
                display: None,
            },
            ExerciseStep {
                label: "Clone instead of move",
                code: "let s2 = s1.clone();",
                display: None,
            },
            ExerciseStep {
                label: "Both are still valid after clone",
                code: r#"println!("s1={}, s2={}", s1, s2);"#,
                display: None,
            },
            ExerciseStep {
                label: "Integers implement Copy (no move)",
                code: "let a = 42; let b = a;",
                display: Some("let a = 42;\nlet b = a;"),
            },
            ExerciseStep {
                label: "Both ints are still valid",
                code: r#"println!("a={}, b={}", a, b);"#,
                display: None,
            },
        ],
    },
    // 11: References & Borrowing
    Exercise {
        title: "References & Borrowing",
        description: "References let you use a value without taking ownership. `&T` is an immutable reference, `&mut T` is a mutable one. Rule: you can have many &T OR one &mut T, never both at the same time.",
        category: "Intermediate",
        steps: &[
            ExerciseStep {
                label: "Create a string",
                code: r#"let mut text = String::from("hello");"#,
                display: None,
            },
            ExerciseStep {
                label: "Define a function that borrows",
                code: "fn count_chars(s: &str) -> usize { s.len() };",
                display: Some("fn count_chars(s: &str) -> usize {\n  s.len()\n};"),
            },
            ExerciseStep {
                label: "Borrow immutably",
                code: r#"let len = count_chars(&text); println!("Length: {}", len);"#,
                display: Some("let len = count_chars(&text);\nprintln!(\"Length: {}\", len);"),
            },
            ExerciseStep {
                label: "Borrow mutably and modify",
                code: r#"text.push_str(" world"); println!("Modified: {}", text);"#,
                display: Some("text.push_str(\" world\");\nprintln!(\"Modified: {}\", text);"),
            },
        ],
    },
    // 12: Structs
    Exercise {
        title: "Structs",
        description: "Structs let you create custom data types by grouping related values. Define methods and associated functions with `impl` blocks. Derive macros like `#[derive(Debug)]` add common trait implementations automatically.",
        category: "Intermediate",
        steps: &[
            ExerciseStep {
                label: "Define a struct",
                code: "#[derive(Debug)] struct Rectangle { width: f64, height: f64 };",
                display: Some("#[derive(Debug)]\nstruct Rectangle {\n  width: f64,\n  height: f64,\n};"),
            },
            ExerciseStep {
                label: "Add methods with impl",
                code: "impl Rectangle { fn area(&self) -> f64 { self.width * self.height } fn is_square(&self) -> bool { (self.width - self.height).abs() < f64::EPSILON } };",
                display: Some("impl Rectangle {\n  fn area(&self) -> f64 {\n    self.width * self.height\n  }\n  fn is_square(&self) -> bool {\n    (self.width - self.height).abs()\n      < f64::EPSILON\n  }\n};"),
            },
            ExerciseStep {
                label: "Create an instance",
                code: "let rect = Rectangle { width: 10.0, height: 5.0 };",
                display: Some("let rect = Rectangle {\n  width: 10.0,\n  height: 5.0,\n};"),
            },
            ExerciseStep {
                label: "Call methods and print",
                code: r#"println!("{:?} area={}, square={}", rect, rect.area(), rect.is_square());"#,
                display: Some("println!(\n  \"{:?} area={}, square={}\",\n  rect, rect.area(), rect.is_square()\n);"),
            },
        ],
    },
    // 13: Enums & Pattern Matching
    Exercise {
        title: "Enums & Pattern Matching",
        description: "Enums define a type with multiple variants. Combined with `match`, they're incredibly powerful. Rust's `match` must be exhaustive — you must handle every possible variant, making your code safer.",
        category: "Intermediate",
        steps: &[
            ExerciseStep {
                label: "Define an enum with data",
                code: "#[derive(Debug)] enum Shape { Circle(f64), Square(f64), Rect(f64, f64) };",
                display: Some("#[derive(Debug)]\nenum Shape {\n  Circle(f64),\n  Square(f64),\n  Rect(f64, f64),\n};"),
            },
            ExerciseStep {
                label: "Define an area function using match",
                code: "fn area(s: &Shape) -> f64 { match s { Shape::Circle(r) => std::f64::consts::PI * r * r, Shape::Square(s) => s * s, Shape::Rect(w, h) => w * h } };",
                display: Some("fn area(s: &Shape) -> f64 {\n  match s {\n    Shape::Circle(r) =>\n      std::f64::consts::PI * r * r,\n    Shape::Square(s) => s * s,\n    Shape::Rect(w, h) => w * h,\n  }\n};"),
            },
            ExerciseStep {
                label: "Create shapes and calculate areas",
                code: r#"let shapes = vec![Shape::Circle(5.0), Shape::Square(4.0), Shape::Rect(3.0, 7.0)];"#,
                display: Some("let shapes = vec![\n  Shape::Circle(5.0),\n  Shape::Square(4.0),\n  Shape::Rect(3.0, 7.0),\n];"),
            },
            ExerciseStep {
                label: "Print all areas",
                code: r#"for s in &shapes { println!("{:?} -> area = {:.2}", s, area(s)); }"#,
                display: Some("for s in &shapes {\n  println!(\n    \"{:?} -> area = {:.2}\",\n    s, area(s)\n  );\n}"),
            },
        ],
    },
    // 14: Option & Result
    Exercise {
        title: "Option & Result",
        description: "Rust has no null. Instead, `Option<T>` represents an optional value (Some or None) and `Result<T, E>` represents success or failure. Use pattern matching or methods like `.unwrap_or()` to handle them.",
        category: "Intermediate",
        steps: &[
            ExerciseStep {
                label: "Define a safe division function",
                code: r#"fn safe_div(a: f64, b: f64) -> Option<f64> { if b == 0.0 { None } else { Some(a / b) } };"#,
                display: Some("fn safe_div(a: f64, b: f64) -> Option<f64> {\n  if b == 0.0 {\n    None\n  } else {\n    Some(a / b)\n  }\n};"),
            },
            ExerciseStep {
                label: "Handle the Option",
                code: r#"match safe_div(10.0, 3.0) { Some(v) => println!("10/3 = {:.2}", v), None => println!("Cannot divide by zero!") }"#,
                display: Some("match safe_div(10.0, 3.0) {\n  Some(v) => println!(\"10/3 = {:.2}\", v),\n  None => println!(\"Cannot divide by zero!\"),\n}"),
            },
            ExerciseStep {
                label: "Handle the None case",
                code: r#"match safe_div(10.0, 0.0) { Some(v) => println!("10/0 = {:.2}", v), None => println!("Cannot divide by zero!") }"#,
                display: Some("match safe_div(10.0, 0.0) {\n  Some(v) => println!(\"10/0 = {:.2}\", v),\n  None => println!(\"Cannot divide by zero!\"),\n}"),
            },
            ExerciseStep {
                label: "Use unwrap_or with a default",
                code: r#"let result = safe_div(10.0, 0.0).unwrap_or(0.0); println!("With default: {}", result);"#,
                display: Some("let result = safe_div(10.0, 0.0)\n  .unwrap_or(0.0);\nprintln!(\"With default: {}\", result);"),
            },
        ],
    },
    // 15: Error Handling
    Exercise {
        title: "Error Handling",
        description: "Rust encourages explicit error handling with Result<T, E>. The `?` operator propagates errors automatically. Custom error types help create clear, maintainable error handling throughout your application.",
        category: "Intermediate",
        steps: &[
            ExerciseStep {
                label: "Parse a string to a number",
                code: r#"let good: Result<i32, _> = "42".parse(); let bad: Result<i32, _> = "abc".parse();"#,
                display: Some("let good: Result<i32, _> = \"42\".parse();\nlet bad: Result<i32, _> = \"abc\".parse();"),
            },
            ExerciseStep {
                label: "Handle with match",
                code: r#"match good { Ok(n) => println!("Parsed: {}", n), Err(e) => println!("Error: {}", e) }"#,
                display: Some("match good {\n  Ok(n) => println!(\"Parsed: {}\", n),\n  Err(e) => println!(\"Error: {}\", e),\n}"),
            },
            ExerciseStep {
                label: "Handle the error case",
                code: r#"match bad { Ok(n) => println!("Parsed: {}", n), Err(e) => println!("Error: {}", e) }"#,
                display: Some("match bad {\n  Ok(n) => println!(\"Parsed: {}\", n),\n  Err(e) => println!(\"Error: {}\", e),\n}"),
            },
            ExerciseStep {
                label: "Use map and unwrap_or_else",
                code: r#"let doubled = "21".parse::<i32>().map(|n| n * 2).unwrap_or_else(|e| { println!("Error: {}", e); 0 }); println!("Doubled: {}", doubled);"#,
                display: Some("let doubled = \"21\".parse::<i32>()\n  .map(|n| n * 2)\n  .unwrap_or_else(|e| {\n    println!(\"Error: {}\", e);\n    0\n  });\nprintln!(\"Doubled: {}\", doubled);"),
            },
        ],
    },
    // 16: Traits
    Exercise {
        title: "Traits",
        description: "Traits define shared behavior — similar to interfaces in other languages. Types implement traits to provide specific behavior. Trait bounds let you write generic code that works with any type implementing required traits.",
        category: "Advanced",
        steps: &[
            ExerciseStep {
                label: "Define a trait",
                code: "trait Greetable { fn greet(&self) -> String; };",
                display: Some("trait Greetable {\n  fn greet(&self) -> String;\n};"),
            },
            ExerciseStep {
                label: "Define structs",
                code: "struct Dog { name: String }; struct Cat { name: String };",
                display: Some("struct Dog { name: String };\nstruct Cat { name: String };"),
            },
            ExerciseStep {
                label: "Implement trait for Dog",
                code: r#"impl Greetable for Dog { fn greet(&self) -> String { format!("Woof! I'm {}", self.name) } };"#,
                display: Some("impl Greetable for Dog {\n  fn greet(&self) -> String {\n    format!(\"Woof! I'm {}\", self.name)\n  }\n};"),
            },
            ExerciseStep {
                label: "Implement trait for Cat",
                code: r#"impl Greetable for Cat { fn greet(&self) -> String { format!("Meow! I'm {}", self.name) } };"#,
                display: Some("impl Greetable for Cat {\n  fn greet(&self) -> String {\n    format!(\"Meow! I'm {}\", self.name)\n  }\n};"),
            },
            ExerciseStep {
                label: "Use the trait",
                code: r#"let dog = Dog { name: "Rex".into() }; let cat = Cat { name: "Whiskers".into() }; println!("{}", dog.greet()); println!("{}", cat.greet());"#,
                display: Some("let dog = Dog { name: \"Rex\".into() };\nlet cat = Cat { name: \"Whiskers\".into() };\nprintln!(\"{}\", dog.greet());\nprintln!(\"{}\", cat.greet());"),
            },
        ],
    },
    // 17: Generics
    Exercise {
        title: "Generics",
        description: "Generics let you write code that works with multiple types. Combine with trait bounds to constrain what types are accepted. Rust monomorphizes generics at compile time — zero runtime cost!",
        category: "Advanced",
        steps: &[
            ExerciseStep {
                label: "Define a generic function",
                code: "fn largest<T: PartialOrd>(list: &[T]) -> &T { let mut biggest = &list[0]; for item in &list[1..] { if item > biggest { biggest = item; } } biggest };",
                display: Some("fn largest<T: PartialOrd>(list: &[T]) -> &T {\n  let mut biggest = &list[0];\n  for item in &list[1..] {\n    if item > biggest {\n      biggest = item;\n    }\n  }\n  biggest\n};"),
            },
            ExerciseStep {
                label: "Use with integers",
                code: r#"let numbers = vec![34, 50, 25, 100, 65]; println!("Largest number: {}", largest(&numbers));"#,
                display: Some("let numbers = vec![34, 50, 25, 100, 65];\nprintln!(\n  \"Largest number: {}\",\n  largest(&numbers)\n);"),
            },
            ExerciseStep {
                label: "Use with strings",
                code: r#"let words = vec!["rust", "is", "awesome"]; println!("Largest word: {}", largest(&words));"#,
                display: Some("let words = vec![\"rust\", \"is\", \"awesome\"];\nprintln!(\n  \"Largest word: {}\",\n  largest(&words)\n);"),
            },
        ],
    },
    // 18: Iterators
    Exercise {
        title: "Iterator Magic",
        description: "Rust's iterators are lazy and composable. Chain methods like .map(), .filter(), .fold() to build powerful data pipelines. The compiler optimizes them into efficient loops — zero-cost abstractions!",
        category: "Advanced",
        steps: &[
            ExerciseStep {
                label: "Create a range and transform it",
                code: "let squares: Vec<i32> = (1..=10).map(|x| x * x).collect();",
                display: Some("let squares: Vec<i32> = (1..=10)\n  .map(|x| x * x)\n  .collect();"),
            },
            ExerciseStep {
                label: "Print squares",
                code: r#"println!("Squares: {:?}", squares);"#,
                display: None,
            },
            ExerciseStep {
                label: "Chain filter and sum",
                code: "let even_sum: i32 = (1..=20).filter(|x| x % 2 == 0).sum();",
                display: Some("let even_sum: i32 = (1..=20)\n  .filter(|x| x % 2 == 0)\n  .sum();"),
            },
            ExerciseStep {
                label: "Use fold to build a string",
                code: r#"let csv = vec!["a", "b", "c"].iter().enumerate().fold(String::new(), |acc, (i, s)| { if i > 0 { format!("{},{}", acc, s) } else { s.to_string() } }); println!("Even sum: {}, CSV: {}", even_sum, csv);"#,
                display: Some("let csv = vec![\"a\", \"b\", \"c\"]\n  .iter()\n  .enumerate()\n  .fold(String::new(), |acc, (i, s)| {\n    if i > 0 {\n      format!(\"{},{}\", acc, s)\n    } else {\n      s.to_string()\n    }\n  });\nprintln!(\n  \"Even sum: {}, CSV: {}\",\n  even_sum, csv\n);"),
            },
        ],
    },
    // 19: HashMap
    Exercise {
        title: "HashMaps",
        description: "HashMap<K, V> stores key-value pairs with O(1) average lookup. Import from std::collections. Use .entry() API for elegant insert-or-update patterns.",
        category: "Advanced",
        steps: &[
            ExerciseStep {
                label: "Import and create a HashMap",
                code: r#"use std::collections::HashMap; let mut scores: HashMap<&str, i32> = HashMap::new();"#,
                display: Some("use std::collections::HashMap;\nlet mut scores: HashMap<&str, i32> =\n  HashMap::new();"),
            },
            ExerciseStep {
                label: "Insert entries",
                code: r#"scores.insert("Alice", 95); scores.insert("Bob", 87); scores.insert("Carol", 92);"#,
                display: Some("scores.insert(\"Alice\", 95);\nscores.insert(\"Bob\", 87);\nscores.insert(\"Carol\", 92);"),
            },
            ExerciseStep {
                label: "Use entry API",
                code: r#"scores.entry("Dave").or_insert(78); scores.entry("Alice").and_modify(|s| *s += 5);"#,
                display: Some("scores.entry(\"Dave\").or_insert(78);\nscores.entry(\"Alice\")\n  .and_modify(|s| *s += 5);"),
            },
            ExerciseStep {
                label: "Query and iterate",
                code: r#"println!("Alice: {}", scores["Alice"]); for (name, score) in &scores { print!("{}: {} ", name, score); } println!();"#,
                display: Some("println!(\"Alice: {}\", scores[\"Alice\"]);\nfor (name, score) in &scores {\n  print!(\"{}: {} \", name, score);\n}\nprintln!();"),
            },
        ],
    },
    // Operator Overloading
    Exercise {
        title: "Operator Overloading",
        description: "Rust lets you overload operators by implementing traits from std::ops. Add custom + - * behavior to your types. The Debug trait with #[derive] gives you printable output.",
        category: "Advanced",
        steps: &[
            ExerciseStep {
                label: "Define a Point struct",
                code: "#[derive(Debug, Clone, Copy)] struct Point { x: f64, y: f64 };",
                display: Some("#[derive(Debug, Clone, Copy)]\nstruct Point {\n  x: f64,\n  y: f64,\n};"),
            },
            ExerciseStep {
                label: "Implement Add",
                code: "impl std::ops::Add for Point { type Output = Self; fn add(self, other: Self) -> Self { Point { x: self.x + other.x, y: self.y + other.y } } };",
                display: Some("impl std::ops::Add for Point {\n  type Output = Self;\n  fn add(self, other: Self) -> Self {\n    Point {\n      x: self.x + other.x,\n      y: self.y + other.y,\n    }\n  }\n};"),
            },
            ExerciseStep {
                label: "Add a distance method",
                code: "impl Point { fn distance(&self, other: &Point) -> f64 { ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt() } };",
                display: Some("impl Point {\n  fn distance(&self, other: &Point) -> f64 {\n    ((self.x - other.x).powi(2)\n      + (self.y - other.y).powi(2))\n    .sqrt()\n  }\n};"),
            },
            ExerciseStep {
                label: "Create points and use them",
                code: r#"let p1 = Point { x: 1.0, y: 2.0 }; let p2 = Point { x: 4.0, y: 6.0 }; let p3 = p1 + p2; println!("{:?} + {:?} = {:?}, distance = {:.2}", p1, p2, p3, p1.distance(&p2));"#,
                display: Some("let p1 = Point { x: 1.0, y: 2.0 };\nlet p2 = Point { x: 4.0, y: 6.0 };\nlet p3 = p1 + p2;\nprintln!(\n  \"{:?} + {:?} = {:?}, distance = {:.2}\",\n  p1, p2, p3, p1.distance(&p2)\n);"),
            },
        ],
    },
    // Mini Project - Temperature Converter
    Exercise {
        title: "Mini Project: Temp Converter",
        description: "Build a temperature converter! This mini project combines functions, formatting, and f64 arithmetic. You'll create functions to convert between Celsius and Fahrenheit.",
        category: "Projects",
        steps: &[
            ExerciseStep {
                label: "Define Celsius to Fahrenheit",
                code: "fn c_to_f(c: f64) -> f64 { c * 9.0 / 5.0 + 32.0 };",
                display: Some("fn c_to_f(c: f64) -> f64 {\n  c * 9.0 / 5.0 + 32.0\n};"),
            },
            ExerciseStep {
                label: "Define Fahrenheit to Celsius",
                code: "fn f_to_c(f: f64) -> f64 { (f - 32.0) * 5.0 / 9.0 };",
                display: Some("fn f_to_c(f: f64) -> f64 {\n  (f - 32.0) * 5.0 / 9.0\n};"),
            },
            ExerciseStep {
                label: "Convert body temperature",
                code: r#"println!("37°C = {:.1}°F", c_to_f(37.0));"#,
                display: None,
            },
            ExerciseStep {
                label: "Convert boiling point",
                code: r#"println!("212°F = {:.1}°C", f_to_c(212.0));"#,
                display: None,
            },
            ExerciseStep {
                label: "Print a conversion table",
                code: r#"for c in (0..=100).step_by(25) { println!("{}°C = {:.1}°F", c, c_to_f(c as f64)); }"#,
                display: Some("for c in (0..=100).step_by(25) {\n  println!(\n    \"{}°C = {:.1}°F\",\n    c, c_to_f(c as f64)\n  );\n}"),
            },
        ],
    },
    // 21: Mini Project - FizzBuzz
    Exercise {
        title: "Mini Project: FizzBuzz",
        description: "The classic FizzBuzz challenge in Rust! Print numbers 1-20, but replace multiples of 3 with 'Fizz', multiples of 5 with 'Buzz', and multiples of both with 'FizzBuzz'.",
        category: "Projects",
        steps: &[
            ExerciseStep {
                label: "Define the fizzbuzz function",
                code: r#"fn fizzbuzz(n: u32) -> String { match (n % 3, n % 5) { (0, 0) => "FizzBuzz".into(), (0, _) => "Fizz".into(), (_, 0) => "Buzz".into(), _ => n.to_string() } };"#,
                display: Some("fn fizzbuzz(n: u32) -> String {\n  match (n % 3, n % 5) {\n    (0, 0) => \"FizzBuzz\".into(),\n    (0, _) => \"Fizz\".into(),\n    (_, 0) => \"Buzz\".into(),\n    _ => n.to_string(),\n  }\n};"),
            },
            ExerciseStep {
                label: "Run FizzBuzz for 1 to 20",
                code: r#"for i in 1..=20 { print!("{} ", fizzbuzz(i)); } println!();"#,
                display: Some("for i in 1..=20 {\n  print!(\"{} \", fizzbuzz(i));\n}\nprintln!();"),
            },
        ],
    },
    // 22: Mini Project - Word Counter
    Exercise {
        title: "Mini Project: Word Counter",
        description: "Build a word frequency counter using HashMap! This project practices string manipulation, HashMaps, iterators, and sorting — common patterns in real Rust programs.",
        category: "Projects",
        steps: &[
            ExerciseStep {
                label: "Define the input text",
                code: r#"let text = "the quick brown fox jumps over the lazy dog the fox";"#,
                display: None,
            },
            ExerciseStep {
                label: "Count word frequencies",
                code: "use std::collections::HashMap; let mut counts: HashMap<&str, u32> = HashMap::new(); for word in text.split_whitespace() { *counts.entry(word).or_insert(0) += 1; }",
                display: Some("use std::collections::HashMap;\nlet mut counts: HashMap<&str, u32> =\n  HashMap::new();\nfor word in text.split_whitespace() {\n  *counts.entry(word).or_insert(0) += 1;\n}"),
            },
            ExerciseStep {
                label: "Sort by frequency (descending)",
                code: "let mut sorted: Vec<_> = counts.iter().collect(); sorted.sort_by(|a, b| b.1.cmp(a.1));",
                display: Some("let mut sorted: Vec<_> =\n  counts.iter().collect();\nsorted.sort_by(|a, b| b.1.cmp(a.1));"),
            },
            ExerciseStep {
                label: "Print the results",
                code: r#"for (word, count) in &sorted { println!("{:>6}: {}", word, count); }"#,
                display: Some("for (word, count) in &sorted {\n  println!(\"{:>6}: {}\", word, count);\n}"),
            },
        ],
    },
    // 23: Mini Project - Fibonacci
    Exercise {
        title: "Mini Project: Fibonacci",
        description: "Implement Fibonacci numbers using both iterative and functional approaches. Compare how Rust lets you write the same algorithm in different styles, all with the same performance.",
        category: "Projects",
        steps: &[
            ExerciseStep {
                label: "Iterative Fibonacci function",
                code: "fn fib(n: u64) -> u64 { let (mut a, mut b) = (0, 1); for _ in 0..n { let tmp = a; a = b; b = tmp + b; } a };",
                display: Some("fn fib(n: u64) -> u64 {\n  let (mut a, mut b) = (0, 1);\n  for _ in 0..n {\n    let tmp = a;\n    a = b;\n    b = tmp + b;\n  }\n  a\n};"),
            },
            ExerciseStep {
                label: "Print first 15 Fibonacci numbers",
                code: r#"let fibs: Vec<u64> = (0..15).map(fib).collect(); println!("Fibonacci: {:?}", fibs);"#,
                display: Some("let fibs: Vec<u64> = (0..15)\n  .map(fib)\n  .collect();\nprintln!(\"Fibonacci: {:?}\", fibs);"),
            },
            ExerciseStep {
                label: "Find first Fibonacci number over 1000",
                code: r#"let big = (0..).map(fib).find(|&x| x > 1000).unwrap(); println!("First fib > 1000: {}", big);"#,
                display: Some("let big = (0..)\n  .map(fib)\n  .find(|&x| x > 1000)\n  .unwrap();\nprintln!(\"First fib > 1000: {}\", big);"),
            },
        ],
    },
    // Mini Project - Simple Calculator
    Exercise {
        title: "Mini Project: Calculator",
        description: "Build a calculator using enums and pattern matching! This project demonstrates how Rust's type system lets you model operations cleanly and handle all cases exhaustively.",
        category: "Projects",
        steps: &[
            ExerciseStep {
                label: "Define operations as an enum",
                code: "#[derive(Debug)] enum Op { Add(f64, f64), Sub(f64, f64), Mul(f64, f64), Div(f64, f64) };",
                display: Some("#[derive(Debug)]\nenum Op {\n  Add(f64, f64),\n  Sub(f64, f64),\n  Mul(f64, f64),\n  Div(f64, f64),\n};"),
            },
            ExerciseStep {
                label: "Implement calculate function",
                code: r#"fn calc(op: &Op) -> Result<f64, &'static str> { match op { Op::Add(a, b) => Ok(a + b), Op::Sub(a, b) => Ok(a - b), Op::Mul(a, b) => Ok(a * b), Op::Div(a, b) => if *b == 0.0 { Err("Division by zero") } else { Ok(a / b) } } };"#,
                display: Some("fn calc(op: &Op) -> Result<f64, &'static str> {\n  match op {\n    Op::Add(a, b) => Ok(a + b),\n    Op::Sub(a, b) => Ok(a - b),\n    Op::Mul(a, b) => Ok(a * b),\n    Op::Div(a, b) => if *b == 0.0 {\n      Err(\"Division by zero\")\n    } else {\n      Ok(a / b)\n    },\n  }\n};"),
            },
            ExerciseStep {
                label: "Run calculations",
                code: r#"let ops = vec![Op::Add(10.0, 5.0), Op::Sub(10.0, 5.0), Op::Mul(10.0, 5.0), Op::Div(10.0, 5.0), Op::Div(1.0, 0.0)];"#,
                display: Some("let ops = vec![\n  Op::Add(10.0, 5.0),\n  Op::Sub(10.0, 5.0),\n  Op::Mul(10.0, 5.0),\n  Op::Div(10.0, 5.0),\n  Op::Div(1.0, 0.0),\n];"),
            },
            ExerciseStep {
                label: "Print all results",
                code: r#"for op in &ops { match calc(op) { Ok(r) => println!("{:?} = {}", op, r), Err(e) => println!("{:?} = ERROR: {}", op, e) } }"#,
                display: Some("for op in &ops {\n  match calc(op) {\n    Ok(r) => println!(\"{:?} = {}\", op, r),\n    Err(e) => println!(\n      \"{:?} = ERROR: {}\", op, e\n    ),\n  }\n}"),
            },
        ],
    },
    // 26: Mini Project - Stack
    Exercise {
        title: "Mini Project: Stack",
        description: "Build a generic Stack data structure from scratch! This project combines generics, Option, Vec, and methods to create a reusable collection type — a common pattern in real Rust libraries.",
        category: "Projects",
        steps: &[
            ExerciseStep {
                label: "Define a generic Stack struct",
                code: "#[derive(Debug)] struct Stack<T> { elements: Vec<T> };",
                display: Some("#[derive(Debug)]\nstruct Stack<T> {\n  elements: Vec<T>,\n};"),
            },
            ExerciseStep {
                label: "Implement Stack methods",
                code: "impl<T> Stack<T> { fn new() -> Self { Stack { elements: Vec::new() } } fn push(&mut self, item: T) { self.elements.push(item); } fn pop(&mut self) -> Option<T> { self.elements.pop() } fn peek(&self) -> Option<&T> { self.elements.last() } fn is_empty(&self) -> bool { self.elements.is_empty() } fn size(&self) -> usize { self.elements.len() } };",
                display: Some("impl<T> Stack<T> {\n  fn new() -> Self {\n    Stack { elements: Vec::new() }\n  }\n  fn push(&mut self, item: T) {\n    self.elements.push(item);\n  }\n  fn pop(&mut self) -> Option<T> {\n    self.elements.pop()\n  }\n  fn peek(&self) -> Option<&T> {\n    self.elements.last()\n  }\n  fn is_empty(&self) -> bool {\n    self.elements.is_empty()\n  }\n  fn size(&self) -> usize {\n    self.elements.len()\n  }\n};"),
            },
            ExerciseStep {
                label: "Create a stack and push items",
                code: r#"let mut stack: Stack<i32> = Stack::new(); stack.push(10); stack.push(20); stack.push(30); println!("Stack: {:?}, size: {}", stack, stack.size());"#,
                display: Some("let mut stack: Stack<i32> = Stack::new();\nstack.push(10);\nstack.push(20);\nstack.push(30);\nprintln!(\n  \"Stack: {:?}, size: {}\",\n  stack, stack.size()\n);"),
            },
            ExerciseStep {
                label: "Peek and pop",
                code: r#"println!("Peek: {:?}", stack.peek()); let popped = stack.pop(); println!("Popped: {:?}, remaining: {:?}", popped, stack);"#,
                display: Some("println!(\"Peek: {:?}\", stack.peek());\nlet popped = stack.pop();\nprintln!(\n  \"Popped: {:?}, remaining: {:?}\",\n  popped, stack\n);"),
            },
            ExerciseStep {
                label: "Use with strings too",
                code: r#"let mut names: Stack<&str> = Stack::new(); names.push("Alice"); names.push("Bob"); println!("Names: {:?}, empty: {}", names, names.is_empty());"#,
                display: Some("let mut names: Stack<&str> = Stack::new();\nnames.push(\"Alice\");\nnames.push(\"Bob\");\nprintln!(\n  \"Names: {:?}, empty: {}\",\n  names, names.is_empty()\n);"),
            },
        ],
    },
];
