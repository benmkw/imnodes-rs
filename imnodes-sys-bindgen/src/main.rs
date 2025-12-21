// All this crate does is run bindgen on cimplot and store the result
// in the src folder of the imnodes-sys crate. We add those bindings
// to git so people don't have to install clang just to use imnodes-rs.

fn main() {
    // Use CARGO_MANIFEST_DIR to find the workspace root reliably
    let manifest_dir = std::path::PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"),
    );
    let sys_crate_path = manifest_dir
        .join("..")
        .join("imnodes-sys")
        .canonicalize()
        .expect("Could not find sys crate directory");

    let cimgui_include_path = std::path::PathBuf::from(
        std::env::var_os("DEP_IMGUI_THIRD_PARTY").expect("DEP_IMGUI_THIRD_PARTY not defined"),
    );

    let cimnodes_include_path = sys_crate_path.join("third-party").join("cimnodes");

    let imvec2_c_path = sys_crate_path.join("third-party").join("imvec2_c.h");

    let bindings = bindgen::Builder::default()
        // https://github.com/rust-lang/rust-bindgen/issues/1533
        .clang_arg(format!("-I{}", cimgui_include_path.to_str().unwrap()))
        .clang_arg(format!("-I{}", cimnodes_include_path.to_str().unwrap()))
        // Force-include ImVec2_c definition before any cimnodes headers.
        // The cimnodes generator expects cimgui to define ImVec2_c, but imgui-sys's
        // cimgui uses ImVec2 instead. This header bridges the gap.
        .clang_arg(format!("-include{}", imvec2_c_path.to_str().unwrap()))
        .header(cimnodes_include_path.join("cimnodes.h").to_str().unwrap())
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
