trait AppendBar {
    fn append_bar(self) -> Self;
}

// Implement the trait for Vec<String>
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push("Bar".to_string());
        self
    }
} 

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
