fn main() {
    cc::Build::new()
        .file("c_external/nifti2_io.c")
        .file("c_external/znzlib.c")
        .include("c_external")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-variable")
        .compile("nifti_c_lib");
}