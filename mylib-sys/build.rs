fn main() {
    let package_dir = std::env::current_dir().unwrap();
    let workspace_dir = package_dir.parent().unwrap();
    let c_out_dir = workspace_dir
        .join("c-mylib")
        .join("out")
        .to_str()
        .unwrap()
        .to_owned();
    println!("cargo:rustc-link-search=native={}", c_out_dir);
    println!("cargo:rustc-link-lib=static={}", "mylib");
}
