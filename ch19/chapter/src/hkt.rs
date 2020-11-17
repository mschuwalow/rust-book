use std::iter;

trait HKTFunctor<A> {
    type HigherSelf<B, F>: Functor<B>;
}
trait Functor<A> {
    type HigherSelf<B, F: Fn(A) -> B>: Functor<B>;
    fn map<B, F: Fn(A) -> B>(self, f: F) -> Self::HigherSelf<B, F>;
}
trait FunctorMut<A>: Functor<A> {
    type HigherSelfMut<B, F: FnMut(A) -> B>: FunctorMut<B>;
    fn map_mut<B, F: FnMut(A) -> B>(self, f: F) -> Self::HigherSelfMut<B, F>;
}
trait FunctorOnce<A>: FunctorMut<A> {
    type HigherSelfOnce<B, F: FnOnce(A) -> B>: FunctorOnce<B>;
    fn map_once<B, F: FnOnce(A) -> B>(self, f: F)
        -> Self::HigherSelfOnce<B, F>;
}
// impl<A, T: FunctorOnce<A> + HKTFunctor<A>> FunctorMut<A> for T {
//     fn map_mut<B, F: FnMut(A) -> B>(self, f: F) -> Self::HigherSelf<B> {
//         self.map_once(f)
//     }
// }
// impl<A, T: FunctorMut<A>> Functor<A> for T {
//     type HigherSelf<B, F> = <Self as FunctorMut<A>>::HigherSelfMut<B, F>;

//     fn map<B, F: Fn(A) -> B>(self, f: F) -> Self::HigherSelf<B, F> {
//         self.map_mut(f)
//     }
// }

// impl<A> HKT<A> for Option<A> {
//     type HigherSelf<B> = Option<B>;
// }
// impl<A> FunctorOnce<A> for Option<A> {
//     fn map_once<B, F: FnOnce(A) -> B>(self, f: F) -> Self::HigherSelf<B> {
//         self.map(f)
//     }
// }
impl<A, T: Iterator<Item = A>> Functor<A> for T {
    type HigherSelf<B, F: Fn(A) -> B> = iter::Map<Self, F>;

    fn map<B, F: Fn(A) -> B>(self, f: F) -> iter::Map<Self, F> {
        self.map(f)
    }
}
impl<A, T: Iterator<Item = A> + Functor<A>> FunctorMut<A> for T {
    type HigherSelfMut<B, F: FnMut(A) -> B> = iter::Map<Self, F>;

    fn map_mut<B, F: FnMut(A) -> B>(self, f: F) -> iter::Map<Self, F> {
        Iterator::map(self, f)
    }
}
