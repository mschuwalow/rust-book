use std::marker::PhantomData;

trait Family {
    type Member<T>: Mirror<T, Family = Self>;
}
trait Mirror<T> {
    type Family: Family;
    fn as_member(self) -> <Self::Family as Family>::Member<T>;
}

trait Functor: Family {
    fn fmap<A: 'static, B, F: FnMut(A) -> B + 'static>(
        fa: <Self as Family>::Member<A>,
        f: F,
    ) -> <Self as Family>::Member<B>;
}

trait FunctorSyntax<A: 'static, Fam: Functor>:
    Mirror<A, Family = Fam> + Sized
{
    fn fmap<B, F: FnMut(A) -> B + 'static>(
        self,
        f: F,
    ) -> <Fam as Family>::Member<B> {
        <Fam as Functor>::fmap(self.as_member(), f)
    }
}

impl<A: 'static, F: Functor, T: Mirror<A, Family = F>> FunctorSyntax<A, F>
    for T
{
}

trait Applicative: Functor {
    fn pure<A: 'static>(a: A) -> <Self as Family>::Member<A>;
    fn zip<A: 'static, B: 'static>(
        fa: <Self as Family>::Member<A>,
        fb: <Self as Family>::Member<B>,
    ) -> <Self as Family>::Member<(A, B)>;
}

fn pure<F: Applicative, A: 'static>(a: A) -> F::Member<A> {
    <F as Applicative>::pure(a)
}

trait ApplicativeSyntax<A: 'static, Fam: Applicative>:
    Mirror<A, Family = Fam> + Sized
{
    fn zip<B: 'static>(
        self,
        fb: <Fam as Family>::Member<B>,
    ) -> <Fam as Family>::Member<(A, B)> {
        <Fam as Applicative>::zip(self.as_member(), fb)
    }
}

impl<A: 'static, F: Applicative, T: Mirror<A, Family = F>>
    ApplicativeSyntax<A, F> for T
{
}

// usage

struct OptionFamily;
impl Family for OptionFamily {
    type Member<T> = Option<T>;
}
impl<A> Mirror<A> for Option<A> {
    type Family = OptionFamily;

    fn as_member(self) -> Option<A> {
        self
    }
}
impl Functor for OptionFamily {
    fn fmap<A, B, F: FnMut(A) -> B>(fa: Option<A>, f: F) -> Option<B> {
        fa.map(f)
    }
}
impl Applicative for OptionFamily {
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

// Result
struct ResultFamily<E> {
    phantom: PhantomData<E>,
}
impl<E> Family for ResultFamily<E> {
    type Member<T> = Result<T, E>;
}
impl<E, A> Mirror<A> for Result<A, E> {
    type Family = ResultFamily<E>;

    fn as_member(self) -> <Self::Family as Family>::Member<A> {
        self
    }
}
impl<E> Functor for ResultFamily<E> {
    fn fmap<A, B, F: FnMut(A) -> B>(fa: Result<A, E>, f: F) -> Result<B, E> {
        fa.map(f)
    }
}
impl<E> Applicative for ResultFamily<E> {
    fn pure<A>(a: A) -> Result<A, E> {
        Ok(a)
    }

    fn zip<A, B>(fa: Result<A, E>, fb: Result<B, E>) -> Result<(A, B), E> {
        fa.and_then(|a| fb.map(|b| (a, b)))
    }
}

struct IteratorWrap<T: Sized>(Box<dyn Iterator<Item = T>>);
trait IteratorSyntax<T>: Iterator<Item = T> + Sized + 'static {
    fn wrap(self) -> IteratorWrap<T> {
        IteratorWrap(Box::new(self))
    }
}
impl<T, I: Iterator<Item = T> + 'static> IteratorSyntax<T> for I {}
impl<T> Iterator for IteratorWrap<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

// iterator
struct IteratorWrapFamily;
impl Family for IteratorWrapFamily {
    type Member<T> = IteratorWrap<T>;
}
impl<T> Mirror<T> for IteratorWrap<T> {
    type Family = IteratorWrapFamily;

    fn as_member(self) -> <Self::Family as Family>::Member<T> {
        self
    }
}
impl Functor for IteratorWrapFamily {
    fn fmap<A: 'static, B, F: FnMut(A) -> B + 'static>(
        fa: IteratorWrap<A>,
        f: F,
    ) -> IteratorWrap<B> {
        IteratorWrap(Box::new(fa.0.into_iter().map(f).into_iter()))
    }
}
impl Applicative for IteratorWrapFamily {
    fn pure<A: 'static>(a: A) -> <Self as Family>::Member<A> {
        vec![a].into_iter().wrap()
    }

    fn zip<A: 'static, B: 'static>(
        fa: IteratorWrap<A>,
        fb: IteratorWrap<B>,
    ) -> <Self as Family>::Member<(A, B)> {
        let mut result = Vec::new();
        let mut fa = fa;
        let mut fb = fb;
        while let (Some(a), Some(b)) = (fa.next(), fb.next()) {
            result.push((a, b));
        }
        result.into_iter().wrap()
    }
}

fn use_applicative<F: Applicative>(i: i32) -> F::Member<(i32, i32)> {
    let fa = pure::<F, _>(i);
    let fb = pure::<F, _>(i);
    fa.fmap(|x| x + 1).zip(fb)
}

fn foo() -> Option<(i32, i32)> {
    let a = Some(3);
    let b = a.fmap(|x| (x + 1, x));
    b
}

fn use_iterator() -> IteratorWrap<i32> {
    let v = vec![1];
    IteratorWrap(Box::new(v.into_iter())).fmap(|x| x + 1)
}

struct Foo<'a, A> {
    a: A,
    b: &'a i32,
}

// lifitme don't work...

// fn borrowed<'a, A, B>(
//     a: Foo<'a, A>,
//     b: Foo<'a, B>,
// ) -> Option<(Foo<'a, A>, Foo<'a, B>)> {
//     pure::<OptionFamily, _>((a, b))
// }
