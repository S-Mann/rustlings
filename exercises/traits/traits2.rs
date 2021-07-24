trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        let mut new_result: Vec<String> = vec![];
        for val in self.into_iter() {
            new_result.push(val)
        }
        new_result.push(String::from("Bar"));
        new_result
    }
}
/*
// Better solution
impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        let mut new_result = self;
        new_result.push(String::from("Bar"));
        new_result
    }
}
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
