use crate::error::Result;

pub fn run(args: &clap::ArgMatches) -> Result<()> {
    let run_span = tracing::debug_span!("cli run");
    let _enter = run_span.enter();
    tracing::debug!("entered cli run: {:?}", args);

    let (args

    Ok(())
}
