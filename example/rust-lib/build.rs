// use uniffi_swiftpkg_gen::*;

fn main() {
    uniffi::generate_scaffolding("./src/Mobile.udl").unwrap();

    // Generates Xcode Swift package
    // Builder::new().generate();
}
