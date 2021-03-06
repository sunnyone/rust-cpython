// Copyright (c) 2015 Daniel Grunwald
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this
// software and associated documentation files (the "Software"), to deal in the Software
// without restriction, including without limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons
// to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
// INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
// PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE
// FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use std::mem;
use libc;
use ffi;
use python::{Python, PythonObject, PythonObjectWithCheckedDowncast, PythonObjectWithTypeObject, PythonObjectDowncastError, ToPythonPointer};
use objects::PyType;
use err::{PyErr, PyResult};

#[unsafe_no_drop_flag]
#[repr(C)]
pub struct PyObject<'p> {
    // PyObject<'p> owns one reference to the *PyObject
    // ptr is not null (except possibly due to #[unsafe_no_drop_flag])
    ptr: *mut ffi::PyObject,
    py : Python<'p>
}

/// Dropping a `PyObject` decrements the reference count on the object by 1.
impl <'p> Drop for PyObject<'p> {
    #[inline]
    fn drop(&mut self) {
        // TODO: remove if and change Py_XDECREF to Py_DECREF when #[unsafe_no_drop_flag] disappears
        if self.ptr as usize != mem::POST_DROP_USIZE {
            unsafe { ffi::Py_XDECREF(self.ptr); }
        }
    }
}

/// Cloning a `PyObject` increments the reference count on the object by 1.
impl <'p> Clone for PyObject<'p> {
    #[inline]
    fn clone(&self) -> PyObject<'p> {
        unsafe { ffi::Py_INCREF(self.ptr) };
        PyObject { ptr: self.ptr, py: self.py }
    }
}

impl <'p> PythonObject<'p> for PyObject<'p> {
    #[inline]
    fn as_object<'a>(&'a self) -> &'a PyObject<'p> {
        self
    }
    
    #[inline]
    fn into_object(self) -> PyObject<'p> {
        self
    }
    
    #[inline]
    unsafe fn unchecked_downcast_from(o: PyObject<'p>) -> PyObject<'p> {
        o
    }
    
    #[inline]
    unsafe fn unchecked_downcast_borrow_from<'a>(o: &'a PyObject<'p>) -> &'a PyObject<'p> {
        o
    }
    
    #[inline]
    fn python(&self) -> Python<'p> {
        self.py
    }
}

impl <'p> PythonObjectWithCheckedDowncast<'p> for PyObject<'p> {
    #[inline]
    fn downcast_from(obj: PyObject<'p>) -> Result<PyObject<'p>, PythonObjectDowncastError<'p>> {
        Ok(obj)
    }
    
    #[inline]
    fn downcast_borrow_from<'a>(obj: &'a PyObject<'p>) -> Result<&'a PyObject<'p>, PythonObjectDowncastError<'p>> {
        Ok(obj)
    }
}

impl <'p> PythonObjectWithTypeObject<'p> for PyObject<'p> {
    #[inline]
    fn type_object(py: Python<'p>) -> PyType<'p> {
        unsafe { PyType::from_type_ptr(py, &mut ffi::PyBaseObject_Type) }
    }
}

impl <'p> ToPythonPointer for PyObject<'p> {
    #[inline]
    fn as_ptr(&self) -> *mut ffi::PyObject {
        self.ptr
    }
    
    #[inline]
    fn steal_ptr(self) -> *mut ffi::PyObject {
        let ptr = self.ptr;
        mem::forget(self);
        ptr
    }
}


