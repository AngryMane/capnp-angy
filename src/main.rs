use std::path::PathBuf;
use std::env;

use capnp::serialize;
use capnp::schema_capnp::node;
use capnp::schema_capnp::node::struct_;
use capnp::schema_capnp::node::interface;
use capnp::schema_capnp::node::const_;
use capnp::schema_capnp::node::enum_;
use capnp::schema_capnp::node::annotation;
use capnpc::codegen::GeneratorContext;

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

fn main() {
    let args_: Vec<String> = env::args().collect();
    if args_.len() == 1 {
        println!("Please pass target schema pathes as arguments");
        return;
    }
    let args = &args_[1..];
    let no_standard_import = false;
    let import_paths: Vec<PathBuf> = vec![];
    let src_prefixes: Vec<PathBuf> = vec![]; 
    let files: Vec<PathBuf> = args.into_iter().map(|x| PathBuf::from(x)).collect();
    let stdout = run_capnp(no_standard_import, import_paths, src_prefixes, files);
    let message = serialize::read_message(
        ReadWrapper { inner: stdout },
        capnp::message::ReaderOptions::new(),
    ).unwrap();
    let ctx: GeneratorContext = GeneratorContext::new(&message).unwrap();
    for requested_file in ctx.request.get_requested_files().unwrap() {
        load(&ctx, requested_file.get_id());
    }
}

fn load(ctx: &GeneratorContext, node_id: u64) {
    let node_ = ctx.node_map[&node_id];
    match node_.which().unwrap() {
        node::File(_) => {}
        node::Struct(struct_) => {
            handle_struct(ctx, struct_);
        }
        node::Interface(interface_) => {
            handle_interface(ctx, interface_);
        }
        node::Const(const_) => {
            handle_const(ctx, const_);
        }
        node::Enum(enum_) => {
            handle_enum(ctx, enum_);
        }
        node::Annotation(annotation_) => {
            handle_annotation(ctx, annotation_);
        }
    }
    for nested_node in node_.get_nested_nodes().unwrap(){
        load(ctx, nested_node.get_id())
    }
}

fn handle_struct(ctx: &GeneratorContext, struct_: struct_::Reader) {
    println!("struct detected");
    // 
}

fn handle_interface(ctx: &GeneratorContext, interface_: interface::Reader) {
    println!("interface detected");
    // 
}

fn handle_const(ctx: &GeneratorContext, const_: const_::Reader) {
    println!("const detected");
    // 
}

fn handle_enum(ctx: &GeneratorContext, enum_: enum_::Reader) {
    println!("enum detected");
    // 
}

fn handle_annotation(ctx: &GeneratorContext, annotation_: annotation::Reader) {
    println!("annotation detected");
    // 
}

fn run_capnp(no_standard_import: bool, import_paths: Vec<PathBuf>, src_prefixes: Vec<PathBuf>, files: Vec<PathBuf>) -> std::process::ChildStdout {
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

    for file in files {
        command.arg(file);
    }

    command.stdout(::std::process::Stdio::piped());
    command.stderr(::std::process::Stdio::inherit());

    let mut p = command.spawn().unwrap();
    p.stdout.take().unwrap()
}