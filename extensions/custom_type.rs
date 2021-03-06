#![crate_type = "dylib"]
#![feature(plugin)]
#![plugin(interpolate_idents)]

#[macro_use] extern crate cpython;

use cpython::{Python, PyObject, PyRustObject, PyResult};

py_module_initializer!(custom_type, |py, m| {
    try!(m.add(py, "__doc__", "Module documentation string"));
    try!(m.add_type::<i32>(py, "MyType")
        .add("a", py_method!(a()))
        .finish());
    Ok(())
});

fn a(py: Python, slf: &PyRustObject<i32>) -> PyResult<PyObject> {
    println!("a() was called with self={:?}", slf.get(py));
    Ok(py.None())
}

