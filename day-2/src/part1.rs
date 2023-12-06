#[tracing::instrument]
pub fn process(_input: &str) -> Result<String, ()> {
    let result = _input.lines().filter_map(|line| {
        let id = line
            .replace(':', "")
            .split_whitespace()
            .filter_map(|word| word.parse::<u32>().ok())
            .next();

        let processed_line = line
            .split(':')
            .last()
            .unwrap()
            .replace(';', ",")
            .replace("red", "2")
            .replace("blue", "0")
            .replace("green", "1");

        let iterator = processed_line.split(',');
        if iterator
            .map(|a| {
                a.split_whitespace()
                    .filter_map(|part| part.parse::<u32>().ok())
                    .sum::<u32>()
            })
            .any(|x| x > 14)
        {
            None
        } else {
            Some(id.unwrap())
        }
    });

    let answer = result.sum::<u32>();
    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), ()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
