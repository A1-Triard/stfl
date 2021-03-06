use ::libc;
use crate::public::*;
extern "C" {
    pub type ldat;
    #[no_mangle]
    fn waddch(_: *mut WINDOW, _: chtype) -> libc::c_int;
    #[no_mangle]
    fn wmove(_: *mut WINDOW, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn wcschr(_: *const libc::c_int, _: libc::c_int) -> *mut libc::c_int;
    #[no_mangle]
    fn stfl_focus_next(w: *mut stfl_widget, old_fw: *mut stfl_widget,
                       f: *mut stfl_form) -> libc::c_int;
    #[no_mangle]
    fn stfl_matchbind(w: *mut stfl_widget, ch: wchar_t,
                      isfunckey: libc::c_int, name: *mut wchar_t,
                      auto_desc: *mut wchar_t) -> libc::c_int;
    #[no_mangle]
    fn stfl_focus_prev(w: *mut stfl_widget, old_fw: *mut stfl_widget,
                       f: *mut stfl_form) -> libc::c_int;
    #[no_mangle]
    fn stfl_widget_getkv_str(w: *mut stfl_widget, key: *const wchar_t,
                             defval: *const wchar_t) -> *const wchar_t;
    #[no_mangle]
    fn stfl_widget_getkv_int(w: *mut stfl_widget, key: *const wchar_t,
                             defval: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn stfl_widget_style(w: *mut stfl_widget, f: *mut stfl_form,
                         win: *mut WINDOW);
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
}
pub type wchar_t = libc::c_int;
pub type attr_t = chtype;
pub type chtype = libc::c_uint;
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
 *  wt_box.c: Widget types 'hbox' and 'vbox'
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct box_data {
    pub type_0: libc::c_char,
}
unsafe extern "C" fn wt_vbox_init(mut w: *mut stfl_widget) {
    let mut d: *mut box_data =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<box_data>() as libc::c_ulong) as
            *mut box_data;
    (*d).type_0 = 'V' as i32 as libc::c_char;
    (*w).internal_data = d as *mut libc::c_void;
}
unsafe extern "C" fn wt_hbox_init(mut w: *mut stfl_widget) {
    let mut d: *mut box_data =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<box_data>() as libc::c_ulong) as
            *mut box_data;
    (*d).type_0 = 'H' as i32 as libc::c_char;
    (*w).internal_data = d as *mut libc::c_void;
}
unsafe extern "C" fn wt_box_done(w: *mut stfl_widget) {
    free((*w).internal_data);
}
unsafe extern "C" fn wt_box_prepare(w: *mut stfl_widget,
                                    f: *mut stfl_form) {
    let d: *mut box_data = (*w).internal_data as *mut box_data;
    (*w).min_w = 0 as libc::c_int;
    (*w).min_h = 0 as libc::c_int;
    let mut c: *mut stfl_widget = (*w).first_child;
    while !c.is_null() {
        if stfl_widget_getkv_int(c,
                                 (*::std::mem::transmute::<&[u8; 36],
                                                           &[libc::c_int; 9]>(b".\x00\x00\x00d\x00\x00\x00i\x00\x00\x00s\x00\x00\x00p\x00\x00\x00l\x00\x00\x00a\x00\x00\x00y\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                 1 as libc::c_int) != 0 {
            (*(*c).type_0).f_prepare.expect("non-null function pointer")(c,
                                                                         f);
            if (*d).type_0 as libc::c_int == 'H' as i32 {
                if (*w).min_h < (*c).min_h { (*w).min_h = (*c).min_h }
                (*w).min_w += (*c).min_w
            } else {
                if (*w).min_w < (*c).min_w { (*w).min_w = (*c).min_w }
                (*w).min_h += (*c).min_h
            }
        }
        c = (*c).next_sibling
    };
}
unsafe extern "C" fn wt_box_draw(w: *mut stfl_widget,
                                 f: *mut stfl_form,
                                 win: *mut WINDOW) {
    let d: *mut box_data = (*w).internal_data as *mut box_data;
    let mut num_dyn_children: libc::c_int = 0 as libc::c_int;
    let mut min_w: libc::c_int = 0 as libc::c_int;
    let mut min_h: libc::c_int = 0 as libc::c_int;
    let mut c: *mut stfl_widget = (*w).first_child;
    while !c.is_null() {
        if stfl_widget_getkv_int(c,
                                 (*::std::mem::transmute::<&[u8; 36],
                                                           &[libc::c_int; 9]>(b".\x00\x00\x00d\x00\x00\x00i\x00\x00\x00s\x00\x00\x00p\x00\x00\x00l\x00\x00\x00a\x00\x00\x00y\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                 1 as libc::c_int) != 0 {
            let mut size_w: libc::c_int =
                stfl_widget_getkv_int(c,
                                      (*::std::mem::transmute::<&[u8; 28],
                                                                &[libc::c_int; 7]>(b".\x00\x00\x00w\x00\x00\x00i\x00\x00\x00d\x00\x00\x00t\x00\x00\x00h\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                      0 as libc::c_int);
            if size_w < (*c).min_w { size_w = (*c).min_w }
            let mut size_h: libc::c_int =
                stfl_widget_getkv_int(c,
                                      (*::std::mem::transmute::<&[u8; 32],
                                                                &[libc::c_int; 8]>(b".\x00\x00\x00h\x00\x00\x00e\x00\x00\x00i\x00\x00\x00g\x00\x00\x00h\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                      0 as libc::c_int);
            if size_h < (*c).min_h { size_h = (*c).min_h }
            if !wcschr(stfl_widget_getkv_str(c,
                                             (*::std::mem::transmute::<&[u8; 32],
                                                                       &[libc::c_int; 8]>(b".\x00\x00\x00e\x00\x00\x00x\x00\x00\x00p\x00\x00\x00a\x00\x00\x00n\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                             (*::std::mem::transmute::<&[u8; 12],
                                                                       &[libc::c_int; 3]>(b"v\x00\x00\x00h\x00\x00\x00\x00\x00\x00\x00")).as_ptr()),
                       if (*d).type_0 as libc::c_int == 'H' as i32 {
                           'h' as i32
                       } else { 'v' as i32 }).is_null() {
                num_dyn_children += 1
            }
            if (*d).type_0 as libc::c_int == 'H' as i32 {
                min_w += size_w;
                if min_h < size_h { min_h = size_h }
            } else { min_h += size_h; if min_w < size_w { min_w = size_w } }
        }
        c = (*c).next_sibling
    }
    let mut box_x: libc::c_int = (*w).x;
    let mut box_y: libc::c_int = (*w).y;
    let mut box_w: libc::c_int = (*w).w;
    let mut box_h: libc::c_int = (*w).h;
    stfl_widget_style(w, f, win);
    let mut i = box_x;
    while i < box_x + box_w {
        let mut j = box_y;
        while j < box_y + box_h {
            if wmove(win, j, i) == -(1 as libc::c_int) {
            } else { waddch(win, ' ' as i32 as chtype); };
            j += 1
        }
        i += 1
    }
    let mut tie: *const wchar_t =
        stfl_widget_getkv_str(w,
                              (*::std::mem::transmute::<&[u8; 16],
                                                        &[libc::c_int; 4]>(b"t\x00\x00\x00i\x00\x00\x00e\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                              (*::std::mem::transmute::<&[u8; 20],
                                                        &[libc::c_int; 5]>(b"l\x00\x00\x00r\x00\x00\x00t\x00\x00\x00b\x00\x00\x00\x00\x00\x00\x00")).as_ptr());
    if wcschr(tie, 'l' as i32).is_null() && wcschr(tie, 'r' as i32).is_null()
       {
        box_x += (box_w - min_w) / 2 as libc::c_int
    }
    if wcschr(tie, 'l' as i32).is_null() && !wcschr(tie, 'r' as i32).is_null()
       {
        box_x += box_w - min_w
    }
    if wcschr(tie, 'l' as i32).is_null() || wcschr(tie, 'r' as i32).is_null()
       {
        box_w = min_w
    }
    if wcschr(tie, 't' as i32).is_null() && wcschr(tie, 'b' as i32).is_null()
       {
        box_y += (box_h - min_h) / 2 as libc::c_int
    }
    if wcschr(tie, 't' as i32).is_null() && !wcschr(tie, 'b' as i32).is_null()
       {
        box_y += box_h - min_h
    }
    if wcschr(tie, 't' as i32).is_null() || wcschr(tie, 'b' as i32).is_null()
       {
        box_h = min_h
    }
    let mut sizes_extra: libc::c_int =
        if (*d).type_0 as libc::c_int == 'H' as i32 {
            (box_w) - min_w
        } else { (box_h) - min_h };
    let mut cursor: libc::c_int =
        if (*d).type_0 as libc::c_int == 'H' as i32 { box_x } else { box_y };
    let mut c = (*w).first_child;
    while !c.is_null() {
        if stfl_widget_getkv_int(c,
                                 (*::std::mem::transmute::<&[u8; 36],
                                                           &[libc::c_int; 9]>(b".\x00\x00\x00d\x00\x00\x00i\x00\x00\x00s\x00\x00\x00p\x00\x00\x00l\x00\x00\x00a\x00\x00\x00y\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                 1 as libc::c_int) != 0 {
            let mut size: libc::c_int =
                stfl_widget_getkv_int(c,
                                      if (*d).type_0 as libc::c_int ==
                                             'H' as i32 {
                                          (*::std::mem::transmute::<&[u8; 28],
                                                                    &[libc::c_int; 7]>(b".\x00\x00\x00w\x00\x00\x00i\x00\x00\x00d\x00\x00\x00t\x00\x00\x00h\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                                      } else {
                                          (*::std::mem::transmute::<&[u8; 32],
                                                                    &[libc::c_int; 8]>(b".\x00\x00\x00h\x00\x00\x00e\x00\x00\x00i\x00\x00\x00g\x00\x00\x00h\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                                      }, 0 as libc::c_int);
            if size <
                   (if (*d).type_0 as libc::c_int == 'H' as i32 {
                        (*c).min_w
                    } else { (*c).min_h }) {
                size =
                    if (*d).type_0 as libc::c_int == 'H' as i32 {
                        (*c).min_w
                    } else { (*c).min_h }
            }
            if !wcschr(stfl_widget_getkv_str(c,
                                             (*::std::mem::transmute::<&[u8; 32],
                                                                       &[libc::c_int; 8]>(b".\x00\x00\x00e\x00\x00\x00x\x00\x00\x00p\x00\x00\x00a\x00\x00\x00n\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                             (*::std::mem::transmute::<&[u8; 12],
                                                                       &[libc::c_int; 3]>(b"v\x00\x00\x00h\x00\x00\x00\x00\x00\x00\x00")).as_ptr()),
                       if (*d).type_0 as libc::c_int == 'H' as i32 {
                           'h' as i32
                       } else { 'v' as i32 }).is_null() {
                let fresh0 = num_dyn_children;
                num_dyn_children = num_dyn_children - 1;
                let extra: libc::c_int = sizes_extra / fresh0;
                sizes_extra -= extra;
                size += extra
            }
            if (*d).type_0 as libc::c_int == 'H' as i32 {
                (*c).y = box_y;
                (*c).x = cursor;
                (*c).h = box_h;
                (*c).w = size;
                cursor += (*c).w
            } else {
                (*c).x = box_x;
                (*c).y = cursor;
                (*c).w = box_w;
                (*c).h = size;
                cursor += (*c).h
            }
            tie =
                stfl_widget_getkv_str(c,
                                      (*::std::mem::transmute::<&[u8; 20],
                                                                &[libc::c_int; 5]>(b".\x00\x00\x00t\x00\x00\x00i\x00\x00\x00e\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                      (*::std::mem::transmute::<&[u8; 20],
                                                                &[libc::c_int; 5]>(b"l\x00\x00\x00r\x00\x00\x00t\x00\x00\x00b\x00\x00\x00\x00\x00\x00\x00")).as_ptr());
            if wcschr(tie, 'l' as i32).is_null() &&
                   wcschr(tie, 'r' as i32).is_null() {
                (*c).x += ((*c).w - (*c).min_w) / 2 as libc::c_int
            }
            if wcschr(tie, 'l' as i32).is_null() &&
                   !wcschr(tie, 'r' as i32).is_null() {
                (*c).x += (*c).w - (*c).min_w
            }
            if wcschr(tie, 'l' as i32).is_null() ||
                   wcschr(tie, 'r' as i32).is_null() {
                (*c).w = (*c).min_w
            }
            if wcschr(tie, 't' as i32).is_null() &&
                   wcschr(tie, 'b' as i32).is_null() {
                (*c).y += ((*c).h - (*c).min_h) / 2 as libc::c_int
            }
            if wcschr(tie, 't' as i32).is_null() &&
                   !wcschr(tie, 'b' as i32).is_null() {
                (*c).y += (*c).h - (*c).min_h
            }
            if wcschr(tie, 't' as i32).is_null() ||
                   wcschr(tie, 'b' as i32).is_null() {
                (*c).h = (*c).min_h
            }
            (*(*c).type_0).f_draw.expect("non-null function pointer")(c, f,
                                                                      win);
        }
        c = (*c).next_sibling;
    };
}
unsafe extern "C" fn wt_box_process(w: *mut stfl_widget,
                                    fw: *mut stfl_widget,
                                    f: *mut stfl_form, ch: wchar_t,
                                    isfunckey: libc::c_int)
 -> libc::c_int {
    let d: *mut box_data = (*w).internal_data as *mut box_data;
    if (*d).type_0 as libc::c_int == 'H' as i32 {
        if stfl_matchbind(w, ch, isfunckey,
                          (*::std::mem::transmute::<&[u8; 20],
                                                    &[libc::c_int; 5]>(b"l\x00\x00\x00e\x00\x00\x00f\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                              as *mut wchar_t,
                          (*::std::mem::transmute::<&[u8; 20],
                                                    &[libc::c_int; 5]>(b"L\x00\x00\x00E\x00\x00\x00F\x00\x00\x00T\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                              as *mut wchar_t) != 0 {
            return stfl_focus_prev(w, fw, f)
        }
        if stfl_matchbind(w, ch, isfunckey,
                          (*::std::mem::transmute::<&[u8; 24],
                                                    &[libc::c_int; 6]>(b"r\x00\x00\x00i\x00\x00\x00g\x00\x00\x00h\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                              as *mut wchar_t,
                          (*::std::mem::transmute::<&[u8; 24],
                                                    &[libc::c_int; 6]>(b"R\x00\x00\x00I\x00\x00\x00G\x00\x00\x00H\x00\x00\x00T\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                              as *mut wchar_t) != 0 {
            return stfl_focus_next(w, fw, f)
        }
    }
    if (*d).type_0 as libc::c_int == 'V' as i32 {
        if stfl_matchbind(w, ch, isfunckey,
                          (*::std::mem::transmute::<&[u8; 12],
                                                    &[libc::c_int; 3]>(b"u\x00\x00\x00p\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                              as *mut wchar_t,
                          (*::std::mem::transmute::<&[u8; 12],
                                                    &[libc::c_int; 3]>(b"U\x00\x00\x00P\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                              as *mut wchar_t) != 0 {
            return stfl_focus_prev(w, fw, f)
        }
        if stfl_matchbind(w, ch, isfunckey,
                          (*::std::mem::transmute::<&[u8; 20],
                                                    &[libc::c_int; 5]>(b"d\x00\x00\x00o\x00\x00\x00w\x00\x00\x00n\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                              as *mut wchar_t,
                          (*::std::mem::transmute::<&[u8; 20],
                                                    &[libc::c_int; 5]>(b"D\x00\x00\x00O\x00\x00\x00W\x00\x00\x00N\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                              as *mut wchar_t) != 0 {
            return stfl_focus_next(w, fw, f)
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut stfl_widget_type_vbox: stfl_widget_type =
    unsafe {
        {
            let init =
                stfl_widget_type{name:
                                     (*::std::mem::transmute::<&[u8; 20],
                                                               &[libc::c_int; 5]>(b"v\x00\x00\x00b\x00\x00\x00o\x00\x00\x00x\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                                         as *mut wchar_t,
                                 f_init:
                                     Some(wt_vbox_init as
                                              unsafe extern "C" fn(_:
                                                                       *mut stfl_widget)
                                                  -> ()),
                                 f_done:
                                     Some(wt_box_done as
                                              unsafe extern "C" fn(_:
                                                                       *mut stfl_widget)
                                                  -> ()),
                                 f_enter: None,
                                 f_leave: None,
                                 f_prepare:
                                     Some(wt_box_prepare as
                                              unsafe extern "C" fn(_:
                                                                       *mut stfl_widget,
                                                                   _:
                                                                       *mut stfl_form)
                                                  -> ()),
                                 f_draw:
                                     Some(wt_box_draw as
                                              unsafe extern "C" fn(_:
                                                                       *mut stfl_widget,
                                                                   _:
                                                                       *mut stfl_form,
                                                                   _:
                                                                       *mut WINDOW)
                                                  -> ()),
                                 f_process:
                                     Some(wt_box_process as
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
#[no_mangle]
pub static mut stfl_widget_type_hbox: stfl_widget_type =
    unsafe {
        {
            let init =
                stfl_widget_type{name:
                                     (*::std::mem::transmute::<&[u8; 20],
                                                               &[libc::c_int; 5]>(b"h\x00\x00\x00b\x00\x00\x00o\x00\x00\x00x\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                                         as *mut wchar_t,
                                 f_init:
                                     Some(wt_hbox_init as
                                              unsafe extern "C" fn(_:
                                                                       *mut stfl_widget)
                                                  -> ()),
                                 f_done:
                                     Some(wt_box_done as
                                              unsafe extern "C" fn(_:
                                                                       *mut stfl_widget)
                                                  -> ()),
                                 f_enter: None,
                                 f_leave: None,
                                 f_prepare:
                                     Some(wt_box_prepare as
                                              unsafe extern "C" fn(_:
                                                                       *mut stfl_widget,
                                                                   _:
                                                                       *mut stfl_form)
                                                  -> ()),
                                 f_draw:
                                     Some(wt_box_draw as
                                              unsafe extern "C" fn(_:
                                                                       *mut stfl_widget,
                                                                   _:
                                                                       *mut stfl_form,
                                                                   _:
                                                                       *mut WINDOW)
                                                  -> ()),
                                 f_process:
                                     Some(wt_box_process as
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
