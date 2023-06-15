use fdk::{Function, FunctionError, RuntimeContext};
use tokio;

#[tokio::main]
async fn main() -> Result<(), FunctionError> {

    let invocation_result = Function::run(|_: &mut RuntimeContext, i: String| {
        Ok(format!(
            "Hello {}!",
            if i.is_empty() {
                "world"
            } else {
                i.trim_end_matches("\n")
            }
        ))
    }).await;

    if let Err(e) =  invocation_result {
        eprintln!("function invocation error {}", e);
    }

    Ok(())
}
