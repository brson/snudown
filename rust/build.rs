extern crate cc;

fn main() {

    let files = [
        "../src/autolink.c",
        "../src/buffer.c",
        "../src/markdown.c",
        "../src/stack.c",
        "../html/html.c",
        "../html/houdini_href_e.c",
        "../html/houdini_html_e.c",
        "../html/html_smartypants.c",
        "../snudown.c",
    ];
    
    cc::Build::new()
        .include("../html")
        .include("../src/")
        .include("/usr/include/python2.7/")
        .define("RUST", "1")
        .warnings(false)
        .files(&files)
        .compile("snudown_native");
}
