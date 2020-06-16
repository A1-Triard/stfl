use ::libc;
extern "C" {
    pub type ldat;
    #[no_mangle]
    fn wmove(_: *mut WINDOW, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn wcswidth(__s: *const wchar_t, __n: size_t) -> libc::c_int;
    #[no_mangle]
    fn wcslen(_: *const libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn waddnwstr(_: *mut WINDOW, _: *const wchar_t, _: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn stfl_widget_setkv_int(w: *mut stfl_widget, key: *const wchar_t,
                             value: libc::c_int) -> *mut stfl_kv;
    #[no_mangle]
    fn stfl_matchbind(w: *mut stfl_widget, ch: wchar_t,
                      isfunckey: libc::c_int, name: *mut wchar_t,
                      auto_desc: *mut wchar_t) -> libc::c_int;
    #[no_mangle]
    fn stfl_widget_getkv_int(w: *mut stfl_widget, key: *const wchar_t,
                             defval: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn stfl_widget_getkv_str(w: *mut stfl_widget, key: *const wchar_t,
                             defval: *const wchar_t) -> *const wchar_t;
    #[no_mangle]
    fn stfl_style(win: *mut WINDOW, style: *const wchar_t);
    #[no_mangle]
    fn stfl_print_richtext(w: *mut stfl_widget, win: *mut WINDOW,
                           y: libc::c_uint, x: libc::c_uint,
                           text: *const wchar_t, width: libc::c_uint,
                           style: *const wchar_t, has_focus: libc::c_int)
     -> libc::c_uint;
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
 *  Copyright (C) 2006, 2007  Clifford Wolf <clifford@clifford.at>
 *  Copyright (C) 2006  Andreas Krennmair <ak@synflood.at>
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
 *  wt_textview.c: Widget type 'textview'
 */
unsafe extern "C" fn wt_textview_prepare(mut w: *mut stfl_widget,
                                         mut f: *mut stfl_form) {
    let mut c: *mut stfl_widget = (*w).first_child;
    (*w).min_w = 1 as libc::c_int;
    (*w).min_h = 5 as libc::c_int;
    if !c.is_null() { (*w).allow_focus = 1 as libc::c_int }
    while !c.is_null() {
        let mut text: *const wchar_t =
            stfl_widget_getkv_str(c,
                                  (*::std::mem::transmute::<&[u8; 20],
                                                            &[libc::c_int; 5]>(b"t\x00\x00\x00e\x00\x00\x00x\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                  (*::std::mem::transmute::<&[u8; 4],
                                                            &[libc::c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr());
        let mut len: libc::c_int = wcswidth(text, wcslen(text));
        (*w).min_w = if len > (*w).min_w { len } else { (*w).min_w };
        c = (*c).next_sibling
    };
}
unsafe extern "C" fn wt_textview_draw(mut w: *mut stfl_widget,
                                      mut f: *mut stfl_form,
                                      mut win: *mut WINDOW) {
    //fix_offset_pos(w);
    let mut offset: libc::c_int =
        stfl_widget_getkv_int(w,
                              (*::std::mem::transmute::<&[u8; 28],
                                                        &[libc::c_int; 7]>(b"o\x00\x00\x00f\x00\x00\x00f\x00\x00\x00s\x00\x00\x00e\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                              0 as libc::c_int);
    let mut is_richtext: libc::c_int =
        stfl_widget_getkv_int(w,
                              (*::std::mem::transmute::<&[u8; 36],
                                                        &[libc::c_int; 9]>(b"r\x00\x00\x00i\x00\x00\x00c\x00\x00\x00h\x00\x00\x00t\x00\x00\x00e\x00\x00\x00x\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                              0 as libc::c_int);
    let mut style_normal: *const wchar_t =
        stfl_widget_getkv_str(w,
                              (*::std::mem::transmute::<&[u8; 52],
                                                        &[libc::c_int; 13]>(b"s\x00\x00\x00t\x00\x00\x00y\x00\x00\x00l\x00\x00\x00e\x00\x00\x00_\x00\x00\x00n\x00\x00\x00o\x00\x00\x00r\x00\x00\x00m\x00\x00\x00a\x00\x00\x00l\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                              (*::std::mem::transmute::<&[u8; 4],
                                                        &[libc::c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr());
    let mut style_end: *const wchar_t =
        stfl_widget_getkv_str(w,
                              (*::std::mem::transmute::<&[u8; 40],
                                                        &[libc::c_int; 10]>(b"s\x00\x00\x00t\x00\x00\x00y\x00\x00\x00l\x00\x00\x00e\x00\x00\x00_\x00\x00\x00e\x00\x00\x00n\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                              (*::std::mem::transmute::<&[u8; 4],
                                                        &[libc::c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr());
    let mut c: *mut stfl_widget = 0 as *mut stfl_widget;
    let mut i: libc::c_int = 0;
    stfl_style(win, style_normal);
    i = 0 as libc::c_int;
    c = (*w).first_child;
    while !c.is_null() && i < offset + (*w).h {
        let mut text: *const wchar_t =
            stfl_widget_getkv_str(c,
                                  (*::std::mem::transmute::<&[u8; 20],
                                                            &[libc::c_int; 5]>(b"t\x00\x00\x00e\x00\x00\x00x\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                  (*::std::mem::transmute::<&[u8; 4],
                                                            &[libc::c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr());
        if i < offset {
            if is_richtext != 0 {
                stfl_print_richtext(w, win, (*w).y as libc::c_uint,
                                    (*w).x as libc::c_uint, text,
                                    0 as libc::c_int as libc::c_uint,
                                    style_normal, 0 as libc::c_int);
            }
        } else if is_richtext != 0 {
            stfl_print_richtext(w, win, ((*w).y + i - offset) as libc::c_uint,
                                (*w).x as libc::c_uint, text,
                                (*w).w as libc::c_uint, style_normal,
                                0 as libc::c_int);
        } else {
            if wmove(win, (*w).y + i - offset, (*w).x) == -(1 as libc::c_int)
               {
            } else { waddnwstr(win, text, (*w).w); };
        }
        i += 1;
        c = (*c).next_sibling
    }
    stfl_style(win, style_end);
    while i < offset + (*w).h {
        if wmove(win, (*w).y + i - offset, (*w).x) == -(1 as libc::c_int) {
        } else {
            waddnwstr(win,
                      (*::std::mem::transmute::<&[u8; 8],
                                                &[libc::c_int; 2]>(b"~\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                      (*w).w);
        };
        i += 1
    }
    if (*f).current_focus_id == (*w).id {
        (*f).cursor_y = -(1 as libc::c_int);
        (*f).cursor_x = (*f).cursor_y;
        (*(*f).root).cur_y = (*f).cursor_x;
        (*(*f).root).cur_x = (*(*f).root).cur_y
    };
}
unsafe extern "C" fn wt_textview_process(mut w: *mut stfl_widget,
                                         mut fw: *mut stfl_widget,
                                         mut f: *mut stfl_form,
                                         mut ch: wchar_t,
                                         mut isfunckey: libc::c_int)
 -> libc::c_int {
    //int pos = stfl_widget_getkv_int(w, "pos", 0);
    let mut offset: libc::c_int =
        stfl_widget_getkv_int(w,
                              (*::std::mem::transmute::<&[u8; 28],
                                                        &[libc::c_int; 7]>(b"o\x00\x00\x00f\x00\x00\x00f\x00\x00\x00s\x00\x00\x00e\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                              0 as libc::c_int);
    let mut maxoffset: libc::c_int = -(1 as libc::c_int);
    let mut c: *mut stfl_widget = (*w).first_child;
    while !c.is_null() { maxoffset += 1; c = (*c).next_sibling }
    if offset > 0 as libc::c_int &&
           stfl_matchbind(w, ch, isfunckey,
                          (*::std::mem::transmute::<&[u8; 12],
                                                    &[libc::c_int; 3]>(b"u\x00\x00\x00p\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                              as *mut wchar_t,
                          (*::std::mem::transmute::<&[u8; 12],
                                                    &[libc::c_int; 3]>(b"U\x00\x00\x00P\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                              as *mut wchar_t) != 0 {
        stfl_widget_setkv_int(w,
                              (*::std::mem::transmute::<&[u8; 28],
                                                        &[libc::c_int; 7]>(b"o\x00\x00\x00f\x00\x00\x00f\x00\x00\x00s\x00\x00\x00e\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                              offset - 1 as libc::c_int);
        //fix_offset_pos(w);
        return 1 as libc::c_int
    }
    if offset < maxoffset &&
           stfl_matchbind(w, ch, isfunckey,
                          (*::std::mem::transmute::<&[u8; 20],
                                                    &[libc::c_int; 5]>(b"d\x00\x00\x00o\x00\x00\x00w\x00\x00\x00n\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                              as *mut wchar_t,
                          (*::std::mem::transmute::<&[u8; 20],
                                                    &[libc::c_int; 5]>(b"D\x00\x00\x00O\x00\x00\x00W\x00\x00\x00N\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                              as *mut wchar_t) != 0 {
        stfl_widget_setkv_int(w,
                              (*::std::mem::transmute::<&[u8; 28],
                                                        &[libc::c_int; 7]>(b"o\x00\x00\x00f\x00\x00\x00f\x00\x00\x00s\x00\x00\x00e\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                              offset + 1 as libc::c_int);
        //fix_offset_pos(w);
        return 1 as libc::c_int
    }
    if stfl_matchbind(w, ch, isfunckey,
                      (*::std::mem::transmute::<&[u8; 32],
                                                &[libc::c_int; 8]>(b"p\x00\x00\x00a\x00\x00\x00g\x00\x00\x00e\x00\x00\x00_\x00\x00\x00u\x00\x00\x00p\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                          as *mut wchar_t,
                      (*::std::mem::transmute::<&[u8; 24],
                                                &[libc::c_int; 6]>(b"P\x00\x00\x00P\x00\x00\x00A\x00\x00\x00G\x00\x00\x00E\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                          as *mut wchar_t) != 0 {
        if offset - (*w).h + 1 as libc::c_int > 0 as libc::c_int {
            // XXX: first page handling won't work with that
            stfl_widget_setkv_int(w,
                                  (*::std::mem::transmute::<&[u8; 28],
                                                            &[libc::c_int; 7]>(b"o\x00\x00\x00f\x00\x00\x00f\x00\x00\x00s\x00\x00\x00e\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                  offset - (*w).h + 1 as libc::c_int);
        } else {
            stfl_widget_setkv_int(w,
                                  (*::std::mem::transmute::<&[u8; 28],
                                                            &[libc::c_int; 7]>(b"o\x00\x00\x00f\x00\x00\x00f\x00\x00\x00s\x00\x00\x00e\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                  0 as libc::c_int);
        }
        return 1 as libc::c_int
    }
    if stfl_matchbind(w, ch, isfunckey,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_int; 10]>(b"p\x00\x00\x00a\x00\x00\x00g\x00\x00\x00e\x00\x00\x00_\x00\x00\x00d\x00\x00\x00o\x00\x00\x00w\x00\x00\x00n\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                          as *mut wchar_t,
                      (*::std::mem::transmute::<&[u8; 24],
                                                &[libc::c_int; 6]>(b"N\x00\x00\x00P\x00\x00\x00A\x00\x00\x00G\x00\x00\x00E\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                          as *mut wchar_t) != 0 {
        if (offset + (*w).h - 1 as libc::c_int) < maxoffset {
            // XXX: last page handling won't work with that
            stfl_widget_setkv_int(w,
                                  (*::std::mem::transmute::<&[u8; 28],
                                                            &[libc::c_int; 7]>(b"o\x00\x00\x00f\x00\x00\x00f\x00\x00\x00s\x00\x00\x00e\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                  offset + (*w).h - 1 as libc::c_int);
        } else {
            stfl_widget_setkv_int(w,
                                  (*::std::mem::transmute::<&[u8; 28],
                                                            &[libc::c_int; 7]>(b"o\x00\x00\x00f\x00\x00\x00f\x00\x00\x00s\x00\x00\x00e\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                  maxoffset);
        }
        return 1 as libc::c_int
    }
    if stfl_matchbind(w, ch, isfunckey,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_int; 5]>(b"h\x00\x00\x00o\x00\x00\x00m\x00\x00\x00e\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                          as *mut wchar_t,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_int; 5]>(b"H\x00\x00\x00O\x00\x00\x00M\x00\x00\x00E\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                          as *mut wchar_t) != 0 {
        stfl_widget_setkv_int(w,
                              (*::std::mem::transmute::<&[u8; 28],
                                                        &[libc::c_int; 7]>(b"o\x00\x00\x00f\x00\x00\x00f\x00\x00\x00s\x00\x00\x00e\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                              0 as libc::c_int);
        return 1 as libc::c_int
    }
    if stfl_matchbind(w, ch, isfunckey,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_int; 4]>(b"e\x00\x00\x00n\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                          as *mut wchar_t,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_int; 4]>(b"E\x00\x00\x00N\x00\x00\x00D\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                          as *mut wchar_t) != 0 {
        stfl_widget_setkv_int(w,
                              (*::std::mem::transmute::<&[u8; 28],
                                                        &[libc::c_int; 7]>(b"o\x00\x00\x00f\x00\x00\x00f\x00\x00\x00s\x00\x00\x00e\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                              if (maxoffset - (*w).h + 2 as libc::c_int) <
                                     0 as libc::c_int {
                                  0 as libc::c_int
                              } else {
                                  (maxoffset - (*w).h) + 2 as libc::c_int
                              });
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut stfl_widget_type_textview: stfl_widget_type =
    unsafe {
        {
            let mut init =
                stfl_widget_type{name:
                                     (*::std::mem::transmute::<&[u8; 36],
                                                               &[libc::c_int; 9]>(b"t\x00\x00\x00e\x00\x00\x00x\x00\x00\x00t\x00\x00\x00v\x00\x00\x00i\x00\x00\x00e\x00\x00\x00w\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                                         as *mut wchar_t,
                                 f_init: None,
                                 f_done: None,
                                 f_enter: None,
                                 f_leave: None,
                                 f_prepare:
                                     Some(wt_textview_prepare as
                                              unsafe extern "C" fn(_:
                                                                       *mut stfl_widget,
                                                                   _:
                                                                       *mut stfl_form)
                                                  -> ()),
                                 f_draw:
                                     Some(wt_textview_draw as
                                              unsafe extern "C" fn(_:
                                                                       *mut stfl_widget,
                                                                   _:
                                                                       *mut stfl_form,
                                                                   _:
                                                                       *mut WINDOW)
                                                  -> ()),
                                 f_process:
                                     Some(wt_textview_process as
                                              unsafe extern "C" fn(_:
                                                                       *mut stfl_widget,
                                                                   _:
                                                                       *mut stfl_widget,
                                                                   _:
                                                                       *mut stfl_form,
                                                                   _: wchar_t,
                                                                   _:
                                                                       libc::c_int)
                                                  -> libc::c_int),};
            init
        }
    };
