pub trait Prepend<T> {
    type Output;

    fn prepend(self, other: T) -> Self::Output;
}

pub trait Append<T> {
    type Output;

    fn append(self, other: T) -> Self::Output;
}

pub trait Flatten {
    type Output;

    fn flatten(self) -> Self::Output;
}

impl<T> Prepend<T> for () {
    type Output = (T, Self);

    fn prepend(self, other: T) -> Self::Output {
        (other, self)
    }
}

impl<T, A, B> Prepend<T> for (A, B) {
    type Output = (T, Self);

    fn prepend(self, other: T) -> Self::Output {
        (other, self)
    }
}

impl<T> Append<T> for () {
    type Output = (T, Self);

    fn append(self, other: T) -> Self::Output {
        (other, self)
    }
}

impl<T, A, B: Append<T>> Append<T> for (A, B) {
    type Output = (A, B::Output);

    fn append(self, other: T) -> Self::Output {
        (self.0, self.1.append(other))
    }
}

impl Flatten for () {
    type Output = ();

    fn flatten(self) -> Self::Output {
        self
    }
}

macro_rules! cons {
    () => {
        ()
    };

    ($head:ident) => {
        ($head, ())
    };

    ($head:ident, $($tail:ident),+) => {
        ($head, cons!($($tail),+))
    };
}

macro_rules! impl_flatten {
    ($head:ident) => {
        impl_flatten!(@impl $head);
    };

    ($head:ident, $($tail:ident),+) => {
        impl_flatten!($($tail),+);
        impl_flatten!(@impl $head, $($tail),+);
    };

    (@impl $($ty:ident),+) => {
        impl<$($ty),+> Flatten for cons!($($ty),+) {
            type Output = ($($ty,)+);

            #[allow(non_snake_case)]
            fn flatten(self) -> Self::Output {
                let cons!($($ty),+) = self;
                ($($ty,)+)
            }
        }
    };
}

impl_flatten!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);