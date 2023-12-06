#[tracing::instrument]
pub fn process(_input: &str) -> Result<String, ()> {
    let result: u32 = _input
        .lines()
        .map(|line| {
            {
                let mut iterator = line.chars().filter_map(|char| char.to_digit(10));
                let first = iterator.next().unwrap();
                match iterator.last() {
                    Some(last) => format!("{first}{last}"),
                    None => format!("{first}{first}"),
                }
                .parse::<u32>()
            }
            .unwrap()
        })
        .sum();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), ()> {
        let input = "1abc2
                     pqr3stu8vwx
                     a1b2c3d4e5f
                     treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
