use color_eyre::Report;

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;

    println!("Hello, world!");

    Ok(())
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1");
    }

    color_eyre::install()?;

    Ok(())
}
