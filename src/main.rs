mod compiler;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);

    let input_file = std::fs::read_to_string(args.next().unwrap())?;
    // let _output_file = args.next().unwrap();

    match compiler::compile(&input_file) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(err.exit_code());
        }
    }

    Ok(())
}
