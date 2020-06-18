use libc::*;
use crate::public::*;
extern "C" {
    pub type ldat;
    #[no_mangle]
    fn wmove(_: *mut WINDOW, _: c_int, _: c_int) -> c_int;
    #[no_mangle]
    fn wcswidth(__s: *const wchar_t, __n: size_t) -> c_int;
    #[no_mangle]
    fn wcslen(_: *const c_int) -> c_ulong;
    #[no_mangle]
    fn waddnwstr(_: *mut WINDOW, _: *const wchar_t, _: c_int)
     -> c_int;
    #[no_mangle]
    fn stfl_widget_getkv_str(w: *mut stfl_widget, key: *const wchar_t,
                             defval: *const wchar_t) -> *const wchar_t;
    #[no_mangle]
    fn stfl_print_richtext(w: *mut stfl_widget, win: *mut WINDOW,
                           y: c_uint, x: c_uint,
                           text: *const wchar_t, width: c_uint,
                           style: *const wchar_t, has_focus: c_int)
     -> c_uint;
    #[no_mangle]
    fn stfl_widget_getkv_int(w: *mut stfl_widget, key: *const wchar_t,
                             defval: c_int) -> c_int;
    #[no_mangle]
    fn stfl_widget_style(w: *mut stfl_widget, f: *mut stfl_form,
                         win: *mut WINDOW);
    #[no_mangle]
    fn malloc(_: c_ulong) -> *mut c_void;
    #[no_mangle]
    fn free(__ptr: *mut c_void);
}
pub type size_t = c_ulong;
pub type wchar_t = c_int;
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
 *  wt_label.c: Widget type 'label'
 */
unsafe extern "C" fn wt_label_prepare(w: *mut stfl_widget, _: *mut stfl_form) {
    let text: *const wchar_t =
        stfl_widget_getkv_str(w,
                              (*::std::mem::transmute::<&[u8; 20],
                                                        &[c_int; 5]>(b"t\x00\x00\x00e\x00\x00\x00x\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                              (*::std::mem::transmute::<&[u8; 4],
                                                        &[c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr());
    (*w).min_w = wcswidth(text, wcslen(text));
    (*w).min_h = 1 as c_int;
}
unsafe extern "C" fn wt_label_draw(w: *mut stfl_widget,
                                   f: *mut stfl_form,
                                   win: *mut WINDOW) {
    let is_richtext: c_int =
        stfl_widget_getkv_int(w,
                              (*::std::mem::transmute::<&[u8; 36],
                                                        &[c_int; 9]>(b"r\x00\x00\x00i\x00\x00\x00c\x00\x00\x00h\x00\x00\x00t\x00\x00\x00e\x00\x00\x00x\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                              0 as c_int);
    let style: *const wchar_t =
        stfl_widget_getkv_str(w,
                              (*::std::mem::transmute::<&[u8; 52],
                                                        &[c_int; 13]>(b"s\x00\x00\x00t\x00\x00\x00y\x00\x00\x00l\x00\x00\x00e\x00\x00\x00_\x00\x00\x00n\x00\x00\x00o\x00\x00\x00r\x00\x00\x00m\x00\x00\x00a\x00\x00\x00l\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                              (*::std::mem::transmute::<&[u8; 4],
                                                        &[c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr());
    stfl_widget_style(w, f, win);
    let text =
        stfl_widget_getkv_str(w,
                              (*::std::mem::transmute::<&[u8; 20],
                                                        &[c_int; 5]>(b"t\x00\x00\x00e\x00\x00\x00x\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                              (*::std::mem::transmute::<&[u8; 4],
                                                        &[c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr());
    let fillup: *mut wchar_t =
        malloc((::std::mem::size_of::<wchar_t>() as
                    c_ulong).wrapping_mul(((*w).w + 1 as c_int) as
                                                    c_ulong)) as
            *mut wchar_t;
    let mut i = 0 as c_uint;
    while i < (*w).w as c_uint {
        *fillup.offset(i as isize) = ' ' as i32;
        i = i.wrapping_add(1)
    }
    *fillup.offset((*w).w as isize) = '\u{0}' as i32;
    if wmove(win, (*w).y, (*w).x) == -(1 as c_int) {
    } else { waddnwstr(win, fillup, wcswidth(fillup, wcslen(fillup))); };
    free(fillup as *mut c_void);
    if is_richtext != 0 {
        stfl_print_richtext(w, win, (*w).y as c_uint,
                            (*w).x as c_uint, text,
                            (*w).w as c_uint, style, 0 as c_int);
    } else {
        if wmove(win, (*w).y, (*w).x) == -(1 as c_int) {
        } else { waddnwstr(win, text, (*w).w); };
    };
}
#[no_mangle]
pub static mut stfl_widget_type_label: stfl_widget_type =
    unsafe {
        {
            let init =
                stfl_widget_type{name:
                                     (*::std::mem::transmute::<&[u8; 24],
                                                               &[c_int; 6]>(b"l\x00\x00\x00a\x00\x00\x00b\x00\x00\x00e\x00\x00\x00l\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                                         as *mut wchar_t,
                                 f_init: None,
                                 f_done: None,
                                 f_enter: None,
                                 f_leave: None,
                                 f_prepare:
                                     Some(wt_label_prepare as
                                              unsafe extern "C" fn(_:
                                                                       *mut stfl_widget,
                                                                   _:
                                                                       *mut stfl_form)
                                                  -> ()),
                                 f_draw:
                                     Some(wt_label_draw as
                                              unsafe extern "C" fn(_:
                                                                       *mut stfl_widget,
                                                                   _:
                                                                       *mut stfl_form,
                                                                   _:
                                                                       *mut WINDOW)
                                                  -> ()),
                                 f_process: None,};
            init
        }
    };
