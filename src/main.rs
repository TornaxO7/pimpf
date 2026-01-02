mod compiler;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);

    let input_file = std::fs::read_to_string(args.next().unwrap())?;
    let _output_file = args.next().unwrap();

    let _bin = compiler::compile(&input_file);

    Ok(())
}
