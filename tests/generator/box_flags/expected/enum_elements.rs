#[derive(Debug)]
pub struct MyChoiceType {
    pub content: MyChoiceTypeContent,
}
#[derive(Debug)]
pub enum MyChoiceTypeContent {
    A(Box<i32>),
    B(Box<String>),
    C(Box<MySequenceType>),
}
#[derive(Debug)]
pub struct MySequenceType {
    pub a: i32,
    pub b: String,
    pub c: Box<MyChoiceType>,
}
