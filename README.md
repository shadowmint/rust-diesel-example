# Example

    doug:tmp doug$ git clone git@github.com:shadowmint/rust-diesel-example.git
    Cloning into 'rust-diesel-example'...
    remote: Counting objects: 11, done.
    remote: Compressing objects: 100% (11/11), done.
    remote: Total 11 (delta 0), reused 11 (delta 0), pack-reused 0
    Receiving objects: 100% (11/11), done.
    Checking connectivity... done.
    doug:tmp doug$ cd rust-diesel-example/
    doug:rust-diesel-example doug$ time cargo build
        Updating registry `https://github.com/rust-lang/crates.io-index`
       Compiling bitflags v0.3.3
       Compiling libc v0.2.2
       Compiling byteorder v0.3.13
       Compiling winapi-build v0.1.1
       Compiling unicode-xid v0.0.3
       Compiling rustc-serialize v0.3.16
       Compiling winapi v0.2.5
       Compiling kernel32-sys v0.2.1
       Compiling pq-sys v0.2.0
       Compiling log v0.3.4
       Compiling diesel v0.1.0
       Compiling term v0.2.14
       Compiling syntex_syntax v0.22.0
       Compiling quasi v0.3.8
       Compiling aster v0.8.0
       Compiling syntex v0.22.0
       Compiling quasi_codegen v0.3.9
       Compiling diesel_codegen v0.1.0
       Compiling rust-model-diesel v0.0.1 (file:///Users/doug/tmp/rust-diesel-example)
    src/lib.rs:7:5: 7:17 warning: unused import, #[warn(unused_imports)] on by default
    src/lib.rs:7 use schema::User;
                     ^~~~~~~~~~~~
    src/lib.rs:17:1: 19:2 warning: enum is never used: `MyErr`, #[warn(dead_code)] on by default
    src/lib.rs:17 enum MyErr {
    src/lib.rs:18   Failed
    src/lib.rs:19 }
    src/lib.rs:21:1: 29:2 warning: function is never used: `users_with_name`, #[warn(dead_code)] on by default
    src/lib.rs:21 fn users_with_name(connection: &Connection, target_name: &str)
    src/lib.rs:22   -> Option<Vec<(i32, String, Option<String>)>>
    src/lib.rs:23 {
    src/lib.rs:24     use self::users::dsl::*;
    src/lib.rs:25     match users.filter(name.eq(target_name)).load(connection) {
    src/lib.rs:26       Ok(x) => Some(x.collect()),
                  ...

    real  1m26.966s
    user  1m32.251s
    sys 0m5.358s
