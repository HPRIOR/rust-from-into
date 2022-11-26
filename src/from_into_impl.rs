#![allow(unused)]

// for any type define From, which returns itself
// eg. define the conversion from some type T to the type which is implementing this trait
trait TFrom<T>: Sized {
    fn from(arg: T) -> Self;
}

// for any type define Into, which returns a value of the generic type
trait TInto<T>: Sized {
    fn into(self) -> T;
}

/* For any type T, implement Into<U>, where U has implemented From<T>
 * This mechanism allows a function to accept a From implemented type as an argument which accepts
 * into. Calling into on that type simply delegates the from call to the types implementation of
 * from. This effectively defines Into for any type that's implemented From, and uses the method
 * from From to do the conversion.
 */
impl<T, U> TInto<U> for T
where
    U: TFrom<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}

impl TFrom<String> for i32 {
    fn from(string: String) -> Self {
        string.parse().unwrap()
    }
}

fn overloaded(from_type: impl TInto<i32>) -> i32 {
    from_type.into()
}
