use super::ParseError;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct KeywordList<T: FromStr + Into<String>>(Vec<T>);

impl<T: FromStr + Into<String>> KeywordList<T> {
    pub fn new() -> KeywordList<T> {
        KeywordList(Vec::<T>::new())
    }

    pub fn push(&mut self, element: T) {
        self.0.push(element)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T: FromStr + Into<String>> FromStr for KeywordList<T> {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let mut elements = Vec::<T>::new();
        if raw.len() > 0 {
            for element in raw.split(',') {
                elements.push(
                    element
                        .parse()
                        .map_err(|_| ParseError::new("KeywordList"))?,
                );
            }
        }
        Ok(Self(elements))
    }
}

impl<T: FromStr + Into<String>> From<KeywordList<T>> for String {
    fn from(keyword_list: KeywordList<T>) -> String {
        keyword_list
            .0
            .into_iter()
            .map(|a| a.into())
            .collect::<Vec<String>>()
            .join(",")
    }
}

#[cfg(test)]
mod test_keyword_list {
    use super::*;

    #[derive(PartialEq, Debug)]
    struct TestStruct(char);

    impl FromStr for TestStruct {
        type Err = ParseError;

        fn from_str(raw: &str) -> Result<Self, Self::Err> {
            Ok(TestStruct(
                raw.chars()
                    .nth(0)
                    .ok_or_else(|| ParseError::new("TestStruct"))?,
            ))
        }
    }

    impl From<TestStruct> for String {
        fn from(test_struct: TestStruct) -> String {
            let mut result = String::new();
            result.push(test_struct.0);
            result
        }
    }

    #[test]
    fn from_string() {
        assert_eq!(
            Ok(KeywordList(vec![
                TestStruct('a'),
                TestStruct('d'),
                TestStruct('e')
            ])),
            "abc,d,e".parse::<KeywordList<TestStruct>>()
        );
        assert_eq!(
            Ok(KeywordList(Vec::<TestStruct>::new())),
            "".parse::<KeywordList<TestStruct>>()
        );
    }

    #[test]
    fn into_string() {
        assert_eq!(
            "a,b,c".to_string(),
            String::from(KeywordList(vec![
                TestStruct('a'),
                TestStruct('b'),
                TestStruct('c')
            ])),
        );
        assert_eq!(
            "".to_string(),
            String::from(KeywordList(Vec::<TestStruct>::new())),
        );
    }

    #[test]
    fn invalid() {
        assert!("a,,c".parse::<KeywordList<TestStruct>>().is_err());
    }
}
