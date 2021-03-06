use libc::{c_void, c_char, c_int, wchar_t};
use object::*;
use pyport::Py_ssize_t;

pub type Py_UCS4 = u32;
pub type Py_UCS2 = u16;
pub type Py_UCS1 = u8;

extern "C" {
    pub static mut PyUnicode_Type: PyTypeObject;
    pub static mut PyUnicodeIter_Type: PyTypeObject;
}

#[inline(always)]
pub unsafe fn PyUnicode_Check(op : *mut PyObject) -> c_int {
    PyType_FastSubclass(Py_TYPE(op), Py_TPFLAGS_UNICODE_SUBCLASS)
}

#[inline(always)]
pub unsafe fn PyUnicode_CheckExact(op : *mut PyObject) -> c_int {
    (Py_TYPE(op) == &mut PyUnicode_Type) as c_int
}

pub const Py_UNICODE_REPLACEMENT_CHARACTER : Py_UCS4 = 0xFFFD;

extern "C" {
    pub fn PyUnicode_FromStringAndSize(u: *const c_char,
                                       size: Py_ssize_t) -> *mut PyObject;
    pub fn PyUnicode_FromString(u: *const c_char) -> *mut PyObject;
    pub fn PyUnicode_Substring(str: *mut PyObject, start: Py_ssize_t,
                               end: Py_ssize_t) -> *mut PyObject;
    pub fn PyUnicode_AsUCS4(unicode: *mut PyObject, buffer: *mut Py_UCS4,
                            buflen: Py_ssize_t, copy_null: c_int)
     -> *mut Py_UCS4;
    pub fn PyUnicode_AsUCS4Copy(unicode: *mut PyObject) -> *mut Py_UCS4;
    pub fn PyUnicode_GetLength(unicode: *mut PyObject) -> Py_ssize_t;
    pub fn PyUnicode_GetSize(unicode: *mut PyObject) -> Py_ssize_t;
    pub fn PyUnicode_ReadChar(unicode: *mut PyObject, index: Py_ssize_t)
     -> Py_UCS4;
    pub fn PyUnicode_WriteChar(unicode: *mut PyObject, index: Py_ssize_t,
                               character: Py_UCS4) -> c_int;
    pub fn PyUnicode_Resize(unicode: *mut *mut PyObject, length: Py_ssize_t)
     -> c_int;
    pub fn PyUnicode_FromEncodedObject(obj: *mut PyObject,
                                       encoding: *const c_char,
                                       errors: *const c_char)
     -> *mut PyObject;
    pub fn PyUnicode_FromObject(obj: *mut PyObject) -> *mut PyObject;
    //pub fn PyUnicode_FromFormatV(format: *const c_char,
    //                             vargs: va_list) -> *mut PyObject;
    pub fn PyUnicode_FromFormat(format: *const c_char, ...)
     -> *mut PyObject;
    pub fn PyUnicode_InternInPlace(arg1: *mut *mut PyObject) -> ();
    pub fn PyUnicode_InternImmortal(arg1: *mut *mut PyObject) -> ();
    pub fn PyUnicode_InternFromString(u: *const c_char)
     -> *mut PyObject;
    pub fn PyUnicode_FromWideChar(w: *const wchar_t, size: Py_ssize_t)
     -> *mut PyObject;
    pub fn PyUnicode_AsWideChar(unicode: *mut PyObject, w: *mut wchar_t,
                                size: Py_ssize_t) -> Py_ssize_t;
    pub fn PyUnicode_AsWideCharString(unicode: *mut PyObject,
                                      size: *mut Py_ssize_t) -> *mut wchar_t;
    pub fn PyUnicode_FromOrdinal(ordinal: c_int) -> *mut PyObject;
    pub fn PyUnicode_ClearFreeList() -> c_int;
    pub fn PyUnicode_GetDefaultEncoding() -> *const c_char;
    pub fn PyUnicode_Decode(s: *const c_char, size: Py_ssize_t,
                            encoding: *const c_char,
                            errors: *const c_char) -> *mut PyObject;
    pub fn PyUnicode_AsDecodedObject(unicode: *mut PyObject,
                                     encoding: *const c_char,
                                     errors: *const c_char)
     -> *mut PyObject;
    pub fn PyUnicode_AsDecodedUnicode(unicode: *mut PyObject,
                                      encoding: *const c_char,
                                      errors: *const c_char)
     -> *mut PyObject;
    pub fn PyUnicode_AsEncodedObject(unicode: *mut PyObject,
                                     encoding: *const c_char,
                                     errors: *const c_char)
     -> *mut PyObject;
    pub fn PyUnicode_AsEncodedString(unicode: *mut PyObject,
                                     encoding: *const c_char,
                                     errors: *const c_char)
     -> *mut PyObject;
    pub fn PyUnicode_AsEncodedUnicode(unicode: *mut PyObject,
                                      encoding: *const c_char,
                                      errors: *const c_char)
     -> *mut PyObject;
    pub fn PyUnicode_BuildEncodingMap(string: *mut PyObject) -> *mut PyObject;
    pub fn PyUnicode_DecodeUTF7(string: *const c_char,
                                length: Py_ssize_t,
                                errors: *const c_char)
     -> *mut PyObject;
    pub fn PyUnicode_DecodeUTF7Stateful(string: *const c_char,
                                        length: Py_ssize_t,
                                        errors: *const c_char,
                                        consumed: *mut Py_ssize_t)
     -> *mut PyObject;
    pub fn PyUnicode_DecodeUTF8(string: *const c_char,
                                length: Py_ssize_t,
                                errors: *const c_char)
     -> *mut PyObject;
    pub fn PyUnicode_DecodeUTF8Stateful(string: *const c_char,
                                        length: Py_ssize_t,
                                        errors: *const c_char,
                                        consumed: *mut Py_ssize_t)
     -> *mut PyObject;
    pub fn PyUnicode_AsUTF8String(unicode: *mut PyObject) -> *mut PyObject;
    pub fn PyUnicode_DecodeUTF32(string: *const c_char,
                                 length: Py_ssize_t,
                                 errors: *const c_char,
                                 byteorder: *mut c_int)
     -> *mut PyObject;
    pub fn PyUnicode_DecodeUTF32Stateful(string: *const c_char,
                                         length: Py_ssize_t,
                                         errors: *const c_char,
                                         byteorder: *mut c_int,
                                         consumed: *mut Py_ssize_t)
     -> *mut PyObject;
    pub fn PyUnicode_AsUTF32String(unicode: *mut PyObject) -> *mut PyObject;
    pub fn PyUnicode_DecodeUTF16(string: *const c_char,
                                 length: Py_ssize_t,
                                 errors: *const c_char,
                                 byteorder: *mut c_int)
     -> *mut PyObject;
    pub fn PyUnicode_DecodeUTF16Stateful(string: *const c_char,
                                         length: Py_ssize_t,
                                         errors: *const c_char,
                                         byteorder: *mut c_int,
                                         consumed: *mut Py_ssize_t)
     -> *mut PyObject;
    pub fn PyUnicode_AsUTF16String(unicode: *mut PyObject) -> *mut PyObject;
    pub fn PyUnicode_DecodeUnicodeEscape(string: *const c_char,
                                         length: Py_ssize_t,
                                         errors: *const c_char)
     -> *mut PyObject;
    pub fn PyUnicode_AsUnicodeEscapeString(unicode: *mut PyObject)
     -> *mut PyObject;
    pub fn PyUnicode_DecodeRawUnicodeEscape(string: *const c_char,
                                            length: Py_ssize_t,
                                            errors: *const c_char)
     -> *mut PyObject;
    pub fn PyUnicode_AsRawUnicodeEscapeString(unicode: *mut PyObject)
     -> *mut PyObject;
    pub fn PyUnicode_DecodeLatin1(string: *const c_char,
                                  length: Py_ssize_t,
                                  errors: *const c_char)
     -> *mut PyObject;
    pub fn PyUnicode_AsLatin1String(unicode: *mut PyObject) -> *mut PyObject;
    pub fn PyUnicode_DecodeASCII(string: *const c_char,
                                 length: Py_ssize_t,
                                 errors: *const c_char)
     -> *mut PyObject;
    pub fn PyUnicode_AsASCIIString(unicode: *mut PyObject) -> *mut PyObject;
    pub fn PyUnicode_DecodeCharmap(string: *const c_char,
                                   length: Py_ssize_t, mapping: *mut PyObject,
                                   errors: *const c_char)
     -> *mut PyObject;
    pub fn PyUnicode_AsCharmapString(unicode: *mut PyObject,
                                     mapping: *mut PyObject) -> *mut PyObject;
    pub fn PyUnicode_DecodeLocaleAndSize(str: *const c_char,
                                         len: Py_ssize_t,
                                         errors: *const c_char)
     -> *mut PyObject;
    pub fn PyUnicode_DecodeLocale(str: *const c_char,
                                  errors: *const c_char)
     -> *mut PyObject;
    pub fn PyUnicode_EncodeLocale(unicode: *mut PyObject,
                                  errors: *const c_char)
     -> *mut PyObject;
    pub fn PyUnicode_FSConverter(arg1: *mut PyObject,
                                 arg2: *mut c_void) -> c_int;
    pub fn PyUnicode_FSDecoder(arg1: *mut PyObject, arg2: *mut c_void)
     -> c_int;
    pub fn PyUnicode_DecodeFSDefault(s: *const c_char)
     -> *mut PyObject;
    pub fn PyUnicode_DecodeFSDefaultAndSize(s: *const c_char,
                                            size: Py_ssize_t)
     -> *mut PyObject;
    pub fn PyUnicode_EncodeFSDefault(unicode: *mut PyObject) -> *mut PyObject;
    pub fn PyUnicode_Concat(left: *mut PyObject, right: *mut PyObject)
     -> *mut PyObject;
    pub fn PyUnicode_Append(pleft: *mut *mut PyObject, right: *mut PyObject)
     -> ();
    pub fn PyUnicode_AppendAndDel(pleft: *mut *mut PyObject,
                                  right: *mut PyObject) -> ();
    pub fn PyUnicode_Split(s: *mut PyObject, sep: *mut PyObject,
                           maxsplit: Py_ssize_t) -> *mut PyObject;
    pub fn PyUnicode_Splitlines(s: *mut PyObject, keepends: c_int)
     -> *mut PyObject;
    pub fn PyUnicode_Partition(s: *mut PyObject, sep: *mut PyObject)
     -> *mut PyObject;
    pub fn PyUnicode_RPartition(s: *mut PyObject, sep: *mut PyObject)
     -> *mut PyObject;
    pub fn PyUnicode_RSplit(s: *mut PyObject, sep: *mut PyObject,
                            maxsplit: Py_ssize_t) -> *mut PyObject;
    pub fn PyUnicode_Translate(str: *mut PyObject, table: *mut PyObject,
                               errors: *const c_char)
     -> *mut PyObject;
    pub fn PyUnicode_Join(separator: *mut PyObject, seq: *mut PyObject)
     -> *mut PyObject;
    pub fn PyUnicode_Tailmatch(str: *mut PyObject, substr: *mut PyObject,
                               start: Py_ssize_t, end: Py_ssize_t,
                               direction: c_int) -> Py_ssize_t;
    pub fn PyUnicode_Find(str: *mut PyObject, substr: *mut PyObject,
                          start: Py_ssize_t, end: Py_ssize_t,
                          direction: c_int) -> Py_ssize_t;
    pub fn PyUnicode_FindChar(str: *mut PyObject, ch: Py_UCS4,
                              start: Py_ssize_t, end: Py_ssize_t,
                              direction: c_int) -> Py_ssize_t;
    pub fn PyUnicode_Count(str: *mut PyObject, substr: *mut PyObject,
                           start: Py_ssize_t, end: Py_ssize_t) -> Py_ssize_t;
    pub fn PyUnicode_Replace(str: *mut PyObject, substr: *mut PyObject,
                             replstr: *mut PyObject, maxcount: Py_ssize_t)
     -> *mut PyObject;
    pub fn PyUnicode_Compare(left: *mut PyObject, right: *mut PyObject)
     -> c_int;
    pub fn PyUnicode_CompareWithASCIIString(left: *mut PyObject,
                                            right: *const c_char)
     -> c_int;
    pub fn PyUnicode_RichCompare(left: *mut PyObject, right: *mut PyObject,
                                 op: c_int) -> *mut PyObject;
    pub fn PyUnicode_Format(format: *mut PyObject, args: *mut PyObject)
     -> *mut PyObject;
    pub fn PyUnicode_Contains(container: *mut PyObject,
                              element: *mut PyObject) -> c_int;
    pub fn PyUnicode_IsIdentifier(s: *mut PyObject) -> c_int;
}

