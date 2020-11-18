use std::marker::PhantomData;

trait Family {
    type Member<'a, T: 'a>: Mirror<'a, T, Family = Self>;
}
trait Mirror<'a, T> {
    type Family: Family;
    fn as_member(self) -> <Self::Family as Family>::Member<'a, T>;
}

trait Functor: Family {
    fn fmap<'a, A: 'a, B: 'a, F: FnMut(A) -> B + 'a>(
        fa: <Self as Family>::Member<'a, A>,
        f: F,
    ) -> <Self as Family>::Member<'a, B>;
}

trait FunctorSyntax<'a, A: 'a, Fam: Functor>:
    Mirror<'a, A, Family = Fam> + Sized
{
    fn fmap<B, F: FnMut(A) -> B + 'a>(
        self,
        f: F,
    ) -> <Fam as Family>::Member<'a, B> {
        <Fam as Functor>::fmap(self.as_member(), f)
    }
}

impl<'a, A: 'a, F: Functor, T: Mirror<'a, A, Family = F>>
    FunctorSyntax<'a, A, F> for T
{
}

trait Applicative: Functor {
    fn pure<'a, A: 'a>(a: A) -> <Self as Family>::Member<'a, A>;
    fn zip<'a, A: 'a, B: 'a>(
        fa: <Self as Family>::Member<'a, A>,
        fb: <Self as Family>::Member<'a, B>,
    ) -> <Self as Family>::Member<'a, (A, B)>;
}

fn pure<'a, F: Applicative, A: 'a>(a: A) -> F::Member<'a, A> {
    <F as Applicative>::pure(a)
}

trait ApplicativeSyntax<'a, A: 'a, Fam: Applicative>:
    Mirror<'a, A, Family = Fam> + Sized
{
    fn zip<B: 'a>(
        self,
        fb: <Fam as Family>::Member<'a, B>,
    ) -> <Fam as Family>::Member<'a, (A, B)> {
        <Fam as Applicative>::zip(self.as_member(), fb)
    }
}

impl<'a, A: 'a, F: Applicative, T: Mirror<'a, A, Family = F>>
    ApplicativeSyntax<'a, A, F> for T
{
}

// usage

struct OptionFamily;
impl Family for OptionFamily {
    type Member<'a, T: 'a> = Option<T>;
}
impl<'a, A: 'a> Mirror<'a, A> for Option<A> {
    type Family = OptionFamily;

    fn as_member(self) -> Option<A> {
        self
    }
}
impl Functor for OptionFamily {
    fn fmap<'a, A: 'a, B: 'a, F: FnMut(A) -> B + 'a>(
        fa: Option<A>,
        f: F,
    ) -> Option<B> {
        fa.map(f)
    }
}
impl Applicative for OptionFamily {
    fn pure<'a, A: 'a>(a: A) -> Option<A> {
        Some(a)
    }

    fn zip<'a, A: 'a, B: 'a>(fa: Option<A>, fb: Option<B>) -> Option<(A, B)> {
        match (fa, fb) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        }
    }
}

// Result
struct ResultFamily<E> {
    phantom: PhantomData<E>,
}
impl<E: 'static> Family for ResultFamily<E> {
    type Member<'a, T: 'a> = Result<T, E>;
}
impl<'a, E: 'static, A: 'a> Mirror<'a, A> for Result<A, E> {
    type Family = ResultFamily<E>;

    fn as_member(self) -> <Self::Family as Family>::Member<'a, A> {
        self
    }
}
impl<E: 'static> Functor for ResultFamily<E> {
    fn fmap<'a, A: 'a, B: 'a, F: FnMut(A) -> B + 'a>(
        fa: Result<A, E>,
        f: F,
    ) -> Result<B, E> {
        fa.map(f)
    }
}
impl<E: 'static> Applicative for ResultFamily<E> {
    fn pure<'a, A: 'a>(a: A) -> Result<A, E> {
        Ok(a)
    }

    fn zip<'a, A: 'a, B: 'a>(
        fa: Result<A, E>,
        fb: Result<B, E>,
    ) -> Result<(A, B), E> {
        fa.and_then(|a| fb.map(|b| (a, b)))
    }
}

struct IteratorWrap<'a, T: Sized>(Box<dyn Iterator<Item = T> + 'a>);
trait IteratorSyntax<'a, T>: Iterator<Item = T> + Sized + 'a {
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
impl Family for IteratorWrapFamily {
    type Member<'a, T: 'a> = IteratorWrap<'a, T>;
}
impl<'a, T: 'a> Mirror<'a, T> for IteratorWrap<'a, T> {
    type Family = IteratorWrapFamily;

    fn as_member(self) -> <Self::Family as Family>::Member<'a, T> {
        self
    }
}
impl Functor for IteratorWrapFamily {
    fn fmap<'a, A: 'a, B: 'a, F: FnMut(A) -> B + 'a>(
        fa: IteratorWrap<'a, A>,
        f: F,
    ) -> IteratorWrap<'a, B> {
        IteratorWrap(Box::new(fa.0.into_iter().map(f).into_iter()))
    }
}
impl Applicative for IteratorWrapFamily {
    fn pure<'a, A: 'a>(a: A) -> IteratorWrap<'a, A> {
        vec![a].into_iter().wrap()
    }

    fn zip<'a, A: 'a, B: 'a>(
        fa: IteratorWrap<A>,
        fb: IteratorWrap<B>,
    ) -> IteratorWrap<'a, (A, B)> {
        let mut result = Vec::new();
        let mut fa = fa;
        let mut fb = fb;
        while let (Some(a), Some(b)) = (fa.next(), fb.next()) {
            result.push((a, b));
        }
        result.into_iter().wrap()
    }
}

fn use_applicative<'a, A: 'a, F: Applicative>(
    a: A,
    b: A,
) -> F::Member<'a, (A, A)> {
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
