use protobuf_codegen::Codegen;

fn main() {
    Codegen::new()
        .pure()
        .includes(["src/protos"])
        .input("src/protos/control.proto")
        .cargo_out_dir("protos")
        .run_from_script();
}
