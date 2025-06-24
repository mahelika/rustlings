// This powerful wrapper provides the ability to store any value.
struct Wrapper<T> {
    value: T,
}

// Adapt the struct's implementation to be generic over the wrapped value.
impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // Experiment here
    let int_wrapper = Wrapper::new(10);
    let str_wrapper = Wrapper::new("Hello");

    println!("Int: {}", int_wrapper.value);
    println!("Str: {}", str_wrapper.value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}