use ranty::Ranty;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a Ranty context and load the standard library
    let mut ranty = Ranty::with_random_seed();

    // Compile a simple program
    let program = ranty.compile_quiet(
        r#"
  [$greet:name] {
    {Hello|Hi|Hey} <name>!
  }
  [greet:world]
  "#,
    )?;

    // Run the program and fetch the result string
    let output = ranty.run(&program)?;
    println!("{}", output);
    Ok(())
}
