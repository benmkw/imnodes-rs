// All this crate does is run bindgen on cimplot and store the result
// in the src folder of the imnodes-sys crate. We add those bindings
// to git so people don't have to install clang just to use imnodes-rs.

fn main() {
    let cwd = std::env::current_dir().expect("Could not read current directory");
    let sys_crate_path = cwd
        .join("..")
        .join("imnodes-sys")
        .canonicalize()
        .expect("Could not find sys crate directory");

    let cimgui_include_path = std::path::PathBuf::from(
        std::env::var_os("DEP_IMGUI_THIRD_PARTY").expect("DEP_IMGUI_THIRD_PARTY not defined"),
    );

    let bindings = bindgen::Builder::default()
        .header(
            cimgui_include_path
                .join("cimgui.h")
                .to_str()
                .expect("Could not convert cimgui.h path to string"),
        )
        .header(
            sys_crate_path
                .join("third-party")
                .join("cimnodes")
                .join("cimnodes.h")
                .to_str()
                .expect("Could not turn cimnodes.h path into string"),
        )
        // https://github.com/rust-lang/rust-bindgen/issues/1533
        .clang_arg(format!("-I{}", cimgui_include_path.to_str().unwrap()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_arg("-DCIMGUI_DEFINE_ENUMS_AND_STRUCTS=1")
        .allowlist_function("imnodes_.*")
        .allowlist_function("ImNodes.*")
        .allowlist_type("ImNodes.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = sys_crate_path.join("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
