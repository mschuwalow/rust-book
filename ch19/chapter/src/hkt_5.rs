use std::marker::PhantomData;

trait Semigroup {
    fn combine(self, other: Self) -> Self;
}

trait Monoid: Semigroup {
    fn empty() -> Self;
}

// hkt
trait Family<'a> {
    type Member<T: 'a>: Mirror<'a, T = T, Family = Self>;
}
trait Mirror<'a>: 'a {
    type T: 'a;
    type Family: Family<'a>;
    fn as_member(self) -> <Self::Family as Family<'a>>::Member<Self::T>;
}
trait Functor<'a>: Family<'a> {
    fn fmap<A: 'a, B: 'a, F: FnMut(A) -> B + 'a>(
        fa: Self::Member<A>,
        f: F,
    ) -> Self::Member<B>;
}

trait FunctorSyntax<'a, Fam: Functor<'a>>:
    Mirror<'a, Family = Fam> + Sized
{
    fn fmap<B, F: FnMut(Self::T) -> B + 'a>(self, f: F) -> Fam::Member<B> {
        Fam::fmap(self.as_member(), f)
    }
}

impl<'a, F: Functor<'a>, T: Mirror<'a, Family = F>> FunctorSyntax<'a, F> for T {}

trait Applicative<'a>: Functor<'a> {
    fn pure<A: 'a>(a: A) -> Self::Member<A>;
    fn zip<A: 'a, B: 'a>(
        fa: Self::Member<A>,
        fb: Self::Member<B>,
    ) -> Self::Member<(A, B)>;
}

fn pure<'a, F: Applicative<'a>, A: 'a>(a: A) -> F::Member<A> {
    F::pure(a)
}

trait ApplicativeSyntax<'a, App: Applicative<'a>>:
    Mirror<'a, Family = App> + Sized
{
    fn zip<B: 'a>(self, fb: App::Member<B>) -> App::Member<(Self::T, B)> {
        App::zip(self.as_member(), fb)
    }
}

impl<'a, App: Applicative<'a>, T: Mirror<'a, Family = App>>
    ApplicativeSyntax<'a, App> for T
{
}
trait Monad<'a>: Applicative<'a> {
    fn bind<A: 'a, B: 'a, F: FnMut(A) -> Self::Member<B>>(
        fa: Self::Member<A>,
        f: F,
    ) -> Self::Member<B>;
}
trait MonadSyntax<'a, Mon: Monad<'a>>: Mirror<'a, Family = Mon> + Sized {
    fn bind<B: 'a, F: FnMut(Self::T) -> Mon::Member<B>>(
        self,
        f: F,
    ) -> Mon::Member<B> {
        Mon::bind(self.as_member(), f)
    }
}
impl<'a, F: Monad<'a>, T: Mirror<'a, Family = F>> MonadSyntax<'a, F> for T {}

trait Foldable<'a>: Family<'a> {
    fn fold_map<A: 'a, M: Monoid + 'a, F: FnMut(A) -> M + 'a>(
        fa: Self::Member<A>,
        f: F,
    ) -> M;
}
trait FoldableSyntax<'a, Fam: Foldable<'a>>:
    Mirror<'a, Family = Fam> + Sized
{
    fn fold_map<M: Monoid + 'a, F: FnMut(Self::T) -> M + 'a>(self, f: F) -> M {
        Fam::fold_map(self.as_member(), f)
    }
}
impl<'a, F: Foldable<'a>, T: Mirror<'a, Family = F>> FoldableSyntax<'a, F>
    for T
{
}

trait Traversable<'a>: Foldable<'a> {
    fn traverse<
        App: Applicative<'a>,
        A: 'a,
        AppB: Mirror<'a, Family = App>,
        F: FnMut(A) -> AppB + 'a,
    >(
        fa: Self::Member<A>,
        f: F,
    ) -> App::Member<Self::Member<AppB::T>>;
}
trait TraversableSyntax<'a, Tr: Traversable<'a>>:
    Mirror<'a, Family = Tr> + Sized
{
    fn traverse<
        App: Applicative<'a>,
        B: 'a,
        AppB: Mirror<'a, T = B, Family = App>,
        F: FnMut(Self::T) -> AppB + 'a,
    >(
        self,
        f: F,
    ) -> App::Member<Tr::Member<B>> {
        Tr::traverse(self.as_member(), f)
    }
}
impl<'a, F: Traversable<'a>, T: Mirror<'a, Family = F>> TraversableSyntax<'a, F>
    for T
{
}

trait SequenceSyntax<
    'a,
    App: Applicative<'a>,
    Tr: Traversable<'a>,
    AppA: Mirror<'a, Family = App>,
