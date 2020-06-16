use ::libc;
extern "C" {
    pub type ldat;
    #[no_mangle]
    fn vswprintf(__s: *mut wchar_t, __n: size_t, __format: *const wchar_t,
                 __arg: ::std::ffi::VaList) -> libc::c_int;
    #[no_mangle]
    fn wmemcpy(__s1: *mut wchar_t, __s2: *const wchar_t, __n: size_t)
     -> *mut wchar_t;
    #[no_mangle]
    fn wcslen(_: *const libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn wcscspn(__wcs: *const wchar_t, __reject: *const wchar_t) -> size_t;
    #[no_mangle]
    fn wcscmp(_: *const libc::c_int, _: *const libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type va_list = __builtin_va_list;
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
 *  dump.c: Create STFL code from a widget tree
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct txtnode {
    pub prev: *mut txtnode,
    pub value: *mut wchar_t,
    pub len: libc::c_int,
}
unsafe extern "C" fn newtxt(mut o: *mut *mut txtnode, mut fmt: *const wchar_t,
                            mut args: ...) 
 /* __attribute__ ((format (wprintf, 2, 3))) */
 {
    let mut n: *mut txtnode =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<txtnode>() as libc::c_ulong) as
            *mut txtnode;
    (*n).prev = *o;
    *o = n;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    let mut buf: *mut wchar_t =
        malloc((4096 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<wchar_t>()
                                                    as libc::c_ulong)) as
            *mut wchar_t;
    let mut buf_len: libc::c_int = 4096 as libc::c_int;
    loop  {
        let mut rc: libc::c_int =
            vswprintf(buf, buf_len as size_t, fmt, ap.as_va_list());
        if rc < 0 as libc::c_int {
            free(buf as *mut libc::c_void);
            buf = 0 as *mut wchar_t;
            break ;
        } else if (rc + 1 as libc::c_int) < buf_len {
            buf =
                realloc(buf as *mut libc::c_void,
                        ((rc + 1 as libc::c_int) as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<wchar_t>()
                                                             as
                                                             libc::c_ulong))
                    as *mut wchar_t;
            break ;
        } else {
            buf_len = buf_len * 2 as libc::c_int;
            buf =
                realloc(buf as *mut libc::c_void,
                        (buf_len as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<wchar_t>()
                                                             as
                                                             libc::c_ulong))
                    as *mut wchar_t
        }
    }
    (*n).value = buf;
    if !(*n).value.is_null() {
        (*n).len = wcslen((*n).value) as libc::c_int
    } else { (*n).len = 0 as libc::c_int };
}
unsafe extern "C" fn myquote(mut txt: *mut *mut txtnode,
                             mut text: *const wchar_t) {
    let mut q: [wchar_t; 2] = ['\"' as i32, 0 as libc::c_int];
    let mut segment_len: libc::c_int = 0;
    if wcscspn(text,
               (*::std::mem::transmute::<&[u8; 8],
                                         &[libc::c_int; 2]>(b"\'\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
           >
           wcscspn(text,
                   (*::std::mem::transmute::<&[u8; 8],
                                             &[libc::c_int; 2]>(b"\"\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
       {
        q[0 as libc::c_int as usize] = '\'' as i32
    }
    while *text != 0 {
        segment_len = wcscspn(text, q.as_mut_ptr()) as libc::c_int;
        newtxt(txt,
               (*::std::mem::transmute::<&[u8; 40],
                                         &[libc::c_int; 10]>(b"%\x00\x00\x00c\x00\x00\x00%\x00\x00\x00.\x00\x00\x00*\x00\x00\x00l\x00\x00\x00s\x00\x00\x00%\x00\x00\x00c\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
               q[0 as libc::c_int as usize], segment_len, text,
               q[0 as libc::c_int as usize]);
        q[0 as libc::c_int as usize] =
            if q[0 as libc::c_int as usize] == '\"' as i32 {
                '\'' as i32
            } else { '\"' as i32 };
        text = text.offset(segment_len as isize)
    };
}
unsafe extern "C" fn mydump(mut w: *mut stfl_widget,
                            mut prefix: *const wchar_t,
                            mut focus_id: libc::c_int,
                            mut txt: *mut *mut txtnode) {
    newtxt(txt,
           (*::std::mem::transmute::<&[u8; 32],
                                     &[libc::c_int; 8]>(b"{\x00\x00\x00%\x00\x00\x00l\x00\x00\x00s\x00\x00\x00%\x00\x00\x00l\x00\x00\x00s\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
           if (*w).id == focus_id {
               (*::std::mem::transmute::<&[u8; 8],
                                         &[libc::c_int; 2]>(b"!\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
           } else {
               (*::std::mem::transmute::<&[u8; 4],
                                         &[libc::c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr()
           }, (*(*w).type_0).name);
    if !(*w).cls.is_null() {
        newtxt(txt,
               (*::std::mem::transmute::<&[u8; 20],
                                         &[libc::c_int; 5]>(b"#\x00\x00\x00%\x00\x00\x00l\x00\x00\x00s\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
               (*w).cls);
    }
    if !(*w).name.is_null() {
        newtxt(txt,
               (*::std::mem::transmute::<&[u8; 8],
                                         &[libc::c_int; 2]>(b"[\x00\x00\x00\x00\x00\x00\x00")).as_ptr());
        myquote(txt, prefix);
        myquote(txt, (*w).name);
        newtxt(txt,
               (*::std::mem::transmute::<&[u8; 8],
                                         &[libc::c_int; 2]>(b"]\x00\x00\x00\x00\x00\x00\x00")).as_ptr());
    }
    let mut kv: *mut stfl_kv = (*w).kv_list;
    while !kv.is_null() {
        if !(*kv).name.is_null() {
            newtxt(txt,
                   (*::std::mem::transmute::<&[u8; 24],
                                             &[libc::c_int; 6]>(b" \x00\x00\x00%\x00\x00\x00l\x00\x00\x00s\x00\x00\x00[\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                   (*kv).key);
            myquote(txt, prefix);
            myquote(txt, (*kv).name);
            newtxt(txt,
                   (*::std::mem::transmute::<&[u8; 12],
                                             &[libc::c_int; 3]>(b"]\x00\x00\x00:\x00\x00\x00\x00\x00\x00\x00")).as_ptr());
        } else {
            newtxt(txt,
                   (*::std::mem::transmute::<&[u8; 24],
                                             &[libc::c_int; 6]>(b" \x00\x00\x00%\x00\x00\x00l\x00\x00\x00s\x00\x00\x00:\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                   (*kv).key);
        }
        myquote(txt, (*kv).value);
        kv = (*kv).next
    }
    let mut c: *mut stfl_widget = (*w).first_child;
    while !c.is_null() {
        mydump(c, prefix, focus_id, txt);
        c = (*c).next_sibling
    }
    newtxt(txt,
           (*::std::mem::transmute::<&[u8; 8],
                                     &[libc::c_int; 2]>(b"}\x00\x00\x00\x00\x00\x00\x00")).as_ptr());
}
unsafe extern "C" fn mytext(mut w: *mut stfl_widget,
                            mut txt: *mut *mut txtnode) {
    if wcscmp((*(*w).type_0).name,
              (*::std::mem::transmute::<&[u8; 36],
                                        &[libc::c_int; 9]>(b"l\x00\x00\x00i\x00\x00\x00s\x00\x00\x00t\x00\x00\x00i\x00\x00\x00t\x00\x00\x00e\x00\x00\x00m\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
           == 0 {
        let mut kv: *mut stfl_kv = (*w).kv_list;
        while !kv.is_null() {
            if wcscmp((*kv).key,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_int; 5]>(b"t\x00\x00\x00e\x00\x00\x00x\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                   == 0 {
                newtxt(txt,
                       (*::std::mem::transmute::<&[u8; 20],
                                                 &[libc::c_int; 5]>(b"%\x00\x00\x00l\x00\x00\x00s\x00\x00\x00\n\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                       (*kv).value);
            }
            kv = (*kv).next
        }
    }
    let mut c: *mut stfl_widget = (*w).first_child;
    while !c.is_null() { mytext(c, txt); c = (*c).next_sibling };
}
unsafe extern "C" fn txt2string(mut txt: *mut txtnode) -> *mut wchar_t {
    let mut string_len: libc::c_int = 0 as libc::c_int;
    let mut t: *mut txtnode = 0 as *mut txtnode;
    let mut prev: *mut txtnode = 0 as *mut txtnode;
    t = txt;
    while !t.is_null() { string_len += (*t).len; t = (*t).prev }
    let mut string: *mut wchar_t =
        malloc((::std::mem::size_of::<wchar_t>() as
                    libc::c_ulong).wrapping_mul((string_len +
                                                     1 as libc::c_int) as
                                                    libc::c_ulong)) as
            *mut wchar_t;
    let mut i: libc::c_int = string_len;
    t = txt;
    while !t.is_null() {
        i -= (*t).len;
        wmemcpy(string.offset(i as isize), (*t).value, (*t).len as size_t);
        prev = (*t).prev;
        free((*t).value as *mut libc::c_void);
        free(t as *mut libc::c_void);
        t = prev
    }
    *string.offset(string_len as isize) = 0 as libc::c_int;
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_quote_backend(mut text: *const wchar_t)
 -> *mut wchar_t {
    let mut txt: *mut txtnode = 0 as *mut txtnode;
    myquote(&mut txt, text);
    return txt2string(txt);
}
#[no_mangle]
pub unsafe extern "C" fn stfl_widget_dump(mut w: *mut stfl_widget,
                                          mut prefix: *const wchar_t,
                                          mut focus_id: libc::c_int)
 -> *mut wchar_t {
    let mut txt: *mut txtnode = 0 as *mut txtnode;
    mydump(w, prefix, focus_id, &mut txt);
    return txt2string(txt);
}
#[no_mangle]
pub unsafe extern "C" fn stfl_widget_text(mut w: *mut stfl_widget)
 -> *mut wchar_t {
    let mut txt: *mut txtnode = 0 as *mut txtnode;
    mytext(w, &mut txt);
    return txt2string(txt);
}
