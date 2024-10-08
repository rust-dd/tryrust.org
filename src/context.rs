use leptos::{html::Input, NodeRef, ReadSignal, RwSignal, WriteSignal};

#[derive(Clone, Copy)]
pub struct CodeSetter(pub WriteSignal<String>);

#[derive(Clone, Copy)]
pub struct InputRef(pub ReadSignal<NodeRef<Input>>);

#[derive(Clone, Copy, Default)]
pub struct Progress(pub RwSignal<usize>);

#[derive(Clone)]
pub struct Exercises<'a>
where
    'a: 'static,
{
    pub exercise_01: &'a str,
    pub exercise_02: &'a str,
    pub exercise_03: &'a str,
    pub exercise_04: &'a str,
    pub exercise_05: &'a str,
    pub exercise_06: &'a str,
    pub exercise_07: &'a str,
    pub count: usize,
}

impl<'a> Default for Exercises<'a> {
    fn default() -> Self {
        Self {
            exercise_01: r#"println!("Hello, world!");"#,
            exercise_02: r#"let a: usize = 5;1.let b: usize = 7;2.println!("The sum of {} + {} is: {}", a, b, a + b);"#,
            exercise_03: r#"let mut x = 10;1.x = 15;2.println!("The value of x is: {}", x);"#,
            exercise_04: r#"let mut y = 20;1.let y_ref1 = &y;2.let y_ref2 = &y;3.println!("y_ref1: {}, y_ref2: {}", y_ref1, y_ref2);4.let y_mut_ref = &mut y;5.println!("y_mut_ref: {}", y_mut_ref);"#,
            exercise_05: r#"let tuple = (10, 20);1.println!("The elements are: {} {}", tuple.0, tuple.1);2.let (x, y) = tuple;3.println!("Destructured: x = {}, y = {}", x, y);"#,
            exercise_06: r#"fn greet() { println!("Hi there!"); };1.fn dice_roll() -> i32 { 4 };2.greet();3.let result = dice_roll(); println!("Dice roll result: {}", result);"#,
            exercise_07: r#"#[derive(Debug)] struct Vec { x: i32, y: i32 };1.impl std::ops::Add for Vec { type Output = Self; fn add(self, other: Self) -> Self { Self { x: self.x + other.x, y: self.y + other.y } } };2.let v1 = Vec { x: 1, y: 2 };3.let v2 = Vec { x: 3, y: 4 };4.let result = v1 + v2;5.println!("result: {:?}", result);"#,
            count: 7,
        }
    }
}
