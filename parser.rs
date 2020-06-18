use libc::*;
use crate::public::*;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ldat;
    #[no_mangle]
    fn fread(__ptr: *mut c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const c_char, _: ...) -> c_int;
    #[no_mangle]
    fn fopen(__filename: *const c_char, __modes: *const c_char)
     -> *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> c_int;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn mbsrtowcs(__dst: *mut wchar_t, __src: *mut *const c_char,
                 __len: size_t, __ps: *mut mbstate_t) -> size_t;
    #[no_mangle]
    fn wmemcpy(__s1: *mut wchar_t, __s2: *const wchar_t, __n: size_t)
     -> *mut wchar_t;
    #[no_mangle]
    fn wcslen(_: *const c_int) -> c_ulong;
    #[no_mangle]
    fn wcscspn(__wcs: *const wchar_t, __reject: *const wchar_t) -> size_t;
    #[no_mangle]
    fn stfl_widget_new(type_0: *const wchar_t) -> *mut stfl_widget;
    #[no_mangle]
    fn stfl_widget_setkv_str(w: *mut stfl_widget, key: *const wchar_t,
                             value: *const wchar_t) -> *mut stfl_kv;
    #[no_mangle]
    fn malloc(_: c_ulong) -> *mut c_void;
    #[no_mangle]
    fn realloc(_: *mut c_void, _: c_ulong) -> *mut c_void;
    #[no_mangle]
    fn free(__ptr: *mut c_void);
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn mbstowcs(__pwcs: *mut wchar_t, __s: *const c_char, __n: size_t)
     -> size_t;
    #[no_mangle]
    fn wcstombs(__s: *mut c_char, __pwcs: *const wchar_t, __n: size_t)
     -> size_t;
    #[no_mangle]
    fn memcpy(_: *mut c_void, _: *const c_void, _: c_ulong)
     -> *mut c_void;
    #[no_mangle]
    fn strlen(_: *const c_char) -> c_ulong;
    #[no_mangle]
    fn __assert_fail(__assertion: *const c_char,
                     __file: *const c_char, __line: c_uint,
                     __function: *const c_char) -> !;
}
pub type size_t = c_ulong;
pub type wchar_t = c_int;
pub type wint_t = c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: c_uint,
    pub __wchb: [c_char; 4],
}
pub type mbstate_t = __mbstate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: c_int,
    pub _IO_read_ptr: *mut c_char,
    pub _IO_read_end: *mut c_char,
    pub _IO_read_base: *mut c_char,
    pub _IO_write_base: *mut c_char,
    pub _IO_write_ptr: *mut c_char,
    pub _IO_write_end: *mut c_char,
    pub _IO_buf_base: *mut c_char,
    pub _IO_buf_end: *mut c_char,
    pub _IO_save_base: *mut c_char,
    pub _IO_backup_base: *mut c_char,
    pub _IO_save_end: *mut c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: c_int,
    pub _flags2: c_int,
    pub _old_offset: __off_t,
    pub _cur_column: c_ushort,
    pub _vtable_offset: c_schar,
    pub _shortbuf: [c_char; 1],
    pub _lock: *mut c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut c_void,
    pub __pad5: size_t,
    pub _mode: c_int,
    pub _unused2: [c_char; 20],
}
pub type __off64_t = c_long;
pub type _IO_lock_t = ();
pub type __off_t = c_long;
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cchar_t {
    pub attr: attr_t,
    pub chars: [wchar_t; 5],
    pub ext_color: c_int,
}
pub type attr_t = chtype;
pub type chtype = c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdat {
    pub _pad_y: c_short,
    pub _pad_x: c_short,
    pub _pad_top: c_short,
    pub _pad_left: c_short,
    pub _pad_bottom: c_short,
    pub _pad_right: c_short,
}
/*
 *  STFL - The Structured Terminal Forms Language/Library
 *  Copyright (C) 2006, 2007  Clifford Wolf <clifford@clifford.at>
 *
 *  This library is free software; you can redistribute it and/or
 *  modify it under the terms of the GNU Lesser General Public
 *  License as published by the Free Software Foundation; either
 *  version 3 of the License, or (at your option) any later version.
 *  
 *  This library is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 *  Lesser General Public License for more details.
 *  
 *  You should have received a copy of the GNU Lesser General Public
 *  License along with this library; if not, write to the Free Software
 *  Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
 *  MA 02110-1301 USA
 *
 *  stfl_internals.h: The STFL C header file (Internal STFL APIs)
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stfl_kv {
    pub next: *mut stfl_kv,
    pub widget: *mut stfl_widget,
    pub key: *mut wchar_t,
    pub value: *mut wchar_t,
    pub name: *mut wchar_t,
    pub id: c_int,
}
pub type C2RustUnnamed_0 = c_uint;
pub const DOUBLE_QUOTE_NAME: C2RustUnnamed_0 = 5;
pub const DOUBLE_QUOTE: C2RustUnnamed_0 = 4;
pub const SINGLE_QUOTE_NAME: C2RustUnnamed_0 = 3;
pub const SINGLE_QUOTE: C2RustUnnamed_0 = 2;
pub const NAME_BLOCK: C2RustUnnamed_0 = 1;
pub const PLAIN: C2RustUnnamed_0 = 0;
/*
 *  STFL - The Structured Terminal Forms Language/Library
 *  Copyright (C) 2007  Clifford Wolf <clifford@clifford.at>
 *
 *  This library is free software; you can redistribute it and/or
 *  modify it under the terms of the GNU Lesser General Public
 *  License as published by the Free Software Foundation; either
 *  version 3 of the License, or (at your option) any later version.
 *  
 *  This library is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 *  Lesser General Public License for more details.
 *  
 *  You should have received a copy of the GNU Lesser General Public
 *  License along with this library; if not, write to the Free Software
 *  Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
 *  MA 02110-1301 USA
 *
 *  stfl_compat.h: Some compatibility hacks for b0rken architectures
 */
