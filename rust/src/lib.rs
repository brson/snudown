#![feature(proc_macro)]
#![allow(unused)]

extern crate pyo3;

use pyo3::prelude::*;

/*#[py::modinit(snudown)]
fn init(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "run", args="*", kwargs="**")]
    fn run_fn(_py: Python, args: &PyTuple, kwargs: Option<&PyDict>) -> PyResult<()> {
        run(args, kwargs)
    }

    #[pyfn(m, "val")]
    fn val(_py: Python) -> PyResult<i32> {
        Ok(42)
    }

    #[pyfn(m, "markdown")]
    fn snudown_md(_py: Python, self_: &PyObject,
                  args: &PyObject, kwargs: &PyObject) -> PyResult<PyObject> {
        panic!()
    }

    Ok(())
}*/

extern {
    fn initsnudown_real();
}

#[no_mangle]
pub unsafe extern fn initsnudown() {
    initsnudown_real()
}

fn run(args: &PyTuple, kwargs: Option<&PyDict>) -> PyResult<()> {
    println!("Rust says: Hello Python!");
    for arg in args.iter() {
        println!("Rust got {}", arg);
    }
    if let Some(kwargs) = kwargs {
        for (key, val) in kwargs.iter() {
            println!("{} = {}", key, val);
        }
    }
    Ok(())
}

#[allow(bad_style)]
pub mod c {
    extern crate libc;
    
    pub mod markdown {

        use super::libc::{c_int, c_uint, uint8_t, c_void, size_t};

        #[repr(C)]
        pub struct sd_callbacks {
            blockcode: unsafe extern fn(ob: *mut buf, text: *const buf,
                                        lang: *const buf, opaque: *mut c_void),
        }

        #[repr(C)]
        pub struct buf {
            data: *mut uint8_t,
            size: size_t,
            asize: size_t,
            unit: size_t,
        }

        pub struct sd_markdown {
        }        
        
        #[no_mangle]
        #[export_name = "rsd_markdown_new"]
        pub extern fn sd_markdown_new(
            extensions: c_uint,
            max_nesting: size_t,
            max_table_cols: size_t,
            callbacks: *mut sd_callbacks,
            opaque: *mut c_void
        ) -> *mut sd_markdown {
            panic!()
        }

        #[no_mangle]
        #[export_name = "rsd_markdown_render"]
        pub extern fn sd_markdown_render(
            ob: *mut buf,
            document: *const uint8_t,
            doc_size: size_t,
            md: *mut sd_markdown
        ) {
            panic!()
        }

        #[no_mangle]
        #[export_name = "rsd_markdown_free"]
        pub extern fn sd_markdown_free(md: *mut sd_markdown) {
            panic!()
        }

        #[no_mangle]
        #[export_name = "rsd_version"]
        pub extern fn sd_version(
            major: *mut c_int,
            minor: *mut c_int,
            revision: *mut c_int
        ) {
            panic!()
        }
    }
}

