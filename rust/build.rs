fn main() {
    println!("cargo:rerun-if-changed=../asm/*.S");

    cc::Build::new().file("../asm/add.S").compile("add-lib");

    cc::Build::new()
        .file("../asm/minus.S")
        .compile("subtract-lib");
}
