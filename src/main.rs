mod is_broken;
mod util;
use crate::is_broken::*;

use std::path::PathBuf;
use capnp::serialize;
use capnpc::codegen::GeneratorContext;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// the path to the base capn'proto schema file
    base_file_path: String,
    /// the path to the changed capn'proto schema file
    changed_file_path: String,
    /// whether to output to file. The default value is None, and does not output as a file.
    #[arg(short, long, default_value=None)]
    output_file_path: Option<String>,
    /// whether to import the standard path("/usr/local/include" and "/usr/include") or not
    #[arg(short, long, default_value_t = false)]
    no_standard_import: bool,
    /// paths to the capn' proto schema files you want to import from the target schema
    #[arg(short, long, default_values_t = Vec::<String>::new(), num_args(0..))]
    import_paths: Vec<String>,
    /// prefixes of the schema file
    #[arg(short, long, default_values_t = Vec::<String>::new(), num_args(0..))]
    src_prefixes: Vec<String>,
}

struct ReadWrapper<R>
where
    R: std::io::Read,
{
    inner: R,
}

impl<R> capnp::io::Read for ReadWrapper<R>
where
    R: std::io::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> capnp::Result<usize> {
        loop {
            match std::io::Read::read(&mut self.inner, buf) {
                Ok(n) => return Ok(n),
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => return Err(capnp::Error{description: format!("{e}"), kind: capnp::ErrorKind::Failed}),
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = Args::parse();
    let base_file = PathBuf::from(args.base_file_path);
    let stdout = run_capnp(
        args.no_standard_import, 
        args.import_paths.iter().map(PathBuf::from).collect(), 
        args.src_prefixes.iter().map(PathBuf::from).collect(), 
        base_file);
    let message = serialize::read_message(
        ReadWrapper { inner: stdout },
        capnp::message::ReaderOptions::new(),
    )?;
    let base_ctx: GeneratorContext = GeneratorContext::new(&message)?;

    let chnaged_file = PathBuf::from(args.changed_file_path);
    let stdout = run_capnp(
        args.no_standard_import, 
        args.import_paths.iter().map(PathBuf::from).collect(), 
        args.src_prefixes.iter().map(PathBuf::from).collect(), 
        chnaged_file);
    let message = serialize::read_message(
        ReadWrapper { inner: stdout },
        capnp::message::ReaderOptions::new(),
    )?;
    let changed_ctx: GeneratorContext = GeneratorContext::new(&message)?;

    // loop based on the nodes in base schema 
    for requested_file in base_ctx.request.get_requested_files()? {
        let _ = is_broken(&base_ctx, &changed_ctx, requested_file.get_id());
    }

    Ok(())
}

fn run_capnp(no_standard_import: bool, import_paths: Vec<PathBuf>, src_prefixes: Vec<PathBuf>, target_file: PathBuf) -> std::process::ChildStdout {
    let mut command = ::std::process::Command::new("capnp");
    command.env_remove("PWD");
    command.arg("compile").arg("-o").arg("-");
    if no_standard_import {
        command.arg("--no-standard-import");
    }

    for import_path in import_paths {
        command.arg(&format!("--import-path={}", import_path.display()));
    }

    for src_prefix in src_prefixes {
        command.arg(&format!("--src-prefix={}", src_prefix.display()));
    }

    command.arg(target_file);

    command.stdout(::std::process::Stdio::piped());
    command.stderr(::std::process::Stdio::inherit());

    let mut p = command.spawn().unwrap();
    p.stdout.take().unwrap()
}