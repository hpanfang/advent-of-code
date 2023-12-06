#[tracing::instrument]
pub fn process(_input: &str) -> Result<String, ()> {
    let result = _input.lines().map(|line| {
        let processed_line = line.split(':').last().unwrap().replace(';', ",");

        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        for entry in processed_line.split(',') {
            let mut parts = entry.split_whitespace();
            let number = parts.next().unwrap().parse::<u32>().unwrap();
            let color = parts.next().unwrap();
            match color {
                "red" => {
                    red = red.max(number);
                }
                "blue" => {
                    blue = blue.max(number);
                }
                "green" => {
                    green = green.max(number);
                }
                _ => panic!("unknown color"),
            }
        }
        red * blue * green
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
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}
