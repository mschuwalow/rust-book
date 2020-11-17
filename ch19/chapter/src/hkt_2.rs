use std::iter;

trait Functor<A> {
    type Out<B>: Functor<B>;
    fn fmap<B, F: FnMut(A) -> B>(self, f: F) -> Self::Out<B>;
}

impl<A> Functor<A> for Option<A> {
    type Out<B> = Option<B>;

    fn fmap<B, F: FnMut(A) -> B>(self, f: F) -> Self::Out<B> {
        self.map(f)
    }
}

impl<A, E> Functor<A> for Result<A, E> {
    type Out<B> = Result<B, E>;

    fn fmap<B, F: FnMut(A) -> B>(self, f: F) -> Self::Out<B> {
        self.map(f)
    }
}

impl<A> Functor<A> for Vec<A> {
    type Out<B> = Vec<B>;

    fn fmap<B, F: FnMut(A) -> B>(self, f: F) -> Self::Out<B> {
        self.into_iter().map(f).collect()
    }
}

fn use_functor<F: Functor<i32>>(
    col: F,
) -> <<F as Functor<i32>>::Out<i32> as Functor<i32>>::Out<i32> {
    let result = col.fmap(|x| x + 1).fmap(|x| x + 1);
    result
}

fn foo() -> Option<i32> {
    let a = Some(1);
    use_functor(a)
}
