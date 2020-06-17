use ::libc;
extern "C" {
    pub type ldat;
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
    #[no_mangle]
    fn stfl_widget_free(w: *mut stfl_widget);
    #[no_mangle]
    fn stfl_widget_text(w: *mut stfl_widget) -> *mut wchar_t;
    #[no_mangle]
    fn stfl_widget_dump(w: *mut stfl_widget, prefix: *const wchar_t,
                        focus_id: libc::c_int) -> *mut wchar_t;
    #[no_mangle]
    fn pthread_key_create(__key: *mut pthread_key_t,
                          __destr_function:
                              Option<unsafe extern "C" fn(_:
                                                              *mut libc::c_void)
                                         -> ()>) -> libc::c_int;
    #[no_mangle]
    fn pthread_getspecific(__key: pthread_key_t) -> *mut libc::c_void;
    #[no_mangle]
    fn stfl_quote_backend(text: *const wchar_t) -> *mut wchar_t;
    #[no_mangle]
    fn pthread_setspecific(__key: pthread_key_t,
                           __pointer: *const libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn stfl_switch_focus(old_fw: *mut stfl_widget, new_fw: *mut stfl_widget,
                         f: *mut stfl_form) -> libc::c_int;
    #[no_mangle]
    fn stfl_widget_by_id(w: *mut stfl_widget, id: libc::c_int)
     -> *mut stfl_widget;
    #[no_mangle]
    fn stfl_setkv_by_name_str(w: *mut stfl_widget, name: *const wchar_t,
                              value: *const wchar_t) -> *mut stfl_kv;
    #[no_mangle]
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn stfl_widget_by_name(w: *mut stfl_widget, name: *const wchar_t)
     -> *mut stfl_widget;
    #[no_mangle]
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn stfl_getkv_by_name_str(w: *mut stfl_widget, name: *const wchar_t,
                              defval: *const wchar_t) -> *const wchar_t;
    #[no_mangle]
    fn stfl_form_reset();
    #[no_mangle]
    fn stfl_form_redraw();
    #[no_mangle]
    fn stfl_form_run(f: *mut stfl_form, timeout: libc::c_int);
    #[no_mangle]
    fn stfl_form_free(f: *mut stfl_form);
    #[no_mangle]
    fn stfl_parser(text: *const wchar_t) -> *mut stfl_widget;
    #[no_mangle]
    fn stfl_check_setfocus(f: *mut stfl_form, w: *mut stfl_widget);
    #[no_mangle]
    fn stfl_form_new() -> *mut stfl_form;
    #[no_mangle]
    fn swprintf(__s: *mut wchar_t, __n: size_t, __format: *const wchar_t,
                _: ...) -> libc::c_int;
    #[no_mangle]
    fn wmemcpy(__s1: *mut wchar_t, __s2: *const wchar_t, __n: size_t)
     -> *mut wchar_t;
    #[no_mangle]
    fn wcschr(_: *const libc::c_int, _: libc::c_int) -> *mut libc::c_int;
    #[no_mangle]
    fn wcscmp(_: *const libc::c_int, _: *const libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abort() -> !;
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
pub type pthread_key_t = libc::c_uint;
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
 *  public.c: Public STFL API
 */
#[no_mangle]
pub static mut stfl_api_allow_null_pointers: libc::c_int = 1 as libc::c_int;
unsafe extern "C" fn checkret(txt: *const wchar_t) -> *const wchar_t {
    if stfl_api_allow_null_pointers == 0 && txt.is_null() {
        return (*::std::mem::transmute::<&[u8; 4],
                                         &[libc::c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr()
    }
    return txt;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_create(text: *const wchar_t)
 -> *mut stfl_form {
    let mut f: *mut stfl_form = stfl_form_new();
    (*f).root =
        stfl_parser(if !text.is_null() {
                        text
                    } else {
                        (*::std::mem::transmute::<&[u8; 4],
                                                  &[libc::c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr()
                    });
    stfl_check_setfocus(f, (*f).root);
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_free(f: *mut stfl_form) {
    stfl_form_free(f);
}
#[no_mangle]
pub unsafe extern "C" fn stfl_redraw() { stfl_form_redraw(); }
#[no_mangle]
pub unsafe extern "C" fn stfl_run(f: *mut stfl_form,
                                  timeout: libc::c_int)
 -> *const wchar_t {
    stfl_form_run(f, timeout);
    return checkret((*f).event);
}
#[no_mangle]
pub unsafe extern "C" fn stfl_reset() { stfl_form_reset(); }
#[no_mangle]
pub unsafe extern "C" fn stfl_get(f: *mut stfl_form,
                                  name: *const wchar_t)
 -> *const wchar_t {
    let pseudovar_sep: *mut wchar_t =
        if !name.is_null() {
            wcschr(name, ':' as i32)
        } else { 0 as *mut libc::c_int };
    pthread_mutex_lock(&mut (*f).mtx);
    if !pseudovar_sep.is_null() {
        let vla =
            (pseudovar_sep.wrapping_offset_from(name) as libc::c_long +
                 1 as libc::c_int as libc::c_long) as usize;
        let mut w_name: Vec<wchar_t> = ::std::vec::from_elem(0, vla);
        wmemcpy(w_name.as_mut_ptr(), name,
                pseudovar_sep.wrapping_offset_from(name) as libc::c_long as
                    size_t);
        *w_name.as_mut_ptr().offset(pseudovar_sep.wrapping_offset_from(name)
                                        as libc::c_long as isize) =
            0 as libc::c_int;
        let w: *mut stfl_widget =
            stfl_widget_by_name((*f).root, w_name.as_mut_ptr());
        static mut ret_buffer: [wchar_t; 16] = [0; 16];
        if !w.is_null() {
            if wcscmp(pseudovar_sep.offset(1 as libc::c_int as isize),
                      (*::std::mem::transmute::<&[u8; 8],
                                                &[libc::c_int; 2]>(b"x\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                   == 0 {
                swprintf(ret_buffer.as_mut_ptr(), 16 as libc::c_int as size_t,
                         (*::std::mem::transmute::<&[u8; 12],
                                                   &[libc::c_int; 3]>(b"%\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                         (*w).x);
                pthread_mutex_unlock(&mut (*f).mtx);
                return checkret(ret_buffer.as_mut_ptr())
            }
            if wcscmp(pseudovar_sep.offset(1 as libc::c_int as isize),
                      (*::std::mem::transmute::<&[u8; 8],
                                                &[libc::c_int; 2]>(b"y\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                   == 0 {
                swprintf(ret_buffer.as_mut_ptr(), 16 as libc::c_int as size_t,
                         (*::std::mem::transmute::<&[u8; 12],
                                                   &[libc::c_int; 3]>(b"%\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                         (*w).y);
                pthread_mutex_unlock(&mut (*f).mtx);
                return checkret(ret_buffer.as_mut_ptr())
            }
            if wcscmp(pseudovar_sep.offset(1 as libc::c_int as isize),
                      (*::std::mem::transmute::<&[u8; 8],
                                                &[libc::c_int; 2]>(b"w\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                   == 0 {
                swprintf(ret_buffer.as_mut_ptr(), 16 as libc::c_int as size_t,
                         (*::std::mem::transmute::<&[u8; 12],
                                                   &[libc::c_int; 3]>(b"%\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                         (*w).w);
                pthread_mutex_unlock(&mut (*f).mtx);
                return checkret(ret_buffer.as_mut_ptr())
            }
            if wcscmp(pseudovar_sep.offset(1 as libc::c_int as isize),
                      (*::std::mem::transmute::<&[u8; 8],
                                                &[libc::c_int; 2]>(b"h\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                   == 0 {
                swprintf(ret_buffer.as_mut_ptr(), 16 as libc::c_int as size_t,
                         (*::std::mem::transmute::<&[u8; 12],
                                                   &[libc::c_int; 3]>(b"%\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                         (*w).h);
                pthread_mutex_unlock(&mut (*f).mtx);
                return checkret(ret_buffer.as_mut_ptr())
            }
            if wcscmp(pseudovar_sep.offset(1 as libc::c_int as isize),
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_int; 5]>(b"m\x00\x00\x00i\x00\x00\x00n\x00\x00\x00w\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                   == 0 {
                swprintf(ret_buffer.as_mut_ptr(), 16 as libc::c_int as size_t,
                         (*::std::mem::transmute::<&[u8; 12],
                                                   &[libc::c_int; 3]>(b"%\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                         (*w).min_w);
                pthread_mutex_unlock(&mut (*f).mtx);
                return checkret(ret_buffer.as_mut_ptr())
            }
            if wcscmp(pseudovar_sep.offset(1 as libc::c_int as isize),
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_int; 5]>(b"m\x00\x00\x00i\x00\x00\x00n\x00\x00\x00h\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                   == 0 {
                swprintf(ret_buffer.as_mut_ptr(), 16 as libc::c_int as size_t,
                         (*::std::mem::transmute::<&[u8; 12],
                                                   &[libc::c_int; 3]>(b"%\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                         (*w).min_h);
                pthread_mutex_unlock(&mut (*f).mtx);
                return checkret(ret_buffer.as_mut_ptr())
            }
        }
    }
    let tmpstr: *const wchar_t =
        stfl_getkv_by_name_str((*f).root,
                               if !name.is_null() {
                                   name
                               } else {
                                   (*::std::mem::transmute::<&[u8; 4],
                                                             &[libc::c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr()
                               }, 0 as *const wchar_t);
    pthread_mutex_unlock(&mut (*f).mtx);
    return checkret(tmpstr);
}
#[no_mangle]
pub unsafe extern "C" fn stfl_set(f: *mut stfl_form,
                                  name: *const wchar_t,
                                  value: *const wchar_t) {
    pthread_mutex_lock(&mut (*f).mtx);
    stfl_setkv_by_name_str((*f).root,
                           if !name.is_null() {
                               name
                           } else {
                               (*::std::mem::transmute::<&[u8; 4],
                                                         &[libc::c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr()
                           },
                           if !value.is_null() {
                               value
                           } else {
                               (*::std::mem::transmute::<&[u8; 4],
                                                         &[libc::c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr()
                           });
    pthread_mutex_unlock(&mut (*f).mtx);
}
#[no_mangle]
pub unsafe extern "C" fn stfl_get_focus(f: *mut stfl_form)
 -> *const wchar_t {
    pthread_mutex_lock(&mut (*f).mtx);
    let fw = stfl_widget_by_id((*f).root, (*f).current_focus_id);
    let tmpstr = checkret(if !fw.is_null() { (*fw).name } else { 0 as *mut wchar_t });
    pthread_mutex_unlock(&mut (*f).mtx);
    return tmpstr;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_set_focus(f: *mut stfl_form,
                                        name: *const wchar_t) {
    pthread_mutex_lock(&mut (*f).mtx);
    let fw =
        stfl_widget_by_name((*f).root,
                            if !name.is_null() {
                                name
                            } else {
                                (*::std::mem::transmute::<&[u8; 4],
                                                          &[libc::c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr()
                            });
    stfl_switch_focus(0 as *mut stfl_widget, fw, f);
    pthread_mutex_unlock(&mut (*f).mtx);
}
#[no_mangle]
pub unsafe extern "C" fn stfl_quote(text: *const wchar_t)
 -> *const wchar_t {
    static mut mtx: pthread_mutex_t =
        pthread_mutex_t{__data:
                            {
                                let init =
                                    __pthread_mutex_s{__lock:
                                                          0 as libc::c_int,
                                                      __count:
                                                          0 as libc::c_int as
                                                              libc::c_uint,
                                                      __owner:
                                                          0 as libc::c_int,
                                                      __nusers:
                                                          0 as libc::c_int as
                                                              libc::c_uint,
                                                      __kind:
                                                          0 as libc::c_int,
                                                      __spins:
                                                          0 as libc::c_int as
                                                              libc::c_short,
                                                      __elision:
                                                          0 as libc::c_int as
                                                              libc::c_short,
                                                      __list:
                                                          {
                                                              let init =
                                                                  __pthread_internal_list{__prev:
                                                                                              0
                                                                                                  as
                                                                                                  *const __pthread_internal_list
                                                                                                  as
                                                                                                  *mut __pthread_internal_list,
                                                                                          __next:
                                                                                              0
                                                                                                  as
                                                                                                  *const __pthread_internal_list
                                                                                                  as
                                                                                                  *mut __pthread_internal_list,};
                                                              init
                                                          },};
                                init
                            },};
    static mut retbuffer_key: pthread_key_t = 0;
    static mut firstrun: libc::c_int = 1 as libc::c_int;
    static mut retbuffer: *mut wchar_t = 0 as *const wchar_t as *mut wchar_t;
    pthread_mutex_lock(&mut mtx);
    if firstrun != 0 {
        pthread_key_create(&mut retbuffer_key,
                           Some(free as
                                    unsafe extern "C" fn(_: *mut libc::c_void)
                                        -> ()));
        firstrun = 0 as libc::c_int
    }
    retbuffer = pthread_getspecific(retbuffer_key) as *mut wchar_t;
    if !retbuffer.is_null() { free(retbuffer as *mut libc::c_void); }
    retbuffer =
        stfl_quote_backend(if !text.is_null() {
                               text
                           } else {
                               (*::std::mem::transmute::<&[u8; 4],
                                                         &[libc::c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr()
                           });
    pthread_setspecific(retbuffer_key, retbuffer as *const libc::c_void);
    pthread_mutex_unlock(&mut mtx);
    return checkret(retbuffer);
}
#[no_mangle]
pub unsafe extern "C" fn stfl_dump(f: *mut stfl_form,
                                   name: *const wchar_t,
                                   prefix: *const wchar_t,
                                   focus: libc::c_int) -> *const wchar_t {
    static mut mtx: pthread_mutex_t =
        pthread_mutex_t{__data:
                            {
                                let init =
                                    __pthread_mutex_s{__lock:
                                                          0 as libc::c_int,
                                                      __count:
                                                          0 as libc::c_int as
                                                              libc::c_uint,
                                                      __owner:
                                                          0 as libc::c_int,
                                                      __nusers:
                                                          0 as libc::c_int as
                                                              libc::c_uint,
                                                      __kind:
                                                          0 as libc::c_int,
                                                      __spins:
                                                          0 as libc::c_int as
                                                              libc::c_short,
                                                      __elision:
                                                          0 as libc::c_int as
                                                              libc::c_short,
                                                      __list:
                                                          {
                                                              let init =
                                                                  __pthread_internal_list{__prev:
                                                                                              0
                                                                                                  as
                                                                                                  *const __pthread_internal_list
                                                                                                  as
                                                                                                  *mut __pthread_internal_list,
                                                                                          __next:
                                                                                              0
                                                                                                  as
                                                                                                  *const __pthread_internal_list
                                                                                                  as
                                                                                                  *mut __pthread_internal_list,};
                                                              init
                                                          },};
                                init
                            },};
    static mut retbuffer_key: pthread_key_t = 0;
    static mut firstrun: libc::c_int = 1 as libc::c_int;
    static mut retbuffer: *mut wchar_t = 0 as *const wchar_t as *mut wchar_t;
    pthread_mutex_lock(&mut mtx);
    pthread_mutex_lock(&mut (*f).mtx);
    if firstrun != 0 {
        pthread_key_create(&mut retbuffer_key,
                           Some(free as
                                    unsafe extern "C" fn(_: *mut libc::c_void)
                                        -> ()));
        firstrun = 0 as libc::c_int
    }
    retbuffer = pthread_getspecific(retbuffer_key) as *mut wchar_t;
    if !retbuffer.is_null() { free(retbuffer as *mut libc::c_void); }
    let w =
        if !name.is_null() && *name != 0 {
            stfl_widget_by_name((*f).root, name)
        } else { (*f).root };
    retbuffer =
        stfl_widget_dump(w,
                         if !prefix.is_null() {
                             prefix
                         } else {
                             (*::std::mem::transmute::<&[u8; 4],
                                                       &[libc::c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr()
                         },
                         if focus != 0 {
                             (*f).current_focus_id
                         } else { 0 as libc::c_int });
    pthread_setspecific(retbuffer_key, retbuffer as *const libc::c_void);
    pthread_mutex_unlock(&mut (*f).mtx);
    pthread_mutex_unlock(&mut mtx);
    return checkret(retbuffer);
}
#[no_mangle]
pub unsafe extern "C" fn stfl_text(f: *mut stfl_form,
                                   name: *const wchar_t)
 -> *const wchar_t {
    static mut mtx: pthread_mutex_t =
        pthread_mutex_t{__data:
                            {
                                let init =
                                    __pthread_mutex_s{__lock:
                                                          0 as libc::c_int,
                                                      __count:
                                                          0 as libc::c_int as
                                                              libc::c_uint,
                                                      __owner:
                                                          0 as libc::c_int,
                                                      __nusers:
                                                          0 as libc::c_int as
                                                              libc::c_uint,
                                                      __kind:
                                                          0 as libc::c_int,
                                                      __spins:
                                                          0 as libc::c_int as
                                                              libc::c_short,
                                                      __elision:
                                                          0 as libc::c_int as
                                                              libc::c_short,
                                                      __list:
                                                          {
                                                              let init =
                                                                  __pthread_internal_list{__prev:
                                                                                              0
                                                                                                  as
                                                                                                  *const __pthread_internal_list
                                                                                                  as
                                                                                                  *mut __pthread_internal_list,
                                                                                          __next:
                                                                                              0
                                                                                                  as
                                                                                                  *const __pthread_internal_list
                                                                                                  as
                                                                                                  *mut __pthread_internal_list,};
                                                              init
                                                          },};
                                init
                            },};
    static mut retbuffer_key: pthread_key_t = 0;
    static mut firstrun: libc::c_int = 1 as libc::c_int;
    static mut retbuffer: *mut wchar_t = 0 as *const wchar_t as *mut wchar_t;
    pthread_mutex_lock(&mut mtx);
    pthread_mutex_lock(&mut (*f).mtx);
    if firstrun != 0 {
        pthread_key_create(&mut retbuffer_key,
                           Some(free as
                                    unsafe extern "C" fn(_: *mut libc::c_void)
                                        -> ()));
        firstrun = 0 as libc::c_int
    }
    retbuffer = pthread_getspecific(retbuffer_key) as *mut wchar_t;
    if !retbuffer.is_null() { free(retbuffer as *mut libc::c_void); }
    let w =
        if !name.is_null() && *name != 0 {
            stfl_widget_by_name((*f).root, name)
        } else { (*f).root };
    retbuffer = stfl_widget_text(w);
    pthread_setspecific(retbuffer_key, retbuffer as *const libc::c_void);
    pthread_mutex_unlock(&mut (*f).mtx);
    pthread_mutex_unlock(&mut mtx);
    return checkret(retbuffer);
}
unsafe extern "C" fn stfl_modify_before(w: *mut stfl_widget,
                                        mut n: *mut stfl_widget) {
    if n.is_null() || w.is_null() || (*w).parent.is_null() { return }
    let mut prev_p: *mut *mut stfl_widget = &mut (*(*w).parent).first_child;
    let mut last_n: *mut stfl_widget = 0 as *mut stfl_widget;
    while *prev_p != w { prev_p = &mut (**prev_p).next_sibling }
    *prev_p = n;
    while !n.is_null() {
        last_n = n;
        (*n).parent = (*w).parent;
        n = (*n).next_sibling
    }
    (*last_n).next_sibling = w;
}
unsafe extern "C" fn stfl_modify_after(mut w: *mut stfl_widget,
                                       mut n: *mut stfl_widget) {
    if n.is_null() || w.is_null() || (*w).parent.is_null() { return }
    let first_n: *mut stfl_widget = n;
    let mut last_n: *mut stfl_widget = 0 as *mut stfl_widget;
    while !n.is_null() {
        last_n = n;
        (*n).parent = (*w).parent;
        n = (*n).next_sibling
    }
    if !(*w).next_sibling.is_null() {
        (*last_n).next_sibling = (*w).next_sibling
    } else { (*(*w).parent).last_child = last_n }
    (*w).next_sibling = first_n;
}
unsafe extern "C" fn stfl_modify_insert(mut w: *mut stfl_widget,
                                        mut n: *mut stfl_widget) {
    if n.is_null() || w.is_null() { return }
    let first_n: *mut stfl_widget = n;
    let mut last_n: *mut stfl_widget = 0 as *mut stfl_widget;
    while !n.is_null() { last_n = n; (*n).parent = w; n = (*n).next_sibling }
    if !(*w).first_child.is_null() {
        (*last_n).next_sibling = (*w).first_child
    } else { (*w).last_child = last_n }
    (*w).first_child = first_n;
}
unsafe extern "C" fn stfl_modify_append(mut w: *mut stfl_widget,
                                        mut n: *mut stfl_widget) {
    if n.is_null() || w.is_null() { return }
    let first_n: *mut stfl_widget = n;
    let mut last_n: *mut stfl_widget = 0 as *mut stfl_widget;
    while !n.is_null() { last_n = n; (*n).parent = w; n = (*n).next_sibling }
    if !(*w).last_child.is_null() {
        (*(*w).last_child).next_sibling = first_n
    } else { (*w).first_child = first_n }
    (*w).last_child = last_n;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_modify(mut f: *mut stfl_form,
                                     name: *const wchar_t,
                                     mut mode: *const wchar_t,
                                     text: *const wchar_t) {
    pthread_mutex_lock(&mut (*f).mtx);
    let w = stfl_widget_by_name((*f).root, if !name.is_null() {
        name
    } else {
        (*::std::mem::transmute::<&[u8; 4], &[libc::c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr()
    });
    if !w.is_null() {
        mode =
            if !mode.is_null() {
                mode
            } else {
                (*::std::mem::transmute::<&[u8; 4],
                                          &[libc::c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr()
            };
        if wcscmp(mode,
                  (*::std::mem::transmute::<&[u8; 28],
                                            &[libc::c_int; 7]>(b"d\x00\x00\x00e\x00\x00\x00l\x00\x00\x00e\x00\x00\x00t\x00\x00\x00e\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
               == 0 && w != (*f).root {
            stfl_widget_free(w);
        } else {
            let mut n =
                stfl_parser(if !text.is_null() {
                                text
                            } else {
                                (*::std::mem::transmute::<&[u8; 4],
                                                          &[libc::c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr()
                            });
            if !n.is_null() {
                if wcscmp(mode,
                          (*::std::mem::transmute::<&[u8; 32],
                                                    &[libc::c_int; 8]>(b"r\x00\x00\x00e\x00\x00\x00p\x00\x00\x00l\x00\x00\x00a\x00\x00\x00c\x00\x00\x00e\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                       == 0 {
                    if w == (*f).root {
                        (*f).root = n
                    } else { stfl_modify_after(w, n); }
                    stfl_widget_free(w);
                } else if wcscmp(mode,
                                 (*::std::mem::transmute::<&[u8; 56],
                                                           &[libc::c_int; 14]>(b"r\x00\x00\x00e\x00\x00\x00p\x00\x00\x00l\x00\x00\x00a\x00\x00\x00c\x00\x00\x00e\x00\x00\x00_\x00\x00\x00i\x00\x00\x00n\x00\x00\x00n\x00\x00\x00e\x00\x00\x00r\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                              == 0 {
                    while !(*w).first_child.is_null() {
                        stfl_widget_free((*w).first_child);
                    }
                    stfl_modify_insert(w, (*n).first_child);
                    (*n).last_child = 0 as *mut stfl_widget;
                    (*n).first_child = (*n).last_child;
                    stfl_widget_free(n);
                    n = w
                } else if wcscmp(mode,
                                 (*::std::mem::transmute::<&[u8; 28],
                                                           &[libc::c_int; 7]>(b"i\x00\x00\x00n\x00\x00\x00s\x00\x00\x00e\x00\x00\x00r\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                              == 0 {
                    stfl_modify_insert(w, n);
                } else if wcscmp(mode,
                                 (*::std::mem::transmute::<&[u8; 52],
                                                           &[libc::c_int; 13]>(b"i\x00\x00\x00n\x00\x00\x00s\x00\x00\x00e\x00\x00\x00r\x00\x00\x00t\x00\x00\x00_\x00\x00\x00i\x00\x00\x00n\x00\x00\x00n\x00\x00\x00e\x00\x00\x00r\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                              == 0 {
                    stfl_modify_insert(w, (*n).first_child);
                    (*n).last_child = 0 as *mut stfl_widget;
                    (*n).first_child = (*n).last_child;
                    stfl_widget_free(n);
                    n = w
                } else if wcscmp(mode,
                                 (*::std::mem::transmute::<&[u8; 28],
                                                           &[libc::c_int; 7]>(b"a\x00\x00\x00p\x00\x00\x00p\x00\x00\x00e\x00\x00\x00n\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                              == 0 {
                    stfl_modify_append(w, n);
                } else if wcscmp(mode,
                                 (*::std::mem::transmute::<&[u8; 52],
                                                           &[libc::c_int; 13]>(b"a\x00\x00\x00p\x00\x00\x00p\x00\x00\x00e\x00\x00\x00n\x00\x00\x00d\x00\x00\x00_\x00\x00\x00i\x00\x00\x00n\x00\x00\x00n\x00\x00\x00e\x00\x00\x00r\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                              == 0 {
                    stfl_modify_append(w, (*n).first_child);
                    (*n).last_child = 0 as *mut stfl_widget;
                    (*n).first_child = (*n).last_child;
                    stfl_widget_free(n);
                    n = w
                } else if wcscmp(mode,
                                 (*::std::mem::transmute::<&[u8; 28],
                                                           &[libc::c_int; 7]>(b"b\x00\x00\x00e\x00\x00\x00f\x00\x00\x00o\x00\x00\x00r\x00\x00\x00e\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                              == 0 {
                    stfl_modify_before(w, n);
                } else if wcscmp(mode,
                                 (*::std::mem::transmute::<&[u8; 52],
                                                           &[libc::c_int; 13]>(b"b\x00\x00\x00e\x00\x00\x00f\x00\x00\x00o\x00\x00\x00r\x00\x00\x00e\x00\x00\x00_\x00\x00\x00i\x00\x00\x00n\x00\x00\x00n\x00\x00\x00e\x00\x00\x00r\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                              == 0 {
                    stfl_modify_before(w, (*n).first_child);
                    (*n).last_child = 0 as *mut stfl_widget;
                    (*n).first_child = (*n).last_child;
                    stfl_widget_free(n);
                    n = w
                } else if wcscmp(mode,
                                 (*::std::mem::transmute::<&[u8; 24],
                                                           &[libc::c_int; 6]>(b"a\x00\x00\x00f\x00\x00\x00t\x00\x00\x00e\x00\x00\x00r\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                              == 0 {
                    stfl_modify_after(w, n);
                } else if wcscmp(mode,
                                 (*::std::mem::transmute::<&[u8; 48],
                                                           &[libc::c_int; 12]>(b"a\x00\x00\x00f\x00\x00\x00t\x00\x00\x00e\x00\x00\x00r\x00\x00\x00_\x00\x00\x00i\x00\x00\x00n\x00\x00\x00n\x00\x00\x00e\x00\x00\x00r\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                              == 0 {
                    stfl_modify_after(w, (*n).first_child);
                    (*n).last_child = 0 as *mut stfl_widget;
                    (*n).first_child = (*n).last_child;
                    stfl_widget_free(n);
                    n = w
                }
                stfl_check_setfocus(f, n);
            }
        }
    }
    pthread_mutex_unlock(&mut (*f).mtx);
}
