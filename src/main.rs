use color_eyre::Report;


#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;
    println!("Hello from a (so far completely unnecessary) async runtime");
    Ok(())
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;
    Ok(())
}
