

pub mod math;

pub mod pig_latin;


#[cfg(test)]
mod tests {

    use super::math;
    use super::pig_latin;

    #[test]
    fn test_man(){
        let nums = vec![1,2,3,3,4];
        assert_eq!(math::mean(&nums), 2.6);
        assert_eq!(math::median(&nums), 3.0);
        assert_eq!(math::mode(&nums), 3);

        let nums = vec![1,2,3,4,5,6,6,7];
        assert_eq!(math::mean(&nums), 4.25);
        assert_eq!(math::median(&nums), 4.5);
        assert_eq!(math::mode(&nums), 6);

        let mut word1 = String::from("first");
        let mut word2 = String::from("irst-fay");
        assert_eq!(pig_latin::convert(word1), word2);

        word1 = String::from("apple");
        word2 = String::from("apple-hay");
        assert_eq!(pig_latin::convert(word1), word2);
    }
}