>: Mirror<'a, T = AppA, Family = Tr> + Sized
{
    fn sequence(self) -> App::Member<Tr::Member<AppA::T>> {
        self.as_member().traverse(|x| x.as_member())
    }
}
impl<
        'a,
        App: Applicative<'a>,
        Tr: Traversable<'a>,
        AppA: Mirror<'a, Family = App>,
        TrAppA: Mirror<'a, T = AppA, Family = Tr>,
    > SequenceSyntax<'a, App, Tr, AppA> for TrAppA
{
}
// usage

struct OptionFamily;
impl<'a> Family<'a> for OptionFamily {
    type Member<T: 'a> = Option<T>;
}
impl<'a, A: 'a> Mirror<'a> for Option<A> {
    type Family = OptionFamily;
    type T = A;

    fn as_member(self) -> Option<A> {
        self
    }
}
impl<'a> Functor<'a> for OptionFamily {
    fn fmap<A, B, F: FnMut(A) -> B>(fa: Option<A>, f: F) -> Option<B> {
        fa.map(f)
    }
}
impl<'a> Applicative<'a> for OptionFamily {
    fn pure<A>(a: A) -> Option<A> {
        Some(a)
    }

    fn zip<A, B>(fa: Option<A>, fb: Option<B>) -> Option<(A, B)> {
        match (fa, fb) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        }
    }
}
impl<'a> Foldable<'a> for OptionFamily {
    fn fold_map<A: 'a, M: Monoid + 'a, F: FnMut(A) -> M + 'a>(
        fa: Option<A>,
        mut f: F,
    ) -> M {
        match fa {
            Some(a) => f(a),
            None => M::empty(),
        }
    }
}
impl<'a> Traversable<'a> for OptionFamily {
    fn traverse<
        App: Applicative<'a>,
        A: 'a,
        AppB: Mirror<'a, Family = App>,
        F: FnMut(A) -> AppB + 'a,
    >(
        fa: Option<A>,
        mut f: F,
    ) -> App::Member<Option<AppB::T>> {
        match fa {
            None => App::pure(None),
            Some(a) => f(a).fmap(|x| Some(x)),
        }
    }
}
// vector

struct VectorFamily;
impl<'a> Family<'a> for VectorFamily {
    type Member<T: 'a> = Vec<T>;
}
impl<'a, A: 'a> Mirror<'a> for Vec<A> {
    type Family = VectorFamily;
    type T = A;

    fn as_member(self) -> <Self::Family as Family<'a>>::Member<A> {
        self
    }
}
impl<'a> Functor<'a> for VectorFamily {
    fn fmap<A: 'a, B: 'a, F: FnMut(A) -> B + 'a>(fa: Vec<A>, f: F) -> Vec<B> {
        fa.into_iter().map(f).collect()
    }
}
impl<'a> Applicative<'a> for VectorFamily {
    fn pure<A: 'a>(a: A) -> Vec<A> {
        vec![a]
    }

    fn zip<A: 'a, B: 'a>(fa: Vec<A>, fb: Vec<B>) -> Vec<(A, B)> {
        let mut result = Vec::new();
        let mut iter_a = fa.into_iter();
        let mut iter_b = fb.into_iter();
        while let (Some(a), Some(b)) = (iter_a.next(), iter_b.next()) {
            result.push((a, b));
        }
        result
    }
}
impl<'a> Monad<'a> for VectorFamily {
    fn bind<A: 'a, B: 'a, F: FnMut(A) -> Vec<B>>(
        fa: Vec<A>,
        mut f: F,
    ) -> Vec<B> {
        let mut result = Vec::new();
        for a in fa {
            for b in f(a) {
                result.push(b);
            }
        }
        result
    }
}
impl<'a> Foldable<'a> for VectorFamily {
    fn fold_map<A: 'a, M: Monoid + 'a, F: FnMut(A) -> M + 'a>(
        fa: Vec<A>,
        mut f: F,
    ) -> M {
        let iter = fa.into_iter();
        iter.fold(M::empty(), move |m, a| M::combine(m, f(a)))
    }
}
impl<'a> Traversable<'a> for VectorFamily {
    fn traverse<
        App: Applicative<'a>,
        A: 'a,
        AppB: Mirror<'a, Family = App>,
        F: FnMut(A) -> AppB + 'a,
    >(
        fa: Vec<A>,
        mut f: F,
    ) -> App::Member<Vec<AppB::T>> {
        let iter = fa.into_iter();
        let acc = Vec::new();
        iter.fold(App::pure(acc), move |xs, x| {
            let next = f(x);
            xs.zip(next.as_member()).fmap(move |(mut xs, x)| {
                xs.push(x);
                xs
            })
        })
    }
}

