use {{crate_name}}::part1::process;

#[tracing::instrument]
fn main() -> Result<(), ()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input1.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}
