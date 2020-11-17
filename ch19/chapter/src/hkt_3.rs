trait FunctorFamily {
    type Functor<T>: Functor<T, Family = Self>;
}
trait Functor<A> {
    type Family: FunctorFamily;

    fn fmap<B, F: FnMut(A) -> B>(
        self,
        f: F,
    ) -> <Self::Family as FunctorFamily>::Functor<B>;
}

trait ApplicativeFamily {
    type Applicative<T>: Applicative<T, Family = Self>;
}
trait Applicative<A> {
    type Family: ApplicativeFamily;

    fn pure(a: A) -> <Self::Family as ApplicativeFamily>::Applicative<A>;
    fn zip<B>(
        self,
        b: <Self::Family as ApplicativeFamily>::Applicative<B>,
    ) -> <Self::Family as ApplicativeFamily>::Applicative<(A, B)>;
}

struct OptionFamily;
impl FunctorFamily for OptionFamily {
    type Functor<T> = Option<T>;
}
impl<A> Functor<A> for Option<A> {
    type Family = OptionFamily;

    fn fmap<B, F: FnMut(A) -> B>(self, f: F) -> Option<B> {
        self.map(f)
    }
}
impl ApplicativeFamily for OptionFamily {
    type Applicative<T> = Option<T>;
}
impl<A> Applicative<A> for Option<A> {
    type Family = OptionFamily;

    fn pure(a: A) -> Option<A> {
        Some(a)
    }

    fn zip<B>(self, b: Option<B>) -> Option<(A, B)> {
        match (self, b) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        }
    }
}

fn use_functor<F: FunctorFamily>(col: F::Functor<i32>) -> F::Functor<i32> {
    col.fmap(|x| x + 1).fmap(|x| x + 1)
}

// fn use_applicative<F: ApplicativeFamily + FunctorFamily>(
//     i: i32,
// ) -> F::Applicative<i32> {
//     <F as ApplicativeFamily>::Applicative::pure(i).fmap(|x| x + 1)
// }