impl <'p> PyObject<'p> {
    /// Creates a PyObject instance for the given FFI pointer.
    /// This moves ownership over the pointer into the PyObject.
    /// Undefined behavior if the pointer is NULL or invalid.
    #[inline]
    pub unsafe fn from_owned_ptr(py : Python<'p>, ptr : *mut ffi::PyObject) -> PyObject<'p> {
        debug_assert!(!ptr.is_null() && ffi::Py_REFCNT(ptr) > 0);
        PyObject { py: py, ptr: ptr }
    }
    
    /// Creates a PyObject instance for the given FFI pointer.
    /// Calls Py_INCREF() on the ptr.
    /// Undefined behavior if the pointer is NULL or invalid.
    #[inline]
    pub unsafe fn from_borrowed_ptr(py : Python<'p>, ptr : *mut ffi::PyObject) -> PyObject<'p> {
        debug_assert!(!ptr.is_null() && ffi::Py_REFCNT(ptr) > 0);
        ffi::Py_INCREF(ptr);
        PyObject { py: py, ptr: ptr }
    }

    /// Creates a PyObject instance for the given FFI pointer.
    /// This moves ownership over the pointer into the PyObject.
    /// Returns None for null pointers; undefined behavior if the pointer is invalid.
    #[inline]
    pub unsafe fn from_owned_ptr_opt(py: Python<'p>, ptr: *mut ffi::PyObject) -> Option<PyObject<'p>> {
        if ptr.is_null() {
            None
        } else {
            Some(PyObject::from_owned_ptr(py, ptr))
        }
    }
    
    /// Returns None for null pointers; undefined behavior if the pointer is invalid.
    #[inline]
    pub unsafe fn from_borrowed_ptr_opt(py: Python<'p>, ptr: *mut ffi::PyObject) -> Option<PyObject<'p>> {
        if ptr.is_null() {
            None
        } else {
            Some(PyObject::from_borrowed_ptr(py, ptr))
        }
    }
    
    /// Transmutes an owned FFI pointer to `&PyObject`.
    /// Undefined behavior if the pointer is NULL or invalid.
    #[inline]
    pub unsafe fn borrow_from_owned_ptr<'a>(py : Python<'p>, ptr : &'a *mut ffi::PyObject) -> &'a PyObject<'p> {
        debug_assert!(!ptr.is_null() && ffi::Py_REFCNT(*ptr) > 0);
        mem::transmute(ptr)
    }
    
    /// Transmutes a slice of owned FFI pointers to `&[PyObject]`.
    /// Undefined behavior if the pointer is NULL or invalid.
    #[inline]
    pub unsafe fn borrow_from_owned_ptr_slice<'a>(py : Python<'p>, ptr : &'a [*mut ffi::PyObject]) -> &'a [PyObject<'p>] {
        mem::transmute(ptr)
    }
    
    /// Gets the reference count of this python object.
    #[inline]
    pub fn get_refcnt(&self) -> usize {
        unsafe { ffi::Py_REFCNT(self.as_ptr()) as usize }
    }

    /// Gets the python type object for this object's type.
    #[inline]
    pub fn get_type(&self) -> &PyType<'p> {
        unsafe {
            let t : &*mut ffi::PyTypeObject = &(*self.as_ptr()).ob_type;
            mem::transmute(t)
        }
    }
    
    /// Casts the PyObject to a concrete python object type.
    /// Causes undefined behavior if the object is not of the expected type.
    /// This is a wrapper function around `PythonObject::unchecked_downcast_from()`.
    #[inline]
    pub unsafe fn unchecked_cast_into<T>(self) -> T where T: PythonObject<'p> {
        PythonObject::unchecked_downcast_from(self)
    }
    
    /// Casts the PyObject to a concrete python object type.
    /// Returns a python `TypeError` if the object is not of the expected type.
    /// This is a wrapper function around `PythonObjectWithCheckedDowncast::downcast_from()`.
    #[inline]
    pub fn cast_into<T>(self) -> Result<T, PythonObjectDowncastError<'p>> where T: PythonObjectWithCheckedDowncast<'p> {
        PythonObjectWithCheckedDowncast::downcast_from(self)
    }
    
    /// Casts the PyObject to a concrete python object type.
    /// Causes undefined behavior if the object is not of the expected type.
    /// This is a wrapper function around `PythonObject::unchecked_downcast_borrow_from()`.
    #[inline]
    pub unsafe fn unchecked_cast_as<'s, T>(&'s self) -> &'s T where T: PythonObject<'p> {
        PythonObject::unchecked_downcast_borrow_from(self)
    }
    
    /// Casts the PyObject to a concrete python object type.
    /// Returns a python `TypeError` if the object is not of the expected type.
    /// This is a wrapper function around `PythonObjectWithCheckedDowncast::downcast_borrow_from()`.
    #[inline]
    pub fn cast_as<'s, T>(&'s self) -> Result<&'s T, PythonObjectDowncastError<'p>> where T: PythonObjectWithCheckedDowncast<'p> {
        PythonObjectWithCheckedDowncast::downcast_borrow_from(self)
    }
    
    /// Extracts some type from the python object.
    /// This is a wrapper function around `FromPyObject::from_py_object()`.
    #[inline]
    pub fn extract<T>(&self) -> Result<T, PyErr<'p>> where T: ::conversion::FromPyObject<'p> {
        ::conversion::FromPyObject::from_py_object(self)
    }
}

/// PyObject implements the `==` operator using reference equality:
/// `obj1 == obj2` in rust is equivalent to `obj1 is obj2` in python.
impl <'p> PartialEq for PyObject<'p> {
    #[inline]
    fn eq(&self, o : &PyObject<'p>) -> bool {
        self.ptr == o.ptr
    }
}

/// PyObject implements the `==` operator using reference equality:
/// `obj1 == obj2` in rust is equivalent to `obj1 is obj2` in python.
impl <'p> Eq for PyObject<'p> { }


#[test]
fn test_sizeof() {
    // should be a static_assert, but size_of is not a compile-time const
    // these are necessary for the transmutes in this module
    assert_eq!(mem::size_of::<PyObject>(), mem::size_of::<*mut ffi::PyObject>());
    assert_eq!(mem::size_of::<PyType>(), mem::size_of::<*mut ffi::PyTypeObject>());
}

