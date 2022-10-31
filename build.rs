use glib_build_tools::compile_resources;

fn main() {
    compile_resources(
        "content",
        "content/app.gresource.xml",
        "password-manager.gresource"
    );
}