#[inline]
unsafe extern "C" fn compat_wcsdup(src: *const wchar_t) -> *mut wchar_t {
    let n: size_t =
        wcslen(src).wrapping_add(1 as c_int as
                                     c_ulong).wrapping_mul(::std::mem::size_of::<wchar_t>()
                                                                     as
                                                                     c_ulong);
    let dest: *mut wchar_t = malloc(n) as *mut wchar_t;
    memcpy(dest as *mut c_void, src as *const c_void, n);
    return dest;
}
unsafe extern "C" fn mywcscspn(wcs: *const wchar_t,
                               reject: *const wchar_t,
                               flags: c_int) -> size_t {
    let mut state: C2RustUnnamed_0 = PLAIN;
    let mut len: c_int = 0 as c_int;
    loop  {
        if *wcs.offset(len as isize) == 0 { return len as size_t }
        match state as c_uint {
            0 => {
                if flags & 0x2 as c_int != 0 &&
                       *wcs.offset(len as isize) == '[' as i32 {
                    state = NAME_BLOCK
                } else if flags & 0x1 as c_int != 0 &&
                              *wcs.offset(len as isize) == '\'' as i32 {
                    state = SINGLE_QUOTE
                } else if flags & 0x1 as c_int != 0 &&
                              *wcs.offset(len as isize) == '\"' as i32 {
                    state = DOUBLE_QUOTE
                } else {
                    let mut i = 0 as c_int;
                    while *reject.offset(i as isize) != 0 {
                        if *wcs.offset(len as isize) ==
                               *reject.offset(i as isize) {
                            return len as size_t
                        }
                        i += 1
                    }
                }
            }
            1 => {
                if flags & 0x1 as c_int != 0 &&
                       *wcs.offset(len as isize) == '\'' as i32 {
                    state = SINGLE_QUOTE_NAME
                } else if flags & 0x1 as c_int != 0 &&
                              *wcs.offset(len as isize) == '\"' as i32 {
                    state = DOUBLE_QUOTE_NAME
                } else if *wcs.offset(len as isize) == ']' as i32 {
                    state = PLAIN
                }
            }
            2 | 3 => {
                if *wcs.offset(len as isize) == '\'' as i32 {
                    state =
                        if state as c_uint ==
                               SINGLE_QUOTE as c_int as c_uint {
                            PLAIN as c_int
                        } else { NAME_BLOCK as c_int } as
                            C2RustUnnamed_0
                }
            }
            4 | 5 => {
                if *wcs.offset(len as isize) == '\"' as i32 {
                    state =
                        if state as c_uint ==
                               DOUBLE_QUOTE as c_int as c_uint {
                            PLAIN as c_int
                        } else { NAME_BLOCK as c_int } as
                            C2RustUnnamed_0
                }
            }
            _ => { }
        }
        len += 1
    };
}
unsafe extern "C" fn unquote(text: *const wchar_t, tlen: c_int)
 -> *mut wchar_t {
    let mut len_v: c_int = 0 as c_int;
    if text.is_null() { return 0 as *mut wchar_t }
    let mut i = 0 as c_int;
    while (i < tlen || tlen < 0 as c_int) && *text.offset(i as isize) != 0 {
        if *text.offset(i as isize) == '\'' as i32 {
            loop  {
                i += 1;
                if i == tlen && tlen >= 0 as c_int { break ; }
                if *text.offset(i as isize) == 0 ||
                       *text.offset(i as isize) == '\'' as i32 {
                    break ;
                }
                len_v += 1
            }
        } else if *text.offset(i as isize) == '\"' as i32 {
            loop  {
                i += 1;
                if i == tlen && tlen >= 0 as c_int { break ; }
                if *text.offset(i as isize) == 0 ||
                       *text.offset(i as isize) == '\"' as i32 {
                    break ;
                }
                len_v += 1
            }
        } else { len_v += 1 }
        i += 1
    }
    let value =
        malloc((::std::mem::size_of::<wchar_t>() as
                    c_ulong).wrapping_mul((len_v + 1 as c_int) as
                                                    c_ulong)) as
            *mut wchar_t;
    let mut j = 0 as c_int;
    let mut i = j;
    while (i < tlen || tlen < 0 as c_int) &&
              *text.offset(i as isize) != 0 {
        if *text.offset(i as isize) == '\'' as i32 {
            loop  {
                i += 1;
                if i == tlen && tlen >= 0 as c_int { break ; }
                if *text.offset(i as isize) == 0 ||
                       *text.offset(i as isize) == '\'' as i32 {
                    break ;
                }
                let fresh0 = j;
                j = j + 1;
                *value.offset(fresh0 as isize) = *text.offset(i as isize)
            }
        } else if *text.offset(i as isize) == '\"' as i32 {
            loop  {
                i += 1;
                if i == tlen && tlen >= 0 as c_int { break ; }
                if *text.offset(i as isize) == 0 ||
                       *text.offset(i as isize) == '\"' as i32 {
                    break ;
                }
                let fresh1 = j;
                j = j + 1;
                *value.offset(fresh1 as isize) = *text.offset(i as isize)
            }
        } else {
            let fresh2 = j;
            j = j + 1;
            *value.offset(fresh2 as isize) = *text.offset(i as isize)
        }
        i += 1
    }
    *value.offset(j as isize) = 0 as c_int;
    if j == len_v {
    } else {
        __assert_fail(b"j == len_v\x00" as *const u8 as *const c_char,
                      b"parser.c\x00" as *const u8 as *const c_char,
                      155 as c_int as c_uint,
                      (*::std::mem::transmute::<&[u8; 39],
                                                &[c_char; 39]>(b"wchar_t *unquote(const wchar_t *, int)\x00")).as_ptr());
    }
    return value;
}
#[target_feature(enable = "avx")]
unsafe extern "C" fn extract_name(key: *mut *mut wchar_t,
                                  name: *mut *mut wchar_t) {
    let mut len: c_int =
        wcscspn(*key,
                (*::std::mem::transmute::<&[u8; 8],
                                          &[c_int; 2]>(b"[\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
            as c_int;
    if *(*key).offset(len as isize) == 0 as c_int {
        *name = 0 as *mut wchar_t;
        return
    }
    *name =
        compat_wcsdup((*key).offset(len as
                                        isize).offset(1 as c_int as
                                                          isize));
    *key =
        realloc(*key as *mut c_void,
                (::std::mem::size_of::<wchar_t>() as
                     c_ulong).wrapping_mul((len + 1 as c_int) as
                                                     c_ulong)) as
            *mut wchar_t;
    *(*key).offset(len as isize) = 0 as c_int;
    len =
        mywcscspn(*name,
                  (*::std::mem::transmute::<&[u8; 8],
                                            &[c_int; 2]>(b"]\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                  0x1 as c_int) as c_int;
    *(*name).offset(len as isize) = 0 as c_int;
}
unsafe extern "C" fn extract_class(key: *mut *mut wchar_t,
                                   cls: *mut *mut wchar_t) {
    let len: c_int =
        wcscspn(*key,
                (*::std::mem::transmute::<&[u8; 8],
                                          &[c_int; 2]>(b"#\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
            as c_int;
    if *(*key).offset(len as isize) == 0 as c_int {
        *cls = 0 as *mut wchar_t;
        return
    }
    *cls =
        compat_wcsdup((*key).offset(len as
                                        isize).offset(1 as c_int as
                                                          isize));
    *key =
        realloc(*key as *mut c_void,
                (::std::mem::size_of::<wchar_t>() as
                     c_ulong).wrapping_mul((len + 1 as c_int) as
                                                     c_ulong)) as
            *mut wchar_t;
    *(*key).offset(len as isize) = 0 as c_int;
}
unsafe extern "C" fn read_type(text: *mut *const wchar_t,
                               type_0: *mut *mut wchar_t,
                               name: *mut *mut wchar_t,
                               cls: *mut *mut wchar_t) -> c_int {
    let len: c_int =
        mywcscspn(*text,
                  (*::std::mem::transmute::<&[u8; 32],
                                            &[c_int; 8]>(b" \x00\x00\x00\t\x00\x00\x00\r\x00\x00\x00\n\x00\x00\x00:\x00\x00\x00{\x00\x00\x00}\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                  0x1 as c_int | 0x2 as c_int) as c_int;
    if *(*text).offset(len as isize) == ':' as i32 || len == 0 as c_int
       {
        return 0 as c_int
    }
    *type_0 =
        malloc(((len + 1 as c_int) as
                    c_ulong).wrapping_mul(::std::mem::size_of::<wchar_t>()
                                                    as c_ulong)) as
            *mut wchar_t;
    wmemcpy(*type_0, *text, len as size_t);
    *(*type_0).offset(len as isize) = 0 as c_int;
    *text = (*text).offset(len as isize);
    extract_name(type_0, name);
    extract_class(type_0, cls);
    return 1 as c_int;
}
unsafe extern "C" fn read_kv(text: *mut *const wchar_t,
                             key: *mut *mut wchar_t,
                             name: *mut *mut wchar_t,
                             value: *mut *mut wchar_t) -> c_int {
    let len_k: c_int =
        mywcscspn(*text,
                  (*::std::mem::transmute::<&[u8; 32],
                                            &[c_int; 8]>(b" \x00\x00\x00\t\x00\x00\x00\r\x00\x00\x00\n\x00\x00\x00:\x00\x00\x00{\x00\x00\x00}\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                  0x1 as c_int | 0x2 as c_int) as c_int;
    if *(*text).offset(len_k as isize) != ':' as i32 ||
           len_k == 0 as c_int {
        return 0 as c_int
    }
    *key =
        malloc(((len_k + 1 as c_int) as
                    c_ulong).wrapping_mul(::std::mem::size_of::<wchar_t>()
                                                    as c_ulong)) as
            *mut wchar_t;
    wmemcpy(*key, *text, len_k as size_t);
    *(*key).offset(len_k as isize) = 0 as c_int;
    *text = (*text).offset((len_k + 1 as c_int) as isize);
    extract_name(key, name);
    let qval_len: c_int =
        mywcscspn(*text,
                  (*::std::mem::transmute::<&[u8; 28],
                                            &[c_int; 7]>(b" \x00\x00\x00\t\x00\x00\x00\r\x00\x00\x00\n\x00\x00\x00{\x00\x00\x00}\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                  0x1 as c_int) as c_int;
    *value = unquote(*text, qval_len);
    *text = (*text).offset(qval_len as isize);
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_parser(mut text: *const wchar_t)
 -> *mut stfl_widget {
    let current_block: u64;
    let mut root: *mut stfl_widget = 0 as *mut stfl_widget;
    let mut current: *mut stfl_widget = 0 as *mut stfl_widget;
    let mut bracket_indenting: c_int = -(1 as c_int);
    let mut bracket_level: c_int = 0 as c_int;
    's_17:
        loop  {
            let mut indenting: c_int = 0 as c_int;
            if bracket_indenting >= 0 as c_int {
                while *text == ' ' as i32 || *text == '\t' as i32 {
                    text = text.offset(1)
                }
                while *text == '}' as i32 {
                    bracket_level -= 1;
                    text = text.offset(1);
                    while *text == ' ' as i32 || *text == '\t' as i32 {
                        text = text.offset(1)
                    }
                }
                while *text == '{' as i32 {
                    bracket_level += 1;
                    text = text.offset(1);
                    while *text == ' ' as i32 || *text == '\t' as i32 {
                        text = text.offset(1)
                    }
                }
                if bracket_level == 0 as c_int {
                    bracket_indenting = -(1 as c_int)
                }
                if bracket_level < 0 as c_int {
                    current_block = 6712573431332408062;
                    break ;
                }
            } else if *text == '}' as i32 {
                current_block = 6712573431332408062;
                break ;
            }
            if bracket_indenting >= 0 as c_int {
                while *text == ' ' as i32 || *text == '\t' as i32 {
                    text = text.offset(1)
                }
                if *text == '\r' as i32 || *text == '\n' as i32 {
                    current_block = 6712573431332408062;
                    break ;
                }
                indenting =
                    bracket_indenting + (bracket_level - 1 as c_int)
            } else {
                while *text == ' ' as i32 || *text == '\t' as i32 ||
                          *text == '\r' as i32 || *text == '\n' as i32 {
                    if *text == '\r' as i32 || *text == '\n' as i32 {
                        indenting = 0 as c_int
                    } else if *text == '\t' as i32 {
                        indenting = -(1 as c_int)
                    } else if indenting >= 0 as c_int { indenting += 1 }
                    text = text.offset(1)
                }
                if *text == '*' as i32 {
                    while *text != 0 && *text != '\r' as i32 &&
                              *text != '\n' as i32 {
                        text = text.offset(1)
                    }
                    continue ;
                } else if *text == '{' as i32 {
                    bracket_indenting = indenting;
                    continue ;
                }
            }
            if *text == 0 as c_int {
                current_block = 1425453989644512380;
                break ;
            }
            let mut key: *mut wchar_t = 0 as *mut wchar_t;
            let mut name: *mut wchar_t = 0 as *mut wchar_t;
            let mut cls: *mut wchar_t = 0 as *mut wchar_t;
            let mut value: *mut wchar_t = 0 as *mut wchar_t;
            if indenting < 0 as c_int {
                current_block = 6712573431332408062;
                break ;
            }
            if *text == '<' as i32 {
                text = text.offset(1);
                let filename_len: c_int =
                    wcscspn(text,
                            (*::std::mem::transmute::<&[u8; 8],
                                                      &[c_int; 2]>(b">\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                        as c_int;
                let vla = (filename_len + 1 as c_int) as usize;
                let mut wfn: Vec<wchar_t> = ::std::vec::from_elem(0, vla);
                wmemcpy(wfn.as_mut_ptr(), text,
                        (filename_len + 1 as c_int) as size_t);
                *wfn.as_mut_ptr().offset(filename_len as isize) =
                    0 as c_int;
                let len: size_t =
                    wcstombs(0 as *mut c_char, wfn.as_mut_ptr(),
                             0 as c_int as
                                 size_t).wrapping_add(1 as c_int as
                                                          c_ulong);
                let vla_0 = len as usize;
                let mut filename: Vec<c_char> =
                    ::std::vec::from_elem(0, vla_0);
                let rc: size_t =
                    wcstombs(filename.as_mut_ptr(), wfn.as_mut_ptr(), len);
                if rc != -(1 as c_int) as size_t {
                } else {
                    __assert_fail(b"rc != (size_t)-1\x00" as *const u8 as
                                      *const c_char,
                                  b"parser.c\x00" as *const u8 as
                                      *const c_char,
                                  319 as c_int as c_uint,
                                  (*::std::mem::transmute::<&[u8; 49],
                                                            &[c_char; 49]>(b"struct stfl_widget *stfl_parser(const wchar_t *)\x00")).as_ptr());
                }
                text = text.offset(filename_len as isize);
                if *text != 0 { text = text.offset(1) }
                let mut n: *mut stfl_widget =
                    stfl_parser_file(filename.as_mut_ptr());
                if n.is_null() { return 0 as *mut stfl_widget }
                if !root.is_null() {
                    while (*current).parser_indent >= indenting {
                        current = (*current).parent;
                        if current.is_null() {
                            current_block = 6712573431332408062;
                            break 's_17 ;
                        }
                    }
                    (*n).parent = current;
                    if !(*current).last_child.is_null() {
                        (*(*current).last_child).next_sibling = n;
                        (*current).last_child = n
                    } else {
                        (*current).first_child = n;
                        (*current).last_child = n
                    }
                    (*n).parser_indent = indenting;
                    current = n
                } else { root = n }
            } else if !root.is_null() {
                while (*current).parser_indent >= indenting {
                    current = (*current).parent;
                    if current.is_null() {
                        current_block = 6712573431332408062;
                        break 's_17 ;
                    }
                }
                if read_type(&mut text, &mut key, &mut name, &mut cls) ==
                       1 as c_int {
                    let mut n_0: *mut stfl_widget = stfl_widget_new(key);
                    if n_0.is_null() {
                        current_block = 6712573431332408062;
                        break ;
                    }
                    free(key as *mut c_void);
                    (*n_0).parent = current;
                    if !(*current).last_child.is_null() {
                        (*(*current).last_child).next_sibling = n_0;
                        (*current).last_child = n_0
                    } else {
                        (*current).first_child = n_0;
                        (*current).last_child = n_0
                    }
                    (*n_0).parser_indent = indenting;
                    (*n_0).name = unquote(name, -(1 as c_int));
                    free(name as *mut c_void);
                    (*n_0).cls = cls;
                    current = n_0
                } else {
                    if !(read_kv(&mut text, &mut key, &mut name, &mut value)
                             == 1 as c_int) {
                        current_block = 6712573431332408062;
                        break ;
                    }
                    let mut kv: *mut stfl_kv =
                        stfl_widget_setkv_str(current, key, value);
                    if !(*kv).name.is_null() {
                        free((*kv).name as *mut c_void);
                    }
                    (*kv).name = unquote(name, -(1 as c_int));
                    free(name as *mut c_void);
                    free(key as *mut c_void);
                    free(value as *mut c_void);
                }
            } else {
                if read_type(&mut text, &mut key, &mut name, &mut cls) ==
                       0 as c_int {
                    current_block = 6712573431332408062;
                    break ;
                }
                let mut n_1: *mut stfl_widget = stfl_widget_new(key);
                if n_1.is_null() {
                    current_block = 6712573431332408062;
                    break ;
                }
                free(key as *mut c_void);
                root = n_1;
                current = n_1;
                (*n_1).name = unquote(name, -(1 as c_int));
                free(name as *mut c_void);
                (*n_1).cls = cls
            }
            while *text != 0 && *text != '\n' as i32 && *text != '\r' as i32
                      && *text != '{' as i32 && *text != '}' as i32 {
                while *text == ' ' as i32 || *text == '\t' as i32 {
                    text = text.offset(1)
                }
                if !(*text != 0 && *text != '\n' as i32 &&
                         *text != '\r' as i32 && *text != '{' as i32 &&
                         *text != '}' as i32) {
                    continue ;
                }
                if read_kv(&mut text, &mut key, &mut name, &mut value) ==
                       0 as c_int {
                    current_block = 6712573431332408062;
                    break 's_17 ;
                }
                let mut kv_0: *mut stfl_kv =
                    stfl_widget_setkv_str(current, key, value);
                if !(*kv_0).name.is_null() {
                    free((*kv_0).name as *mut c_void);
                }
                (*kv_0).name = unquote(name, -(1 as c_int));
                free(name as *mut c_void);
                free(key as *mut c_void);
                free(value as *mut c_void);
            }
        }
    match current_block {
        1425453989644512380 => { if !root.is_null() { return root } }
        _ => { }
    }
    fprintf(stderr,
            b"STFL Parser Error near \'\x00" as *const u8 as
                *const c_char);
    let mut i = 0 as c_int;
    while *text != 0 && i < 20 as c_int {
        if *text == '\n' as i32 {
            fprintf(stderr, b"\\n\x00" as *const u8 as *const c_char);
        } else if *text == '\t' as i32 {
            fprintf(stderr, b" \x00" as *const u8 as *const c_char);
        } else if *text < 32 as c_int {
            fprintf(stderr,
                    b"\\%03lo\x00" as *const u8 as *const c_char,
                    *text as c_ulong);
        } else {
            fprintf(stderr, b"%lc\x00" as *const u8 as *const c_char,
                    *text as wint_t);
        }
        i += 1;
        text = text.offset(1)
    }
    fprintf(stderr, b"\'.\r\n\x00" as *const u8 as *const c_char);
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn stfl_parser_file(filename: *const c_char)
 -> *mut stfl_widget {
    let f: *mut FILE =
        fopen(filename, b"r\x00" as *const u8 as *const c_char);
    if f.is_null() {
        fprintf(stderr,
                b"STFL Parser Error: Can\'t read file \'%s\'!\n\x00" as
                    *const u8 as *const c_char, filename);
        abort();
    }
    let mut len: c_int = 0 as c_int;
    let mut text: *mut c_char = 0 as *mut c_char;
    loop  {
        let mut pos: c_int = len;
        len += 4096 as c_int;
        text =
            realloc(text as *mut c_void, len as c_ulong) as
                *mut c_char;
        pos =
            (pos as
                 c_ulong).wrapping_add(fread(text.offset(pos as isize)
                                                       as *mut c_void,
                                                   1 as c_int as size_t,
                                                   4096 as c_int as
                                                       size_t, f)) as
                c_int as c_int;
        if !(pos < len) { continue ; }
        *text.offset(pos as isize) = 0 as c_int as c_char;
        fclose(f);
        break ;
    }
    let mut text1: *const c_char = text;
    let wtextsize: size_t =
        mbsrtowcs(0 as *mut wchar_t, &mut text1,
                  strlen(text1).wrapping_add(1 as c_int as
                                                 c_ulong),
                  0 as
                      *mut mbstate_t).wrapping_add(1 as c_int as
                                                       c_ulong);
    let wtext: *mut wchar_t =
        malloc((::std::mem::size_of::<wchar_t>() as
                    c_ulong).wrapping_mul(wtextsize)) as *mut wchar_t;
    let rc: size_t = mbstowcs(wtext, text, wtextsize);
    if rc != -(1 as c_int) as size_t {
    } else {
        __assert_fail(b"rc != (size_t)-1\x00" as *const u8 as
                          *const c_char,
                      b"parser.c\x00" as *const u8 as *const c_char,
                      490 as c_int as c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[c_char; 51]>(b"struct stfl_widget *stfl_parser_file(const char *)\x00")).as_ptr());
    }
    let w: *mut stfl_widget = stfl_parser(wtext);
    free(text as *mut c_void);
    free(wtext as *mut c_void);
    return w;
}
