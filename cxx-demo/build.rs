//! file: build.rs
//! author: Jacob Xie
//! date: 2023/12/21 17:00:41 Thursday
//! brief:

fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/blobstore.cc")
        .flag_if_supported("-std=c++14")
        .compile("cxx-demo")
}
