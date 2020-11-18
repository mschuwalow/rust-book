use std::marker::PhantomData;

trait Semigroup {
    fn combine(self, other: Self) -> Self;
}

trait Monoid: Semigroup {
    fn empty() -> Self;
}

// hkt

trait Family<'a> {
    type Member<T: 'a>: Mirror<'a, T, Family = Self> + 'a;
}
trait Mirror<'a, T> {
    type Family: Family<'a>;
    fn as_member(self) -> <Self::Family as Family<'a>>::Member<T>;
}

trait Functor<'a>: Family<'a> {
    fn fmap<A: 'a, B: 'a, F: FnMut(A) -> B + 'a>(
        fa: Self::Member<A>,
        f: F,
    ) -> Self::Member<B>;
}

trait FunctorSyntax<'a, A: 'a, Fam: Functor<'a>>:
    Mirror<'a, A, Family = Fam> + Sized
{
    fn fmap<B, F: FnMut(A) -> B + 'a>(self, f: F) -> Fam::Member<B> {
        Fam::fmap(self.as_member(), f)
    }
}

impl<'a, A: 'a, F: Functor<'a>, T: Mirror<'a, A, Family = F>>
    FunctorSyntax<'a, A, F> for T
{
}

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

trait ApplicativeSyntax<'a, A: 'a, Fam: Applicative<'a>>:
    Mirror<'a, A, Family = Fam> + Sized
{
    fn zip<B: 'a>(self, fb: Fam::Member<B>) -> Fam::Member<(A, B)> {
        Fam::zip(self.as_member(), fb)
    }
}

impl<'a, A: 'a, F: Applicative<'a>, T: Mirror<'a, A, Family = F>>
    ApplicativeSyntax<'a, A, F> for T
{
}
trait Monad<'a>: Applicative<'a> {
    fn bind<A: 'a, B: 'a, F: FnMut(A) -> Self::Member<B>>(
        fa: Self::Member<A>,
        f: F,
    ) -> Self::Member<B>;
}
trait MonadSyntax<'a, A: 'a, Fam: Monad<'a>>:
    Mirror<'a, A, Family = Fam> + Sized
{
    fn bind<B: 'a, F: FnMut(A) -> Fam::Member<B>>(
        self,
        f: F,
    ) -> Fam::Member<B> {
        Fam::bind(self.as_member(), f)
    }
}
impl<'a, A: 'a, F: Monad<'a>, T: Mirror<'a, A, Family = F>>
    MonadSyntax<'a, A, F> for T
{
}

trait Foldable<'a>: Family<'a> {
    fn fold_map<A: 'a, M: Monoid + 'a, F: FnMut(A) -> M + 'a>(
        fa: Self::Member<A>,
        f: F,
    ) -> M;
}
trait FoldableSyntax<'a, A: 'a, Fam: Foldable<'a>>:
    Mirror<'a, A, Family = Fam> + Sized
{
    fn fold_map<M: Monoid + 'a, F: FnMut(A) -> M + 'a>(self, f: F) -> M {
        Fam::fold_map(self.as_member(), f)
    }
}
impl<'a, A: 'a, F: Foldable<'a>, T: Mirror<'a, A, Family = F>>
    FoldableSyntax<'a, A, F> for T
{
}

trait Traversable<'a>: Foldable<'a> {
    fn traverse<
        App: Applicative<'a>,
        A: 'a,
        B: 'a,
        C: Mirror<'a, B, Family = App>,
        F: FnMut(A) -> C + 'a,
    >(
        fa: Self::Member<A>,
        f: F,
    ) -> App::Member<Self::Member<B>>;
}
trait TraversableSyntax<'a, A: 'a, Fam: Traversable<'a>>:
    Mirror<'a, A, Family = Fam> + Sized
{
    fn traverse<
        App: Applicative<'a>,
        B: 'a,
        C: Mirror<'a, B, Family = App>,
        F: FnMut(A) -> C + 'a,
    >(
        self,
        f: F,
    ) -> App::Member<Fam::Member<B>> {
        Fam::traverse(self.as_member(), f)
    }
}
impl<'a, A: 'a, F: Traversable<'a>, T: Mirror<'a, A, Family = F>>
    TraversableSyntax<'a, A, F> for T
{
}
// usage

struct OptionFamily;
impl<'a> Family<'a> for OptionFamily {
    type Member<T: 'a> = Option<T>;
}
impl<'a, A: 'a> Mirror<'a, A> for Option<A> {
    type Family = OptionFamily;

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
        B: 'a,
        C: Mirror<'a, B, Family = App>,
        F: FnMut(A) -> C + 'a,
    >(
        fa: Option<A>,
        mut f: F,
    ) -> App::Member<Option<B>> {
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
impl<'a, A: 'a> Mirror<'a, A> for Vec<A> {
    type Family = VectorFamily;

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
        B: 'a,
        C: Mirror<'a, B, Family = App>,
        F: FnMut(A) -> C + 'a,
    >(
        fa: Vec<A>,
        mut f: F,
    ) -> App::Member<Vec<B>> {
        let iter = fa.into_iter();
        let acc: Vec<B> = Vec::new();
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
impl<'a, A: 'a, E: 'a> Mirror<'a, A> for Result<A, E> {
    type Family = ResultFamily<'a, E>;

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
impl<'a, T: 'a> Mirror<'a, T> for IteratorWrap<'a, T> {
    type Family = IteratorWrapFamily;

    fn as_member(self) -> <Self::Family as Family<'a>>::Member<T> {
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

fn use_applicative<'a, A, F: Applicative<'a>>(a: A, b: A) -> F::Member<(A, A)> {
    let fa = pure::<F, _>(a);
    let fb = pure::<F, _>(b);
    fa.zip(fb)
}

fn foo() -> Option<(i32, i32)> {
    let a = Some(3);
    let b = a.fmap(|x| (x + 1, x));
    b
}

fn use_iterator() -> IteratorWrap<'static, i32> {
    let v = vec![1];
    v.into_iter().wrap().fmap(|x| x + 1).fmap(|x| x + 1)
}

struct Foo<'a, A> {
    a: A,
    b: &'a i32,
}

fn borrowed<'a, A, B>(
    a: Foo<'a, A>,
    b: Foo<'a, B>,
) -> Option<(Foo<'a, A>, Foo<'a, B>)> {
    pure::<OptionFamily, _>((a, b))
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
    fn test_bind() {
        let a = vec![1, 2, 3].bind(move |x| vec![x, x]);
        assert_eq!(a, vec![1, 1, 2, 2, 3, 3])
    }
}
