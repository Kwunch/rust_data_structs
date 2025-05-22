pub trait List {
    type Item;
    fn new() -> Self;
    fn append( &mut self, item: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
}

pub trait Node {
    type Item;
    fn new(item: Self::Item) -> Self;
}