use leptos::{html::Input, NodeRef, ReadSignal, RwSignal, WriteSignal};

#[derive(Clone, Copy)]
pub struct CodeSetter(pub WriteSignal<String>);

#[derive(Clone, Copy)]
pub struct InputRef(pub ReadSignal<NodeRef<Input>>);

#[derive(Clone, Copy, Default)]
pub struct Progress(pub RwSignal<usize>);
