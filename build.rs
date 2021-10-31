fn main() {
println!("cargo:rustc-link-arg=-lreadline");
println!("cargo:rustc-link-arg=-lncurses");
println!("cargo:rustc-link-arg=-lm");
println!("cargo:rustc-link-arg=-ldl");
println!("cargo:rustc-link-arg=-lz");
println!("cargo:rustc-link-arg=-lpthread");
}
