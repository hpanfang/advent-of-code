use std::collections::HashMap;

#[tracing::instrument]
pub fn process(_input: &str) -> Result<String, ()> {
    let result: u32 = _input.lines().map(parse_function).sum();
    Ok(result.to_string())
}

#[tracing::instrument]
fn parse_function(line: &str) -> u32 {
    let number_map: HashMap<&str, char> = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]
    .iter()
    .cloned()
    .collect();

    let mut iterator = (0..line.len()).filter_map(|index| {
        let working_line = &line[index..];
        let result = number_map
            .iter()
            .find(|&(key, _)| working_line.starts_with(key))
            .map(|(_, &value)| value)
            .unwrap_or_else(|| working_line.chars().next().unwrap());
        result.to_digit(10)
    });

    let first = iterator.next().unwrap();

    match iterator.last() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), ()> {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
