pub struct Pair<T, R> {
    pub left: T,
    pub right: R,
}

impl<T, R> Pair<T, R> {
    pub fn new(left: T, right: R) -> Pair<T, R> {
        return Pair { left, right };
    }
}

impl<T, R> PartialEq<Self> for Pair<T, R>
where
    R: Eq,
    T: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.right == other.right && self.left == other.left
    }
}
