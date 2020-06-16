use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ldat;
    #[no_mangle]
    fn init_pair(_: libc::c_short, _: libc::c_short, _: libc::c_short)
     -> libc::c_int;
    #[no_mangle]
    fn pair_content(_: libc::c_short, _: *mut libc::c_short,
                    _: *mut libc::c_short) -> libc::c_int;
    #[no_mangle]
    fn wattrset(_: *mut WINDOW, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn wcolor_set(_: *mut WINDOW, _: libc::c_short, _: *mut libc::c_void)
     -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn wcstoul(__nptr: *const wchar_t, __endptr: *mut *mut wchar_t,
               __base: libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn wmemcpy(__s1: *mut wchar_t, __s2: *const wchar_t, __n: size_t)
     -> *mut wchar_t;
    #[no_mangle]
    fn wcsspn(__wcs: *const wchar_t, __accept: *const wchar_t) -> size_t;
    #[no_mangle]
    fn wcscspn(__wcs: *const wchar_t, __reject: *const wchar_t) -> size_t;
    #[no_mangle]
    fn wcsncmp(_: *const libc::c_int, _: *const libc::c_int, _: libc::c_ulong)
     -> libc::c_int;
    #[no_mangle]
    fn wcscmp(_: *const libc::c_int, _: *const libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut COLORS: libc::c_int;
    #[no_mangle]
    static mut COLOR_PAIRS: libc::c_int;
    #[no_mangle]
    fn stfl_widget_getkv_str(w: *mut stfl_widget, key: *const wchar_t,
                             defval: *const wchar_t) -> *const wchar_t;
    #[no_mangle]
    fn abort() -> !;
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stfl_form {
    pub root: *mut stfl_widget,
    pub current_focus_id: libc::c_int,
    pub cursor_x: libc::c_int,
    pub cursor_y: libc::c_int,
    pub event_queue: *mut stfl_event,
    pub event: *mut wchar_t,
    pub mtx: pthread_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stfl_event {
    pub next: *mut stfl_event,
    pub event: *mut wchar_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stfl_widget {
    pub parent: *mut stfl_widget,
    pub next_sibling: *mut stfl_widget,
    pub first_child: *mut stfl_widget,
    pub last_child: *mut stfl_widget,
    pub kv_list: *mut stfl_kv,
    pub type_0: *mut stfl_widget_type,
    pub id: libc::c_int,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub min_w: libc::c_int,
    pub min_h: libc::c_int,
    pub cur_x: libc::c_int,
    pub cur_y: libc::c_int,
    pub parser_indent: libc::c_int,
    pub allow_focus: libc::c_int,
    pub setfocus: libc::c_int,
    pub internal_data: *mut libc::c_void,
    pub name: *mut wchar_t,
    pub cls: *mut wchar_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stfl_widget_type {
    pub name: *mut wchar_t,
    pub f_init: Option<unsafe extern "C" fn(_: *mut stfl_widget) -> ()>,
    pub f_done: Option<unsafe extern "C" fn(_: *mut stfl_widget) -> ()>,
    pub f_enter: Option<unsafe extern "C" fn(_: *mut stfl_widget,
                                             _: *mut stfl_form) -> ()>,
    pub f_leave: Option<unsafe extern "C" fn(_: *mut stfl_widget,
                                             _: *mut stfl_form) -> ()>,
    pub f_prepare: Option<unsafe extern "C" fn(_: *mut stfl_widget,
                                               _: *mut stfl_form) -> ()>,
    pub f_draw: Option<unsafe extern "C" fn(_: *mut stfl_widget,
                                            _: *mut stfl_form, _: *mut WINDOW)
                           -> ()>,
    pub f_process: Option<unsafe extern "C" fn(_: *mut stfl_widget,
                                               _: *mut stfl_widget,
                                               _: *mut stfl_form, _: wchar_t,
                                               _: libc::c_int)
                              -> libc::c_int>,
}
pub type WINDOW = _win_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _win_st {
    pub _cury: libc::c_short,
    pub _curx: libc::c_short,
    pub _maxy: libc::c_short,
    pub _maxx: libc::c_short,
    pub _begy: libc::c_short,
    pub _begx: libc::c_short,
    pub _flags: libc::c_short,
    pub _attrs: attr_t,
    pub _bkgd: chtype,
    pub _notimeout: bool,
    pub _clear: bool,
    pub _leaveok: bool,
    pub _scroll: bool,
    pub _idlok: bool,
    pub _idcok: bool,
    pub _immed: bool,
    pub _sync: bool,
    pub _use_keypad: bool,
    pub _delay: libc::c_int,
    pub _line: *mut ldat,
    pub _regtop: libc::c_short,
    pub _regbottom: libc::c_short,
    pub _parx: libc::c_int,
    pub _pary: libc::c_int,
    pub _parent: *mut WINDOW,
    pub _pad: pdat,
    pub _yoffset: libc::c_short,
    pub _bkgrnd: cchar_t,
    pub _color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cchar_t {
    pub attr: attr_t,
    pub chars: [wchar_t; 5],
    pub ext_color: libc::c_int,
}
pub type attr_t = chtype;
pub type chtype = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdat {
    pub _pad_y: libc::c_short,
    pub _pad_x: libc::c_short,
    pub _pad_top: libc::c_short,
    pub _pad_left: libc::c_short,
    pub _pad_bottom: libc::c_short,
    pub _pad_right: libc::c_short,
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
    pub id: libc::c_int,
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
 *  style.c: Helper functions for text colors and attributes
 */
unsafe extern "C" fn wcssep(mut stringp: *mut *mut wchar_t,
                            mut delim: *const wchar_t) -> *mut wchar_t {
    let mut tmp: *mut wchar_t = *stringp;
    let mut tmp2: *mut wchar_t = tmp;
    let mut tmp3: *const wchar_t = 0 as *const wchar_t;
    if (*stringp).is_null() { return 0 as *mut wchar_t }
    tmp2 = tmp;
    while *tmp2 != 0 {
        tmp3 = delim;
        while *tmp3 != 0 {
            if *tmp2 == *tmp3 {
                /* delimiter found */
                *tmp2 = 0 as libc::c_int;
                *stringp = tmp2.offset(1 as libc::c_int as isize);
                return tmp
            }
            tmp3 = tmp3.offset(1)
        }
        tmp2 = tmp2.offset(1)
    }
    *stringp = 0 as *mut wchar_t;
    return tmp;
}
static mut stfl_colorpair_bg: [libc::c_int; 256] = [0; 256];
static mut stfl_colorpair_fg: [libc::c_int; 256] = [0; 256];
#[no_mangle]
pub static mut stfl_colorpair_counter: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn stfl_style(mut win: *mut WINDOW,
                                    mut style: *const wchar_t) {
    let mut bg_color: libc::c_int = -(1 as libc::c_int);
    let mut fg_color: libc::c_int = -(1 as libc::c_int);
    let mut attr: libc::c_int =
        (1 as libc::c_uint).wrapping_sub(1 as libc::c_uint) as libc::c_int;
    style =
        style.offset(wcsspn(style,
                            (*::std::mem::transmute::<&[u8; 12],
                                                      &[libc::c_int; 3]>(b" \x00\x00\x00\t\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                         as isize);
    while *style != 0 {
        let mut field_len: libc::c_int =
            wcscspn(style,
                    (*::std::mem::transmute::<&[u8; 8],
                                              &[libc::c_int; 2]>(b",\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                as libc::c_int;
        let vla = (field_len + 1 as libc::c_int) as usize;
        let mut field: Vec<wchar_t> = ::std::vec::from_elem(0, vla);
        wmemcpy(field.as_mut_ptr(), style, field_len as size_t);
        *field.as_mut_ptr().offset(field_len as isize) = 0 as libc::c_int;
        style = style.offset(field_len as isize);
        if *style == ',' as i32 { style = style.offset(1) }
        let mut sepp: *mut wchar_t = field.as_mut_ptr();
        let mut key: *mut wchar_t =
            wcssep(&mut sepp,
                   (*::std::mem::transmute::<&[u8; 8],
                                             &[libc::c_int; 2]>(b"=\x00\x00\x00\x00\x00\x00\x00")).as_ptr());
        let mut value: *mut wchar_t =
            wcssep(&mut sepp,
                   (*::std::mem::transmute::<&[u8; 4],
                                             &[libc::c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr());
        if key.is_null() || value.is_null() { continue ; }
        key =
            key.offset(wcsspn(key,
                              (*::std::mem::transmute::<&[u8; 12],
                                                        &[libc::c_int; 3]>(b" \x00\x00\x00\t\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                           as isize);
        key =
            wcssep(&mut key,
                   (*::std::mem::transmute::<&[u8; 12],
                                             &[libc::c_int; 3]>(b" \x00\x00\x00\t\x00\x00\x00\x00\x00\x00\x00")).as_ptr());
        value =
            value.offset(wcsspn(value,
                                (*::std::mem::transmute::<&[u8; 12],
                                                          &[libc::c_int; 3]>(b" \x00\x00\x00\t\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                             as isize);
        value =
            wcssep(&mut value,
                   (*::std::mem::transmute::<&[u8; 12],
                                             &[libc::c_int; 3]>(b" \x00\x00\x00\t\x00\x00\x00\x00\x00\x00\x00")).as_ptr());
        if wcscmp(key,
                  (*::std::mem::transmute::<&[u8; 12],
                                            &[libc::c_int; 3]>(b"b\x00\x00\x00g\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
               == 0 ||
               wcscmp(key,
                      (*::std::mem::transmute::<&[u8; 12],
                                                &[libc::c_int; 3]>(b"f\x00\x00\x00g\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                   == 0 {
            let mut color: libc::c_int = -(1 as libc::c_int);
            if wcscmp(value,
                      (*::std::mem::transmute::<&[u8; 24],
                                                &[libc::c_int; 6]>(b"b\x00\x00\x00l\x00\x00\x00a\x00\x00\x00c\x00\x00\x00k\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                   == 0 {
                color = 0 as libc::c_int
            } else if wcscmp(value,
                             (*::std::mem::transmute::<&[u8; 16],
                                                       &[libc::c_int; 4]>(b"r\x00\x00\x00e\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                          == 0 {
                color = 1 as libc::c_int
            } else if wcscmp(value,
                             (*::std::mem::transmute::<&[u8; 24],
                                                       &[libc::c_int; 6]>(b"g\x00\x00\x00r\x00\x00\x00e\x00\x00\x00e\x00\x00\x00n\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                          == 0 {
                color = 2 as libc::c_int
            } else if wcscmp(value,
                             (*::std::mem::transmute::<&[u8; 28],
                                                       &[libc::c_int; 7]>(b"y\x00\x00\x00e\x00\x00\x00l\x00\x00\x00l\x00\x00\x00o\x00\x00\x00w\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                          == 0 {
                color = 3 as libc::c_int
            } else if wcscmp(value,
                             (*::std::mem::transmute::<&[u8; 20],
                                                       &[libc::c_int; 5]>(b"b\x00\x00\x00l\x00\x00\x00u\x00\x00\x00e\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                          == 0 {
                color = 4 as libc::c_int
            } else if wcscmp(value,
                             (*::std::mem::transmute::<&[u8; 32],
                                                       &[libc::c_int; 8]>(b"m\x00\x00\x00a\x00\x00\x00g\x00\x00\x00e\x00\x00\x00n\x00\x00\x00t\x00\x00\x00a\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                          == 0 {
                color = 5 as libc::c_int
            } else if wcscmp(value,
                             (*::std::mem::transmute::<&[u8; 20],
                                                       &[libc::c_int; 5]>(b"c\x00\x00\x00y\x00\x00\x00a\x00\x00\x00n\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                          == 0 {
                color = 6 as libc::c_int
            } else if wcscmp(value,
                             (*::std::mem::transmute::<&[u8; 24],
                                                       &[libc::c_int; 6]>(b"w\x00\x00\x00h\x00\x00\x00i\x00\x00\x00t\x00\x00\x00e\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                          == 0 {
                color = 7 as libc::c_int
            } else if wcsncmp(value,
                              (*::std::mem::transmute::<&[u8; 24],
                                                        &[libc::c_int; 6]>(b"c\x00\x00\x00o\x00\x00\x00l\x00\x00\x00o\x00\x00\x00r\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                              5 as libc::c_int as libc::c_ulong) == 0 {
                color =
                    wcstoul(value.offset(5 as libc::c_int as isize),
                            0 as *mut *mut wchar_t, 0 as libc::c_int) as
                        libc::c_int
            } else {
                fprintf(stderr,
                        b"STFL Style Error: Unknown %ls color: \'%ls\'\n\x00"
                            as *const u8 as *const libc::c_char, key, value);
                abort();
            }
            if wcscmp(key,
                      (*::std::mem::transmute::<&[u8; 12],
                                                &[libc::c_int; 3]>(b"b\x00\x00\x00g\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                   == 0 {
                bg_color = color
            } else { fg_color = color }
        } else if wcscmp(key,
                         (*::std::mem::transmute::<&[u8; 20],
                                                   &[libc::c_int; 5]>(b"a\x00\x00\x00t\x00\x00\x00t\x00\x00\x00r\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                      == 0 {
            if wcscmp(value,
                      (*::std::mem::transmute::<&[u8; 36],
                                                &[libc::c_int; 9]>(b"s\x00\x00\x00t\x00\x00\x00a\x00\x00\x00n\x00\x00\x00d\x00\x00\x00o\x00\x00\x00u\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                   == 0 {
                attr =
                    (attr as libc::c_uint |
                         (1 as libc::c_uint) <<
                             8 as libc::c_int + 8 as libc::c_int) as
                        libc::c_int
            } else if wcscmp(value,
                             (*::std::mem::transmute::<&[u8; 40],
                                                       &[libc::c_int; 10]>(b"u\x00\x00\x00n\x00\x00\x00d\x00\x00\x00e\x00\x00\x00r\x00\x00\x00l\x00\x00\x00i\x00\x00\x00n\x00\x00\x00e\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                          == 0 {
                attr =
                    (attr as libc::c_uint |
                         (1 as libc::c_uint) <<
                             9 as libc::c_int + 8 as libc::c_int) as
                        libc::c_int
            } else if wcscmp(value,
                             (*::std::mem::transmute::<&[u8; 32],
                                                       &[libc::c_int; 8]>(b"r\x00\x00\x00e\x00\x00\x00v\x00\x00\x00e\x00\x00\x00r\x00\x00\x00s\x00\x00\x00e\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                          == 0 {
                attr =
                    (attr as libc::c_uint |
                         (1 as libc::c_uint) <<
                             10 as libc::c_int + 8 as libc::c_int) as
                        libc::c_int
            } else if wcscmp(value,
                             (*::std::mem::transmute::<&[u8; 24],
                                                       &[libc::c_int; 6]>(b"b\x00\x00\x00l\x00\x00\x00i\x00\x00\x00n\x00\x00\x00k\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                          == 0 {
                attr =
                    (attr as libc::c_uint |
                         (1 as libc::c_uint) <<
                             11 as libc::c_int + 8 as libc::c_int) as
                        libc::c_int
            } else if wcscmp(value,
                             (*::std::mem::transmute::<&[u8; 16],
                                                       &[libc::c_int; 4]>(b"d\x00\x00\x00i\x00\x00\x00m\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                          == 0 {
                attr =
                    (attr as libc::c_uint |
                         (1 as libc::c_uint) <<
                             12 as libc::c_int + 8 as libc::c_int) as
                        libc::c_int
            } else if wcscmp(value,
                             (*::std::mem::transmute::<&[u8; 20],
                                                       &[libc::c_int; 5]>(b"b\x00\x00\x00o\x00\x00\x00l\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                          == 0 {
                attr =
                    (attr as libc::c_uint |
                         (1 as libc::c_uint) <<
                             13 as libc::c_int + 8 as libc::c_int) as
                        libc::c_int
            } else if wcscmp(value,
                             (*::std::mem::transmute::<&[u8; 32],
                                                       &[libc::c_int; 8]>(b"p\x00\x00\x00r\x00\x00\x00o\x00\x00\x00t\x00\x00\x00e\x00\x00\x00c\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                          == 0 {
                attr =
                    (attr as libc::c_uint |
                         (1 as libc::c_uint) <<
                             16 as libc::c_int + 8 as libc::c_int) as
                        libc::c_int
            } else if wcscmp(value,
                             (*::std::mem::transmute::<&[u8; 24],
                                                       &[libc::c_int; 6]>(b"i\x00\x00\x00n\x00\x00\x00v\x00\x00\x00i\x00\x00\x00s\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                          == 0 {
                attr =
                    (attr as libc::c_uint |
                         (1 as libc::c_uint) <<
                             15 as libc::c_int + 8 as libc::c_int) as
                        libc::c_int
            } else {
                fprintf(stderr,
                        b"STFL Style Error: Unknown attribute: \'%ls\'\n\x00"
                            as *const u8 as *const libc::c_char, value);
                abort();
            }
        } else {
            fprintf(stderr,
                    b"STFL Style Error: Unknown keyword: \'%ls\'\n\x00" as
                        *const u8 as *const libc::c_char, key);
            abort();
        }
    }
    let mut f: libc::c_short = 0;
    let mut b: libc::c_short = 0;
    pair_content(0 as libc::c_int as libc::c_short, &mut f, &mut b);
    if fg_color < 0 as libc::c_int || fg_color >= COLORS {
        fg_color = f as libc::c_int
    }
    if bg_color < 0 as libc::c_int || bg_color >= COLORS {
        bg_color = b as libc::c_int
    }
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < stfl_colorpair_counter {
        if stfl_colorpair_fg[i as usize] == fg_color &&
               stfl_colorpair_bg[i as usize] == bg_color {
            break ;
        }
        i += 1
    }
    if i == stfl_colorpair_counter {
        if i == COLOR_PAIRS {
            fprintf(stderr,
                    b"Ncurses limit of color pairs (%d) reached!\n\x00" as
                        *const u8 as *const libc::c_char, COLOR_PAIRS);
            abort();
        }
        if i == 256 as libc::c_int {
            fprintf(stderr,
                    b"Internal STFL limit of color pairs (%d) reached!\n\x00"
                        as *const u8 as *const libc::c_char,
                    256 as libc::c_int);
            abort();
        }
        init_pair(i as libc::c_short, fg_color as libc::c_short,
                  bg_color as libc::c_short);
        stfl_colorpair_fg[i as usize] = fg_color;
        stfl_colorpair_bg[i as usize] = bg_color;
        stfl_colorpair_counter += 1
    }
    wattrset(win, attr);
    wcolor_set(win, i as libc::c_short, 0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn stfl_widget_style(mut w: *mut stfl_widget,
                                           mut f: *mut stfl_form,
                                           mut win: *mut WINDOW) {
    let mut style: *const wchar_t =
        (*::std::mem::transmute::<&[u8; 4],
                                  &[libc::c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr();
    if (*f).current_focus_id == (*w).id {
        style =
            stfl_widget_getkv_str(w,
                                  (*::std::mem::transmute::<&[u8; 48],
                                                            &[libc::c_int; 12]>(b"s\x00\x00\x00t\x00\x00\x00y\x00\x00\x00l\x00\x00\x00e\x00\x00\x00_\x00\x00\x00f\x00\x00\x00o\x00\x00\x00c\x00\x00\x00u\x00\x00\x00s\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                  (*::std::mem::transmute::<&[u8; 4],
                                                            &[libc::c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr())
    }
    if *style == 0 as libc::c_int {
        style =
            stfl_widget_getkv_str(w,
                                  (*::std::mem::transmute::<&[u8; 52],
                                                            &[libc::c_int; 13]>(b"s\x00\x00\x00t\x00\x00\x00y\x00\x00\x00l\x00\x00\x00e\x00\x00\x00_\x00\x00\x00n\x00\x00\x00o\x00\x00\x00r\x00\x00\x00m\x00\x00\x00a\x00\x00\x00l\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                  (*::std::mem::transmute::<&[u8; 4],
                                                            &[libc::c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr())
    }
    stfl_style(win, style);
}
