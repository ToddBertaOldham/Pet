//**************************************************************************************************
// gcf.rs                                                                                          *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

pub trait Gcf<Rhs = Self> {
    type Output;

    fn gcf(self, rhs: Rhs) -> Self::Output;
}

//TODO Should this just be a module function? What signed integers and other cases?

macro_rules! implement_gcf {
    ($integer:ty) => {
        impl Gcf for $integer {
            type Output = $integer;

            fn gcf(self, rhs: $integer) -> Self::Output {
                let mut a = self;
                let mut b = rhs;

                while b != 0 {
                    let r = a % b;
                    a = b;
                    b = r;
                }

                return a;
            }
        }
    };
}

implement_gcf!(u8);
implement_gcf!(u16);
implement_gcf!(u32);
implement_gcf!(u64);
implement_gcf!(u128);
