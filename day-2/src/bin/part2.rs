use day_2::part2::process;

#[tracing::instrument]
fn main() -> Result<(), ()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input2.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}
