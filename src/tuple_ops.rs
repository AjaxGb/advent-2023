mod private {
    pub trait Sealed {}
}

pub trait TupleAppend<T>: Sized + private::Sealed {
    type Out;

    fn append(self, value: T) -> Self::Out;
}

macro_rules! impl_tuple_traits {
    ($(
        ($($var:ident),*)
    )*) => {
        $(
            impl<$($var),*> private::Sealed for ($($var,)*) {}

            impl<$($var,)* T> TupleAppend<T> for ($($var,)*) {
                type Out = ($($var,)* T,);

                fn append(self, value: T) -> Self::Out {
                    #[allow(non_snake_case)]
                    let ($($var,)*) = self;
                    ($($var,)* value,)
                }
            }
        )*
    };
}

impl_tuple_traits! {
    ()
    (A)
    (A, B)
    (A, B, C)
    (A, B, C, D)
    (A, B, C, D, E)
    (A, B, C, D, E, F)
    (A, B, C, D, E, F, G)
    (A, B, C, D, E, F, G, H)
}
