enum Either<A, B> {
    A(A),
    B(B),
}

impl<T, A: Iterator<Item = T>, B: Iterator<Item = T>> Iterator for Either<A, B> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            Either::A(x) => x.next(),
            Either::B(x) => x.next(),
        }
    }
}

pub fn via_impl(only_evens: bool) -> impl Iterator<Item = u32> {
    if only_evens {
        Either::A((1..10000).filter(|x| x % 2 == 0))
    } else {
        Either::B(1..10000)
    }
}

pub fn via_dyn(only_evens: bool) -> Box<dyn Iterator<Item = u32>> {
    if only_evens {
        Box::new((1..10000).filter(|x| x % 2 == 0))
    } else {
        Box::new(1..10000)
    }
}
