fn main() {
    println!("cargo::rerun-if-changed=vendor/microui/src/microui.c,vendor/microui/src/microui.h");
    cc::Build::new()
        .file("vendor/microui/src/microui.c")
        .compile("microui");
}
