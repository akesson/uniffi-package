uniffi::include_scaffolding!("Mobile");

pub fn rust_greeting(to: String) -> String {
    return format!("Hello, {}!", to);
}
