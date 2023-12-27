pub fn add(left: usize, right: usize) -> usize {
    left + right
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