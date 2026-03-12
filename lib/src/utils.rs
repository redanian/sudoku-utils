pub(crate) trait BoolIteratorUtils: Iterator<Item=bool> {
    fn any_true(&mut self) -> bool;
    fn any_true_exhaustive(self) -> bool;
}

impl<I: Iterator<Item=bool>> BoolIteratorUtils for I {
    fn any_true(&mut self) -> bool {
        self.any(|b| b)
    }

    fn any_true_exhaustive(self) -> bool {
        self.fold(false, |acc, b| acc || b)
    }
}
