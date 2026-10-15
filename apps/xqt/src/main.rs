use std::{
    env,
    io::{self, Write},
    path::Path,
    process::ExitCode,
};
use xqt_knowledge::{index::build_index_plan, load_corpus};

fn run(args: &[String]) -> Result<(), String> {
    let usage = "usage: xqt validate <corpus-dir> | xqt index-plan <corpus-dir> [--public]";
    let valid = (args.len() == 2 && matches!(args[0].as_str(), "validate" | "index-plan"))
        || (args.len() == 3 && args[0] == "index-plan" && args[2] == "--public");
    if !valid {
        return Err(usage.into());
    }
    let corpus = load_corpus(Path::new(&args[1])).map_err(|errors| {
        serde_json::to_string_pretty(&errors)
            .unwrap_or_else(|_| "could not serialize diagnostics".into())
    })?;
    let stdout = io::stdout();
    let mut output = stdout.lock();
    let value = if args[0] == "validate" {
        serde_json::json!({ "valid": true, "records": corpus.documents.len(),
            "checks": "schema_references_snapshots_only", "legal_replay": "not_run",
            "engine": "not_run", "lean": "not_run" })
    } else {
        serde_json::to_value(build_index_plan(&corpus, args.len() == 3))
            .map_err(|e| e.to_string())?
    };
    serde_json::to_writer_pretty(&mut output, &value).map_err(|e| e.to_string())?;
    writeln!(output).map_err(|e| e.to_string())
}

fn main() -> ExitCode {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}
