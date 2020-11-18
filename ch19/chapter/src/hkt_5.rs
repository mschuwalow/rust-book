use std::marker::PhantomData;

trait Family<'a> {
    type Member<T: 'a>: Mirror<'a, T, Family = Self>;
}
trait Mirror<'a, T> {
    type Family: Family<'a>;
    fn as_member(self) -> <Self::Family as Family<'a>>::Member<T>;
}

trait Functor<'a>: Family<'a> {
    fn fmap<A: 'a, B: 'a, F: FnMut(A) -> B + 'a>(
        fa: <Self as Family<'a>>::Member<A>,
        f: F,
    ) -> <Self as Family<'a>>::Member<B>;
}

trait FunctorSyntax<'a, A: 'a, Fam: Functor<'a>>:
    Mirror<'a, A, Family = Fam> + Sized
{
    fn fmap<B, F: FnMut(A) -> B + 'a>(
        self,
        f: F,
    ) -> <Fam as Family<'a>>::Member<B> {
        <Fam as Functor>::fmap(self.as_member(), f)
    }
}

impl<'a, A: 'a, F: Functor<'a>, T: Mirror<'a, A, Family = F>>
    FunctorSyntax<'a, A, F> for T
{
}

trait Applicative<'a>: Functor<'a> {
    fn pure<A: 'a>(a: A) -> <Self as Family<'a>>::Member<A>;
    fn zip<A: 'a, B: 'a>(
        fa: <Self as Family<'a>>::Member<A>,
        fb: <Self as Family<'a>>::Member<B>,
    ) -> <Self as Family<'a>>::Member<(A, B)>;
}

fn pure<'a, F: Applicative<'a>, A: 'a>(a: A) -> F::Member<A> {
    <F as Applicative>::pure(a)
}

trait ApplicativeSyntax<'a, A: 'a, Fam: Applicative<'a>>:
    Mirror<'a, A, Family = Fam> + Sized
{
    fn zip<B: 'a>(
        self,
        fb: <Fam as Family<'a>>::Member<B>,
    ) -> <Fam as Family<'a>>::Member<(A, B)> {
        <Fam as Applicative>::zip(self.as_member(), fb)
    }
}

impl<'a, A: 'a, F: Applicative<'a>, T: Mirror<'a, A, Family = F>>
    ApplicativeSyntax<'a, A, F> for T
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
// impl<E: 'static> Functor for ResultFamily<E> {
//     fn fmap<'a, A: 'a, B: 'a, F: FnMut(A) -> B + 'a>(
//         fa: Result<A, E>,
//         f: F,
//     ) -> Result<B, E> {
//         fa.map(f)
//     }
// }
// impl<E: 'static> Applicative for ResultFamily<E> {
//     fn pure<'a, A: 'a>(a: A) -> Result<A, E> {
//         Ok(a)
//     }

//     fn zip<'a, A: 'a, B: 'a>(
//         fa: Result<A, E>,
//         fb: Result<B, E>,
//     ) -> Result<(A, B), E> {
//         fa.and_then(|a| fb.map(|b| (a, b)))
//     }
// }

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

fn use_applicative<'a, A: 'a, F: Applicative<'a>>(
    a: A,
    b: A,
) -> F::Member<(A, A)> {
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
