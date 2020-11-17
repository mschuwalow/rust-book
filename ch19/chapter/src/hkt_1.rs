use std::iter;

trait Functor<A> {
    type Out<B, F: FnMut(A) -> B>: Functor<B>;

    fn fmap<B, F: FnMut(A) -> B>(self, f: F) -> Self::Out<B, F>;
}

impl<A, E> Functor<A> for Result<A, E> {
    type Out<B, F: FnMut(A) -> B> = Result<B, E>;

    fn fmap<B, F: FnMut(A) -> B>(self, f: F) -> Self::Out<B, F> {
        self.map(f)
    }
}

impl<A> Functor<A> for Option<A> {
    type Out<B, F: FnMut(A) -> B> = Option<B>;

    fn fmap<B, F: FnMut(A) -> B>(self, f: F) -> Self::Out<B, F> {
        self.map(f)
    }
}

impl<A, B, I: Iterator<Item = A>, F: FnMut(A) -> B> Functor<B>
    for iter::Map<I, F>
{
    type Out<C, G: FnMut(B) -> C> = iter::Map<iter::Map<I, F>, G>;

    fn fmap<C, G: FnMut(B) -> C>(self, f: G) -> Self::Out<C, G> {
        self.map(f)
    }
}

// impl<A, T: Iterator<Item = A>> Functor<A> for T {
//     default type Out<B, F: FnMut(A) -> B> = iter::Map<Self, F>;

//     default fn fmap<B, F: FnMut(A) -> B>(self, f: F) -> Self::Out<B, F> {
//         self.map(f)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fmap() {
        let a = Some(3);
        let b = a.fmap(|x| x + 1);
        assert_eq!(b, Some(4));
    }
}