// Result
struct ResultFamily<'a, E: 'a> {
    phantom: PhantomData<&'a E>,
}
impl<'a, E: 'a> Family<'a> for ResultFamily<'a, E> {
    type Member<T: 'a> = Result<T, E>;
}
impl<'a, A: 'a, E: 'a> Mirror<'a> for Result<A, E> {
    type Family = ResultFamily<'a, E>;
    type T = A;

    fn as_member(self) -> <Self::Family as Family<'a>>::Member<A> {
        self
    }
}
impl<'a, E: 'a> Functor<'a> for ResultFamily<'a, E> {
    fn fmap<A, B, F: FnMut(A) -> B>(fa: Result<A, E>, f: F) -> Result<B, E> {
        fa.map(f)
    }
}
impl<'a, E: 'a> Applicative<'a> for ResultFamily<'a, E> {
    fn pure<A>(a: A) -> Result<A, E> {
        Ok(a)
    }

    fn zip<A, B>(fa: Result<A, E>, fb: Result<B, E>) -> Result<(A, B), E> {
        fa.and_then(|a| fb.map(|b| (a, b)))
    }
}

// iterator
struct IteratorWrap<'a, T>(Box<dyn Iterator<Item = T> + 'a>);
trait IteratorSyntax<'a, T: Sized>: Iterator<Item = T> + Sized + 'a {
    fn wrap(self) -> IteratorWrap<'a, T> {
        IteratorWrap(Box::new(self))
    }
}
impl<'a, T, I: Iterator<Item = T> + 'a> IteratorSyntax<'a, T> for I {}
impl<'a, T> Iterator for IteratorWrap<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

// iterator
struct IteratorWrapFamily;
impl<'a> Family<'a> for IteratorWrapFamily {
    type Member<T: 'a> = IteratorWrap<'a, T>;
}
impl<'a, A: 'a> Mirror<'a> for IteratorWrap<'a, A> {
    type Family = IteratorWrapFamily;
    type T = A;

    fn as_member(self) -> <Self::Family as Family<'a>>::Member<A> {
        self
    }
}
impl<'a> Functor<'a> for IteratorWrapFamily {
    fn fmap<A: 'a, B, F: FnMut(A) -> B + 'a>(
        fa: IteratorWrap<'a, A>,
        f: F,
    ) -> IteratorWrap<'a, B> {
        IteratorWrap(Box::new(fa.0.into_iter().map(f).into_iter()))
    }
}
impl<'a> Applicative<'a> for IteratorWrapFamily {
    fn pure<A: 'a>(a: A) -> IteratorWrap<'a, A> {
        vec![a].into_iter().wrap()
    }

    fn zip<A: 'a, B: 'a>(
        mut fa: IteratorWrap<A>,
        mut fb: IteratorWrap<B>,
    ) -> IteratorWrap<'a, (A, B)> {
        let mut result = Vec::new();
        while let (Some(a), Some(b)) = (fa.next(), fb.next()) {
            result.push((a, b));
        }
        result.into_iter().wrap()
    }
}

//
struct OptionTFamily<'a, F> {
    phantom: PhantomData<&'a F>,
}
struct OptionT<'a, F: Family<'a>, A: 'a> {
    run:      F::Member<Option<A>>,
    _phantom: PhantomData<A>,
}
impl<'a, F: Family<'a>, A: 'a> OptionT<'a, F, A> {
    fn of(value: F::Member<Option<A>>) -> Self {
        OptionT {
            run:      value,
            _phantom: PhantomData,
        }
    }

    fn run(self) -> F::Member<Option<A>> {
        self.run
    }
}

impl<'a, F: Family<'a>> Family<'a> for OptionTFamily<'a, F> {
    type Member<T: 'a> = OptionT<'a, F, T>;
}
impl<'a, F: Family<'a> + 'a, A: 'a> Mirror<'a> for OptionT<'a, F, A> {
    type Family = OptionTFamily<'a, F>;
    type T = A;

    fn as_member(self) -> <Self::Family as Family<'a>>::Member<A> {
        self
    }
}
impl<'a, M: Functor<'a>> Functor<'a> for OptionTFamily<'a, M> {
    fn fmap<A: 'a, B: 'a, F: FnMut(A) -> B + 'a>(
        fa: OptionT<'a, M, A>,
        mut f: F,
    ) -> OptionT<'a, M, B> {
        OptionT::of(fa.run().fmap(move |x| x.fmap(|y| f(y))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_traverse() {
        let a = vec![1, 2, 3].traverse(|x| Some(x));
        assert_eq!(a, Some(vec![1, 2, 3]));
    }
    #[test]
    fn test_sequence() {
        let a = vec![Some(1), None, Some(3)].sequence();
        assert_eq!(a, None);
    }
    #[test]
    fn test_bind() {
        let a = vec![1, 2, 3].bind(move |x| vec![x, x]);
        assert_eq!(a, vec![1, 1, 2, 2, 3, 3])
    }
}
