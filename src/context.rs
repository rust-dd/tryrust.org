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
    pub count: usize,
}

impl<'a> Default for Exercises<'a> {
    fn default() -> Self {
        Self {
            exercise_01: r#"println!("Hello, world!");"#,
            exercise_02: r#"let a: usize = 5;1.let b: usize = 7;2.println!("The sum of {} + {} is: {}", a, b, a + b);"#,
            count: 2,
        }
    }
}
