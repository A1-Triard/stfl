use ::libc;
use crate::public::*;

extern "C" {
    pub type ldat;
    #[no_mangle]
    fn keyname(_: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn swprintf(__s: *mut wchar_t, __n: size_t, __format: *const wchar_t,
                _: ...) -> libc::c_int;
    #[no_mangle]
    fn wcslen(_: *const libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn wcsspn(__wcs: *const wchar_t, __accept: *const wchar_t) -> size_t;
    #[no_mangle]
    fn wcscspn(__wcs: *const wchar_t, __reject: *const wchar_t) -> size_t;
    #[no_mangle]
    fn wcsncmp(_: *const libc::c_int, _: *const libc::c_int, _: libc::c_ulong)
     -> libc::c_int;
    #[no_mangle]
    fn stfl_widget_getkv_int(w: *mut stfl_widget, key: *const wchar_t,
                             defval: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn stfl_widget_getkv_str(w: *mut stfl_widget, key: *const wchar_t,
                             defval: *const wchar_t) -> *const wchar_t;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
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
        wcslen(src).wrapping_add(1 as libc::c_int as
                                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<wchar_t>()
                                                                     as
                                                                     libc::c_ulong);
    let dest: *mut wchar_t = malloc(n) as *mut wchar_t;
    memcpy(dest as *mut libc::c_void, src as *const libc::c_void, n);
    return dest;
}
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
 *  binding.c: Helper functions for key bindings and stuff
 */
#[no_mangle]
pub unsafe extern "C" fn stfl_keyname(ch: wchar_t,
                                      isfunckey: libc::c_int)
 -> *mut wchar_t {
    if isfunckey == 0 {
        if ch == '\r' as i32 || ch == '\n' as i32 {
            return compat_wcsdup((*::std::mem::transmute::<&[u8; 24],
                                                           &[libc::c_int; 6]>(b"E\x00\x00\x00N\x00\x00\x00T\x00\x00\x00E\x00\x00\x00R\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
        }
        if ch == ' ' as i32 {
            return compat_wcsdup((*::std::mem::transmute::<&[u8; 24],
                                                           &[libc::c_int; 6]>(b"S\x00\x00\x00P\x00\x00\x00A\x00\x00\x00C\x00\x00\x00E\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
        }
        if ch == '\t' as i32 {
            return compat_wcsdup((*::std::mem::transmute::<&[u8; 16],
                                                           &[libc::c_int; 4]>(b"T\x00\x00\x00A\x00\x00\x00B\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
        }
        if ch == 27 as libc::c_int {
            return compat_wcsdup((*::std::mem::transmute::<&[u8; 16],
                                                           &[libc::c_int; 4]>(b"E\x00\x00\x00S\x00\x00\x00C\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
        }
        if ch == 127 as libc::c_int {
            return compat_wcsdup((*::std::mem::transmute::<&[u8; 40],
                                                           &[libc::c_int; 10]>(b"B\x00\x00\x00A\x00\x00\x00C\x00\x00\x00K\x00\x00\x00S\x00\x00\x00P\x00\x00\x00A\x00\x00\x00C\x00\x00\x00E\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
        }
        let ret;
        if ch < 32 as libc::c_int {
            let key: *const libc::c_char = keyname(ch);
            let keylen: libc::c_uint =
                strlen(key).wrapping_add(1 as libc::c_int as libc::c_ulong) as
                    libc::c_uint;
            ret =
                malloc((keylen as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<wchar_t>()
                                                            as libc::c_ulong))
                    as *mut wchar_t;
            let mut i = 0 as libc::c_int as libc::c_uint;
            while i < keylen {
                *ret.offset(i as isize) = *key.offset(i as isize) as wchar_t;
                i = i.wrapping_add(1)
            }
        } else {
            ret =
                compat_wcsdup((*::std::mem::transmute::<&[u8; 8],
                                                        &[libc::c_int; 2]>(b" \x00\x00\x00\x00\x00\x00\x00")).as_ptr());
            *ret.offset(0 as libc::c_int as isize) = ch
        }
        return ret
    }
    if 0o410 as libc::c_int + 0 as libc::c_int <= ch &&
           ch <= 0o410 as libc::c_int + 63 as libc::c_int {
        let name: *mut wchar_t =
            malloc((4 as libc::c_int as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<wchar_t>()
                                                        as libc::c_ulong)) as
                *mut wchar_t;
        swprintf(name, 4 as libc::c_int as size_t,
                 (*::std::mem::transmute::<&[u8; 16],
                                           &[libc::c_int; 4]>(b"F\x00\x00\x00%\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                 ch - 0o410 as libc::c_int);
        return name
    }
    let mut event_c: *const libc::c_char = keyname(ch);
    if event_c.is_null() {
        return compat_wcsdup((*::std::mem::transmute::<&[u8; 32],
                                                       &[libc::c_int; 8]>(b"U\x00\x00\x00N\x00\x00\x00K\x00\x00\x00N\x00\x00\x00O\x00\x00\x00W\x00\x00\x00N\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
    }
    if strncmp(event_c, b"KEY_\x00" as *const u8 as *const libc::c_char,
               4 as libc::c_int as libc::c_ulong) == 0 {
        event_c = event_c.offset(4 as libc::c_int as isize)
    }
    let event_len: libc::c_int =
        strlen(event_c).wrapping_add(1 as libc::c_int as libc::c_ulong) as
            libc::c_int;
    let event: *mut wchar_t =
        malloc((event_len as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<wchar_t>()
                                                    as libc::c_ulong)) as
            *mut wchar_t;
    let mut i_0 = 0 as libc::c_int;
    while i_0 < event_len {
        *event.offset(i_0 as isize) =
            *event_c.offset(i_0 as isize) as wchar_t;
        i_0 += 1
    }
    return event;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_matchbind(w: *mut stfl_widget,
                                        ch: wchar_t,
                                        isfunckey: libc::c_int,
                                        name: *mut wchar_t,
                                        mut auto_desc: *mut wchar_t)
 -> libc::c_int {
    let event: *mut wchar_t = stfl_keyname(ch, isfunckey);
    let event_len: libc::c_int = wcslen(event) as libc::c_int;
    let kvname_len: libc::c_int =
        wcslen(name).wrapping_add(6 as libc::c_int as libc::c_ulong) as
            libc::c_int;
    let vla = kvname_len as usize;
    let mut kvname: Vec<wchar_t> = ::std::vec::from_elem(0, vla);
    swprintf(kvname.as_mut_ptr(), kvname_len as size_t,
             (*::std::mem::transmute::<&[u8; 36],
                                       &[libc::c_int; 9]>(b"b\x00\x00\x00i\x00\x00\x00n\x00\x00\x00d\x00\x00\x00_\x00\x00\x00%\x00\x00\x00l\x00\x00\x00s\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
             name);
    if stfl_widget_getkv_int(w,
                             (*::std::mem::transmute::<&[u8; 36],
                                                       &[libc::c_int; 9]>(b"a\x00\x00\x00u\x00\x00\x00t\x00\x00\x00o\x00\x00\x00b\x00\x00\x00i\x00\x00\x00n\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                             1 as libc::c_int) == 0 as libc::c_int {
        auto_desc =
            (*::std::mem::transmute::<&[u8; 4],
                                      &[libc::c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr()
                as *mut wchar_t
    }
    let mut desc: *const wchar_t =
        stfl_widget_getkv_str(w, kvname.as_mut_ptr(), auto_desc);
    let mut retry_auto_desc: libc::c_int = 0 as libc::c_int;
    loop  {
        while *desc != 0 {
            desc =
                desc.offset(wcsspn(desc,
                                   (*::std::mem::transmute::<&[u8; 20],
                                                             &[libc::c_int; 5]>(b" \x00\x00\x00\t\x00\x00\x00\n\x00\x00\x00\r\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                                as isize);
            let len: libc::c_int =
                wcscspn(desc,
                        (*::std::mem::transmute::<&[u8; 20],
                                                  &[libc::c_int; 5]>(b" \x00\x00\x00\t\x00\x00\x00\n\x00\x00\x00\r\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                    as libc::c_int;
            if retry_auto_desc == 0 && len == 2 as libc::c_int &&
                   wcsncmp(desc,
                           (*::std::mem::transmute::<&[u8; 12],
                                                     &[libc::c_int; 3]>(b"*\x00\x00\x00*\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                           2 as libc::c_int as libc::c_ulong) == 0 {
                retry_auto_desc = 1 as libc::c_int
            }
            if len > 0 as libc::c_int && len == event_len &&
                   wcsncmp(desc, event, len as libc::c_ulong) == 0 {
                free(event as *mut libc::c_void);
                return 1 as libc::c_int
            }
            desc = desc.offset(len as isize)
        }
        if !(retry_auto_desc > 0 as libc::c_int) { break ; }
        retry_auto_desc = -(1 as libc::c_int);
        desc = auto_desc
    }
    free(event as *mut libc::c_void);
    return 0 as libc::c_int;
}
