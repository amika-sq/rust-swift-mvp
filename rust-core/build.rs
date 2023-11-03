use uniffi::generate_scaffolding;
use uniffi_bindgen::{bindings::TargetLanguage::Swift, generate_bindings};

fn main() {
    let udl_file = "./src/rust-core.udl";
    let out_dir = "./bindings";
    generate_scaffolding(udl_file).unwrap();
    generate_bindings(
        udl_file.into(),
        None,
        vec![Swift],
        Some(out_dir.into()),
        None,
        None,
        true,
    )
    .unwrap();
}
