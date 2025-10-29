use clap::{Parser, Subcommand, ValueEnum};
use std::fs;
use std::path::PathBuf;

use crate::ir::tac::{emit_assembly, TacGenerator};
use crate::parser::parse;

/// Runs the command line interface.
pub fn run() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile {
            input,
            emit,
            output,
        } => compile(input, emit, output),
    }
}

fn compile(input: PathBuf, emit: EmitKind, output: Option<PathBuf>) -> Result<(), String> {
    if !has_rpy_extension(&input) {
        return Err(format!(
            "expected an input file with '.rpy' extension, got {:?}",
            input
        ));
    }

    let source = fs::read_to_string(&input)
        .map_err(|err| format!("failed to read '{}': {err}", input.display()))?;

    let (remaining, statements) =
        parse(&source).map_err(|err| format!("failed to parse '{}': {err:?}", input.display()))?;

    if !remaining.trim().is_empty() {
        return Err(format!(
            "input '{}' was not fully consumed. Remaining: {:?}",
            input.display(),
            remaining
        ));
    }

    let mut generator = TacGenerator::new();
    let program = generator.generate(&statements);

    let artifact = match emit {
        EmitKind::Tac => program.to_pretty_string(),
        EmitKind::Assembly => emit_assembly(&program)?,
    };

    if let Some(path) = output {
        fs::write(&path, artifact)
            .map_err(|err| format!("failed to write '{}': {err}", path.display()))?;
    } else {
        println!("{}", artifact);
    }

    Ok(())
}

fn has_rpy_extension(path: &PathBuf) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("rpy"))
        .unwrap_or(false)
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compiles an RPython source file into TAC or native assembly.
    Compile {
        /// Path to the `.rpy` source file.
        input: PathBuf,

        /// Selects the output artefact.
        #[arg(long, value_enum, default_value_t = EmitKind::Tac)]
        emit: EmitKind,

        /// Optional output file. When omitted the artefact is printed to stdout.
        #[arg(long, short)]
        output: Option<PathBuf>,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum EmitKind {
    Tac,
    Assembly,
}
