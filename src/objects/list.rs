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

use std;
use python::{Python, PythonObject, ToPythonPointer};
use err::{self, PyResult, PyErr};
use super::object::PyObject;
use super::exc;
use ffi::{self, Py_ssize_t};
use conversion::{ToPyObject, FromPyObject};

pyobject_newtype!(PyList, PyList_Check, PyList_Type);

impl <'p> PyList<'p> {
    /// Construct a new list with the given elements.
    pub fn new(py: Python<'p>, elements: &[PyObject<'p>]) -> PyList<'p> {
        unsafe {
            let ptr = ffi::PyList_New(elements.len() as Py_ssize_t);
            let t = err::result_from_owned_ptr(py, ptr).unwrap().unchecked_cast_into::<PyList>();
            for (i, e) in elements.iter().enumerate() {
                ffi::PyList_SetItem(ptr, i as Py_ssize_t, e.clone().steal_ptr());
            }
            t
        }
    }

    /// Gets the length of the list.
    #[inline]
    pub fn len(&self) -> usize {
        // non-negative Py_ssize_t should always fit into Rust usize
        unsafe {
            ffi::PyList_Size(self.as_ptr()) as usize
        }
    }

    /// Gets the item at the specified index.
    ///
    /// Panics if the index is out of range.
    pub fn get_item(&self, index: usize) -> PyObject<'p> {
        assert!(index < self.len());
        unsafe {
            PyObject::from_borrowed_ptr(self.python(), ffi::PyList_GetItem(self.as_ptr(), index as Py_ssize_t))
        }
    }

    /// Sets the item at the specified index.
    ///
    /// Panics if the index is out of range.
    pub fn set_item(&self, index: usize, item: PyObject<'p>) {
        let r = unsafe { ffi::PyList_SetItem(self.as_ptr(), index as Py_ssize_t, item.steal_ptr()) };
        assert!(r == 0);
    }

    /// Inserts an item at the specified index.
    ///
    /// Panics if the index is out of range.
    pub fn insert_item(&self, index: usize, item: PyObject<'p>) {
        let r = unsafe { ffi::PyList_Insert(self.as_ptr(), index as Py_ssize_t, item.as_ptr()) };
        assert!(r == 0);
    }
}

impl <'p> IntoIterator for PyList<'p> {
    type Item = PyObject<'p>;
    type IntoIter = PyListIterator<'p>;

    #[inline]
    fn into_iter(self) -> PyListIterator<'p> {
        PyListIterator { list: self, index: 0 }
    }
}

impl <'a, 'p> IntoIterator for &'a PyList<'p> {
    type Item = PyObject<'p>;
    type IntoIter = PyListIterator<'p>;

    #[inline]
    fn into_iter(self) -> PyListIterator<'p> {
        PyListIterator { list: self.clone(), index: 0 }
    }
}

pub struct PyListIterator<'p> {
    list: PyList<'p>,
    index: usize
}

impl <'p> Iterator for PyListIterator<'p> {
    type Item = PyObject<'p>;

    #[inline]
    fn next(&mut self) -> Option<PyObject<'p>> {
        if self.index < self.list.len() {
            let item = self.list.get_item(self.index);
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }

    // Note: we cannot implement size_hint because the length of the list
    // might change during the iteration.
}

impl <'p, T> ToPyObject<'p> for [T] where T: ToPyObject<'p> {
    type ObjectType = PyList<'p>;

    fn to_py_object(&self, py: Python<'p>) -> PyList<'p> {
        unsafe {
            let ptr = ffi::PyList_New(self.len() as Py_ssize_t);
            let t = err::cast_from_owned_ptr_or_panic(py, ptr);
            for (i, e) in self.iter().enumerate() {
                let obj = e.to_py_object(py);
                ffi::PyList_SetItem(ptr, i as Py_ssize_t, obj.steal_ptr());
            }
            t
        }
    }
}

impl <'p, T> FromPyObject<'p> for Vec<T> where T: FromPyObject<'p> {
    fn from_py_object(s: &PyObject<'p>) -> PyResult<'p, Vec<T>> {
        let py = s.python();
        let list = try!(s.cast_as::<PyList>());
        let mut v = Vec::with_capacity(list.len());
        for i in 0 .. list.len() {
            v.push(try!(list.get_item(i).extract::<T>()));
        }
        Ok(v)
    }
}

