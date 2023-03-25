use std::io::Error;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run("127.0.0.1:8000")?.await
}
