mod compiler;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);

    let input_file = std::fs::read_to_string(args.next().unwrap())?;
    // let _output_file = args.next().unwrap();

    // std::thread::spawn({
    //     let input_file = input_file.clone();
    //     let time = std::time::Instant::now();

    //     move || {
    //         while time.elapsed().as_secs() < 1 {}

    //         eprintln!("{}", input_file);
    //         std::process::exit(1);
    //     }
    // });

    match compiler::compile(&input_file) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(err.exit_code());
        }
    }

    Ok(())
}
