mod from_into_impl;

/// How to do function overloading with traits

// Because of the orphan rule we can wrap a value in order to implement a trait on the number
// We could also implement our own version of the From/Into trait to bypass this behaviour.
struct Number(i32);

// simple method to unwrape the value in the struct
impl Number {
    fn value(&self) -> i32 {
        self.0
    }
}

/*
 * We can define the From trait for a type. This defines how we should convert the type to the
 * given generic parameter. It can be read as 'how can be we get T from Number?' 
 * */
impl From<String> for Number {
    fn from(num: String) -> Self {
        Number(num.parse().unwrap())
    }
}

impl From<i32> for Number {
    fn from(num: i32) -> Self {
        Number(num)
    }
}

/*
 * The reciprocol trait for From is Into. If you have implemented From for a type you can pass it
 * into a function accepting an Into type. Calling into on the type will allow you to convert the
 * argument to the desired type.
 * */
fn overloaded_function(num: impl Into<Number>) -> i32 {
    num.into().value()
}


fn main() {
    // The overloading funciton will accept any type that implements From<T> for Number
    overloaded_function("2".to_string());
    overloaded_function(2);
}
