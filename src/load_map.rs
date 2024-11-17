use crate::error::{Error, Result};
use std::fs;

pub struct Map {
    pub content: Vec<Vec<u8>>,
}

impl Map {
    pub fn load_map(path: &str) -> Result<Self> {
        let content = fs::read(path).map_err(|_| Error::OpenMap)?;
        let map = parse_map(&content);
        check_map_format(&map)?;
        Ok(Self { content: map })
    }
}

fn parse_map(content: &[u8]) -> Vec<Vec<u8>> {
    let mut map: Vec<Vec<u8>> = vec![vec![]];

    for &value in content.iter() {
        match value {
            b',' => continue,
            b'\n' => {
                map.push(vec![]);
            }
            _ => {
                map.last_mut().unwrap().push(value);
            }
        }
    }

    if map.last().unwrap().is_empty() {
        map.pop();
    }

    map
}

fn check_map_format(map: &[Vec<u8>]) -> Result<()> {
    if map.is_empty() {
        return Err(Error::EmptyMap);
    }

    let row_length = map[0].len();
    if map.iter().any(|row| row.len() != row_length) {
        return Err(Error::VariousRowLengthMap);
    }

    Ok(())
}

#[test]
fn example_load_map() {
    let map = Map::load_map("maps/l1.csv").unwrap();
    println!("{:?}", map.content);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_parser() {
        let input = [
            1, 44, 2, 44, 3, 10, 4, 44, 5, 44, 6, 10, 7, 44, 8, 44, 9, 10,
        ];
        let map = parse_map(&input);
        let expected = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(map, expected)
    }

    #[test]
    fn test_empty_map_error() {
        let input = [];
        let map = parse_map(&input);
        let map_check_result = check_map_format(&map);
        assert_eq!(map_check_result.unwrap_err(), Error::EmptyMap);
    }

    #[test]
    fn test_various_row_length_error() {
        let input = [1, 44, 2, 44, 3, 10, 4, 44, 10];
        let map = parse_map(&input);
        let map_check_result = check_map_format(&map);
        assert_eq!(map_check_result.unwrap_err(), Error::VariousRowLengthMap);
    }
}
