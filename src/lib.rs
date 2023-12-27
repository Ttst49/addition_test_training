pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn add_two(a: i64)->i64{
    a +2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }



    //#[test]
    fn doesnt_work(){
        panic!("ahhhhhhhhhhhhhhhhhhhhh")
    }

    #[test]
    fn bigger_can_contain_little(){
        let bigger = Rectangle{ width: 5, height: 2 };
        let little = Rectangle{ width: 3, height: 1 };

        assert!(bigger.can_contain(&little));
    }

    #[test]
    fn little_cant_contain_bigger(){
        let bigger = Rectangle{ width: 5, height: 2 };
        let little = Rectangle{ width: 3, height: 1 };

        assert!(!little.can_contain(&bigger))
    }

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_contain(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}