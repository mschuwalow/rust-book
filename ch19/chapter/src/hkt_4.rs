trait Family {
    type Member<T>: Mirror<T, Family = Self>;
}
trait Mirror<T> {
    type Family: Family;
    fn as_member(self) -> <Self::Family as Family>::Member<T>;
}

trait Functor: Family {
    fn fmap<A, B, F: FnMut(A) -> B>(
        fa: <Self as Family>::Member<A>,
        f: F,
    ) -> <Self as Family>::Member<B>;
}

trait FunctorSyntax<A, Fam: Functor>: Mirror<A, Family = Fam> + Sized {
    fn fmap<B, F: FnMut(A) -> B>(self, f: F) -> <Fam as Family>::Member<B> {
        <Fam as Functor>::fmap(self.as_member(), f)
    }
}

impl<A, F: Functor, T: Mirror<A, Family = F>> FunctorSyntax<A, F> for T {}

trait Applicative: Functor {
    fn pure<A>(a: A) -> <Self as Family>::Member<A>;
    fn zip<A, B>(
        fa: <Self as Family>::Member<A>,
        fb: <Self as Family>::Member<B>,
    ) -> <Self as Family>::Member<(A, B)>;
}

fn pure<F: Applicative, A>(a: A) -> F::Member<A> {
    <F as Applicative>::pure(a)
}

trait ApplicativeSyntax<A, Fam: Applicative>:
    Mirror<A, Family = Fam> + Sized
{
    fn zip<B>(
        self,
        fb: <Fam as Family>::Member<B>,
    ) -> <Fam as Family>::Member<(A, B)> {
        <Fam as Applicative>::zip(self.as_member(), fb)
    }
}

impl<A, F: Applicative, T: Mirror<A, Family = F>> ApplicativeSyntax<A, F>
    for T
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
