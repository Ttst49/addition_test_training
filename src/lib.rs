pub fn add(left: usize, right: usize) -> usize {
    left + right
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

pub fn add_two(a: i64)->i64{
    a + 2
}

pub fn reception(name: &str)->String{
    format!("Bonjour")
}

pub struct Supposition {
    value: i32,
}

impl Supposition {
    pub fn new(value: i32) -> Supposition {
        if value < 1 || value > 100 {
            panic!("La supposition doit se trouver entre 1 et 100, et nous avons {}.", value);
        }
        Supposition { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn check_add_two(){
        assert_eq!(4,add_two(2))
    }

    //#[test]
    fn check_reception(){
        let result = reception("Caroline");
        assert!(result.contains("Caroline"),
                "Le message ne contient pas le nom, il vaut '{}'",
                result
        );
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

    //add should panic as a condition for true
    #[test]
    #[should_panic]
    fn plus_grand_que_100() {
        Supposition::new(200);
    }

}
