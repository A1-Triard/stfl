use ::libc;
extern "C" {
    pub type ldat;
    #[no_mangle]
    static mut acs_map: [chtype; 0];
    #[no_mangle]
    fn waddch(_: *mut WINDOW, _: chtype) -> libc::c_int;
    #[no_mangle]
    fn whline(_: *mut WINDOW, _: chtype, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn wmove(_: *mut WINDOW, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn wvline(_: *mut WINDOW, _: chtype, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn wcschr(_: *const libc::c_int, _: libc::c_int) -> *mut libc::c_int;
    #[no_mangle]
    fn wcscmp(_: *const libc::c_int, _: *const libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn stfl_switch_focus(old_fw: *mut stfl_widget, new_fw: *mut stfl_widget,
                         f: *mut stfl_form) -> libc::c_int;
    #[no_mangle]
    fn stfl_find_first_focusable(w: *mut stfl_widget) -> *mut stfl_widget;
    #[no_mangle]
    fn stfl_find_child_tree(w: *mut stfl_widget, c: *mut stfl_widget)
     -> *mut stfl_widget;
    #[no_mangle]
    fn stfl_matchbind(w: *mut stfl_widget, ch: wchar_t,
                      isfunckey: libc::c_int, name: *mut wchar_t,
                      auto_desc: *mut wchar_t) -> libc::c_int;
    #[no_mangle]
    fn stfl_widget_style(w: *mut stfl_widget, f: *mut stfl_form,
                         win: *mut WINDOW);
    #[no_mangle]
    fn stfl_widget_getkv_str(w: *mut stfl_widget, key: *const wchar_t,
                             defval: *const wchar_t) -> *const wchar_t;
    #[no_mangle]
    fn stfl_widget_getkv_int(w: *mut stfl_widget, key: *const wchar_t,
                             defval: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_cell_data {
    pub w: *mut stfl_widget,
    pub mastercell: *mut table_cell_data,
    pub vexpand: libc::c_uchar,
    pub hexpand: libc::c_uchar,
    pub spanpadding: libc::c_uchar,
    pub colspan_nr: libc::c_uchar,
    pub rowspan_nr: libc::c_uchar,
    pub colspan: libc::c_uchar,
    pub rowspan: libc::c_uchar,
    pub mc_border_l: libc::c_uchar,
    pub mc_border_r: libc::c_uchar,
    pub mc_border_t: libc::c_uchar,
    pub mc_border_b: libc::c_uchar,
    pub border_l: libc::c_uchar,
    pub border_r: libc::c_uchar,
    pub border_t: libc::c_uchar,
    pub border_b: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_data {
    pub rows: libc::c_int,
    pub cols: libc::c_int,
    pub map: [[*mut table_cell_data; 30]; 30],
    pub rowd: *mut table_rowcol_data,
    pub cold: *mut table_rowcol_data,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_rowcol_data {
    pub min: libc::c_uchar,
    pub size: libc::c_uchar,
    pub expand: libc::c_uchar,
}
unsafe extern "C" fn free_table_data(mut d: *mut table_data) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 30 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 30 as libc::c_int {
            if !(*d).map[i as usize][j as usize].is_null() {
                free((*d).map[i as usize][j as usize] as *mut libc::c_void);
            }
            j += 1
        }
        i += 1
    }
    free((*d).rowd as *mut libc::c_void);
    free((*d).cold as *mut libc::c_void);
    free(d as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn max(mut a: libc::c_int, mut b: libc::c_int)
 -> libc::c_int {
    return if a > b { a } else { b };
}
unsafe extern "C" fn wt_table_done(mut w: *mut stfl_widget) {
    if !(*w).internal_data.is_null() {
        free_table_data((*w).internal_data as *mut table_data);
    };
}
unsafe extern "C" fn wt_table_prepare(mut w: *mut stfl_widget,
                                      mut f: *mut stfl_form) {
    let mut d: *mut table_data =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<table_data>() as libc::c_ulong) as
            *mut table_data;
    if !(*w).internal_data.is_null() {
        free_table_data((*w).internal_data as *mut table_data);
    }
    (*w).internal_data = d as *mut libc::c_void;
    (*d).rows = 1 as libc::c_int;
    let mut col_counter: libc::c_int = 0 as libc::c_int;
    let mut row_counter: libc::c_int = 0 as libc::c_int;
    let mut max_colspan: libc::c_int = 0 as libc::c_int;
    let mut max_rowspan: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut c: *mut stfl_widget = (*w).first_child;
    while !c.is_null() {
        if wcscmp((*(*c).type_0).name,
                  (*::std::mem::transmute::<&[u8; 32],
                                            &[libc::c_int; 8]>(b"t\x00\x00\x00a\x00\x00\x00b\x00\x00\x00l\x00\x00\x00e\x00\x00\x00b\x00\x00\x00r\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
               == 0 {
            if !(*c).next_sibling.is_null() { row_counter += 1 }
            col_counter = 0 as libc::c_int
        } else {
            while !(*d).map[col_counter as
                                usize][row_counter as usize].is_null() {
                col_counter += 1
            }
            if col_counter < 30 as libc::c_int &&
                   row_counter < 30 as libc::c_int {
            } else {
                __assert_fail(b"col_counter < MAX_COLS && row_counter < MAX_ROWS\x00"
                                  as *const u8 as *const libc::c_char,
                              b"widgets/wt_table.c\x00" as *const u8 as
                                  *const libc::c_char,
                              105 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 64],
                                                        &[libc::c_char; 64]>(b"void wt_table_prepare(struct stfl_widget *, struct stfl_form *)\x00")).as_ptr());
            }
            let mut colspan: libc::c_int =
                stfl_widget_getkv_int(c,
                                      (*::std::mem::transmute::<&[u8; 36],
                                                                &[libc::c_int; 9]>(b".\x00\x00\x00c\x00\x00\x00o\x00\x00\x00l\x00\x00\x00s\x00\x00\x00p\x00\x00\x00a\x00\x00\x00n\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                      1 as libc::c_int);
            let mut rowspan: libc::c_int =
                stfl_widget_getkv_int(c,
                                      (*::std::mem::transmute::<&[u8; 36],
                                                                &[libc::c_int; 9]>(b".\x00\x00\x00r\x00\x00\x00o\x00\x00\x00w\x00\x00\x00s\x00\x00\x00p\x00\x00\x00a\x00\x00\x00n\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                      1 as libc::c_int);
            max_colspan = max(max_colspan, colspan);
            max_rowspan = max(max_rowspan, rowspan);
            (*d).cols = max((*d).cols, col_counter + colspan);
            (*d).rows = max((*d).rows, row_counter + rowspan);
            let mut expand: *const wchar_t =
                stfl_widget_getkv_str(c,
                                      (*::std::mem::transmute::<&[u8; 32],
                                                                &[libc::c_int; 8]>(b".\x00\x00\x00e\x00\x00\x00x\x00\x00\x00p\x00\x00\x00a\x00\x00\x00n\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                      (*::std::mem::transmute::<&[u8; 12],
                                                                &[libc::c_int; 3]>(b"v\x00\x00\x00h\x00\x00\x00\x00\x00\x00\x00")).as_ptr());
            let mut spacer: *const wchar_t =
                stfl_widget_getkv_str(c,
                                      (*::std::mem::transmute::<&[u8; 32],
                                                                &[libc::c_int; 8]>(b".\x00\x00\x00s\x00\x00\x00p\x00\x00\x00a\x00\x00\x00c\x00\x00\x00e\x00\x00\x00r\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                      (*::std::mem::transmute::<&[u8; 4],
                                                                &[libc::c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr());
            let mut border: *const wchar_t =
                stfl_widget_getkv_str(c,
                                      (*::std::mem::transmute::<&[u8; 32],
                                                                &[libc::c_int; 8]>(b".\x00\x00\x00b\x00\x00\x00o\x00\x00\x00r\x00\x00\x00d\x00\x00\x00e\x00\x00\x00r\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                      (*::std::mem::transmute::<&[u8; 4],
                                                                &[libc::c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr());
            i = col_counter;
            while i < col_counter + colspan {
                j = row_counter;
                while j < row_counter + rowspan {
                    (*d).map[i as usize][j as usize] =
                        calloc(1 as libc::c_int as libc::c_ulong,
                               ::std::mem::size_of::<table_cell_data>() as
                                   libc::c_ulong) as *mut table_cell_data;
                    (*(*d).map[i as usize][j as usize]).mastercell =
                        (*d).map[col_counter as usize][row_counter as usize];
                    if i != col_counter || j != row_counter {
                        (*(*d).map[i as usize][j as usize]).spanpadding =
                            1 as libc::c_int as libc::c_uchar
                    }
                    (*(*d).map[i as usize][j as usize]).colspan_nr =
                        (i - col_counter) as libc::c_uchar;
                    (*(*d).map[i as usize][j as usize]).rowspan_nr =
                        (j - row_counter) as libc::c_uchar;
                    (*(*d).map[i as usize][j as usize]).vexpand =
                        (wcschr(expand, 'v' as i32) != 0 as *mut libc::c_int)
                            as libc::c_int as libc::c_uchar;
                    (*(*d).map[i as usize][j as usize]).hexpand =
                        (wcschr(expand, 'h' as i32) != 0 as *mut libc::c_int)
                            as libc::c_int as libc::c_uchar;
                    if i == col_counter {
                        if !wcschr(spacer, 'l' as i32).is_null() {
                            (*(*d).map[i as usize][j as usize]).border_l =
                                1 as libc::c_int as libc::c_uchar
                        }
                        if !wcschr(border, 'l' as i32).is_null() {
                            (*(*d).map[i as usize][j as usize]).border_l =
                                2 as libc::c_int as libc::c_uchar
                        }
                    }
                    if i == col_counter + colspan - 1 as libc::c_int {
                        if !wcschr(spacer, 'r' as i32).is_null() {
                            (*(*d).map[i as usize][j as usize]).border_r =
                                1 as libc::c_int as libc::c_uchar
                        }
                        if !wcschr(border, 'r' as i32).is_null() {
                            (*(*d).map[i as usize][j as usize]).border_r =
                                2 as libc::c_int as libc::c_uchar
                        }
                    }
                    if j == row_counter {
                        if !wcschr(spacer, 't' as i32).is_null() {
                            (*(*d).map[i as usize][j as usize]).border_t =
                                1 as libc::c_int as libc::c_uchar
                        }
                        if !wcschr(border, 't' as i32).is_null() {
                            (*(*d).map[i as usize][j as usize]).border_t =
                                2 as libc::c_int as libc::c_uchar
                        }
                    }
                    if j == row_counter + rowspan - 1 as libc::c_int {
                        if !wcschr(spacer, 'b' as i32).is_null() {
                            (*(*d).map[i as usize][j as usize]).border_b =
                                1 as libc::c_int as libc::c_uchar
                        }
                        if !wcschr(border, 'b' as i32).is_null() {
                            (*(*d).map[i as usize][j as usize]).border_b =
                                2 as libc::c_int as libc::c_uchar
                        }
                    }
                    if i > 0 as libc::c_int &&
                           !(*d).map[(i - 1 as libc::c_int) as
                                         usize][j as usize].is_null() {
                        (*(*d).map[i as usize][j as usize]).border_l =
                            max((*(*d).map[(i - 1 as libc::c_int) as
                                               usize][j as usize]).border_r as
                                    libc::c_int,
                                (*(*d).map[i as usize][j as usize]).border_l
                                    as libc::c_int) as libc::c_uchar;
                        (*(*d).map[(i - 1 as libc::c_int) as
                                       usize][j as usize]).border_r =
                            (*(*d).map[i as usize][j as usize]).border_l
                    }
                    if j > 0 as libc::c_int &&
                           !(*d).map[i as
                                         usize][(j - 1 as libc::c_int) as
                                                    usize].is_null() {
                        (*(*d).map[i as usize][j as usize]).border_t =
                            max((*(*d).map[i as
                                               usize][(j - 1 as libc::c_int)
                                                          as usize]).border_b
                                    as libc::c_int,
                                (*(*d).map[i as usize][j as usize]).border_t
                                    as libc::c_int) as libc::c_uchar;
                        (*(*d).map[i as
                                       usize][(j - 1 as libc::c_int) as
                                                  usize]).border_b =
                            (*(*d).map[i as usize][j as usize]).border_t
                    }
                    (*(*d).map[i as usize][j as usize]).colspan =
                        colspan as libc::c_uchar;
                    (*(*d).map[i as usize][j as usize]).rowspan =
                        rowspan as libc::c_uchar;
                    (*(*d).map[i as usize][j as usize]).w = c;
                    j += 1
                }
                i += 1
            }
            col_counter += colspan
        }
        (*(*c).type_0).f_prepare.expect("non-null function pointer")(c, f);
        c = (*c).next_sibling
    }
    (*d).rowd =
        calloc((*d).rows as libc::c_ulong,
               ::std::mem::size_of::<table_rowcol_data>() as libc::c_ulong) as
            *mut table_rowcol_data;
    (*d).cold =
        calloc((*d).cols as libc::c_ulong,
               ::std::mem::size_of::<table_rowcol_data>() as libc::c_ulong) as
            *mut table_rowcol_data;
    i = 1 as libc::c_int;
    while i <= max_colspan {
        row_counter = 0 as libc::c_int;
        while row_counter < (*d).rows {
            col_counter = 0 as libc::c_int;
            while col_counter < (*d).cols {
                if !((*d).map[col_counter as
                                  usize][row_counter as usize].is_null() ||
                         (*(*d).map[col_counter as
                                        usize][row_counter as usize]).hexpand
                             as libc::c_int == 0 as libc::c_int ||
                         (*(*d).map[col_counter as
                                        usize][row_counter as
                                                   usize]).spanpadding as
                             libc::c_int != 0 ||
                         (*(*d).map[col_counter as
                                        usize][row_counter as usize]).colspan
                             as libc::c_int > i) {
                    let mut expand_ok: libc::c_int = 0 as libc::c_int;
                    j = 0 as libc::c_int;
                    while j <
                              (*(*d).map[col_counter as
                                             usize][row_counter as
                                                        usize]).colspan as
                                  libc::c_int {
                        if (*(*d).cold.offset((col_counter + j) as
                                                  isize)).expand != 0 {
                            expand_ok = 1 as libc::c_int;
                            break ;
                        } else { j += 1 }
                    }
                    if !(expand_ok != 0) {
                        j = 0 as libc::c_int;
                        while j <
                                  (*(*d).map[col_counter as
                                                 usize][row_counter as
                                                            usize]).colspan as
                                      libc::c_int {
                            (*(*d).cold.offset((col_counter + j) as
                                                   isize)).expand =
                                1 as libc::c_int as libc::c_uchar;
                            j += 1
                        }
                    }
                }
                col_counter += 1
            }
            row_counter += 1
        }
        i += 1
    }
    i = 1 as libc::c_int;
    while i <= max_rowspan {
        row_counter = 0 as libc::c_int;
        while row_counter < (*d).rows {
            col_counter = 0 as libc::c_int;
            while col_counter < (*d).cols {
                if !((*d).map[col_counter as
                                  usize][row_counter as usize].is_null() ||
                         (*(*d).map[col_counter as
                                        usize][row_counter as usize]).vexpand
                             as libc::c_int == 0 as libc::c_int ||
                         (*(*d).map[col_counter as
                                        usize][row_counter as
                                                   usize]).spanpadding as
                             libc::c_int != 0 ||
                         (*(*d).map[col_counter as
                                        usize][row_counter as usize]).rowspan
                             as libc::c_int > i) {
                    let mut expand_ok_0: libc::c_int = 0 as libc::c_int;
                    j = 0 as libc::c_int;
                    while j <
                              (*(*d).map[col_counter as
                                             usize][row_counter as
                                                        usize]).rowspan as
                                  libc::c_int {
                        if (*(*d).rowd.offset((row_counter + j) as
                                                  isize)).expand != 0 {
                            expand_ok_0 = 1 as libc::c_int;
                            break ;
                        } else { j += 1 }
                    }
                    if !(expand_ok_0 != 0) {
                        j = 0 as libc::c_int;
                        while j <
                                  (*(*d).map[col_counter as
                                                 usize][row_counter as
                                                            usize]).rowspan as
                                      libc::c_int {
                            (*(*d).rowd.offset((row_counter + j) as
                                                   isize)).expand =
                                1 as libc::c_int as libc::c_uchar;
                            j += 1
                        }
                    }
                }
                col_counter += 1
            }
            row_counter += 1
        }
        i += 1
    }
    row_counter = 0 as libc::c_int;
    while row_counter < (*d).rows {
        col_counter = 0 as libc::c_int;
        while col_counter < (*d).cols {
            let mut m: *mut table_cell_data =
                (*d).map[col_counter as usize][row_counter as usize];
            if !m.is_null() {
                (*(*m).mastercell).mc_border_l =
                    max((*(*m).mastercell).mc_border_l as libc::c_int,
                        (*m).border_l as libc::c_int) as libc::c_uchar;
                (*(*m).mastercell).mc_border_r =
                    max((*(*m).mastercell).mc_border_r as libc::c_int,
                        (*m).border_r as libc::c_int) as libc::c_uchar;
                (*(*m).mastercell).mc_border_t =
                    max((*(*m).mastercell).mc_border_t as libc::c_int,
                        (*m).border_t as libc::c_int) as libc::c_uchar;
                (*(*m).mastercell).mc_border_b =
                    max((*(*m).mastercell).mc_border_b as libc::c_int,
                        (*m).border_b as libc::c_int) as libc::c_uchar
            }
            col_counter += 1
        }
        row_counter += 1
    }
    i = 1 as libc::c_int;
    while i <= max_colspan {
        row_counter = 0 as libc::c_int;
        while row_counter < (*d).rows {
            col_counter = 0 as libc::c_int;
            while col_counter < (*d).cols {
                let mut m_0: *mut table_cell_data =
                    (*d).map[col_counter as usize][row_counter as usize];
                if !(m_0.is_null() || (*m_0).spanpadding as libc::c_int != 0
                         || (*m_0).colspan as libc::c_int > i) {
                    let mut min_w: libc::c_int =
                        max((*(*m_0).w).min_w,
                            stfl_widget_getkv_int((*m_0).w,
                                                  (*::std::mem::transmute::<&[u8; 28],
                                                                            &[libc::c_int; 7]>(b".\x00\x00\x00w\x00\x00\x00i\x00\x00\x00d\x00\x00\x00t\x00\x00\x00h\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                                  1 as libc::c_int));
                    if col_counter == 0 as libc::c_int &&
                           (*m_0).mc_border_l as libc::c_int != 0 {
                        min_w += 3 as libc::c_int
                    }
                    if (*m_0).mc_border_r != 0 { min_w += 3 as libc::c_int }
                    let mut total: libc::c_int = min_w;
                    j = 0 as libc::c_int;
                    while j < (*m_0).colspan as libc::c_int {
                        total -=
                            (*(*d).cold.offset((col_counter + j) as
                                                   isize)).min as libc::c_int;
                        j += 1
                    }
                    if !(total <= 0 as libc::c_int) {
                        let mut expandables: libc::c_int = 0 as libc::c_int;
                        j = 0 as libc::c_int;
                        while j < (*m_0).colspan as libc::c_int {
                            if (*(*d).cold.offset((col_counter + j) as
                                                      isize)).expand != 0 {
                                expandables += 1
                            }
                            j += 1
                        }
                        if expandables > 0 as libc::c_int {
                            let mut per: libc::c_int = total / expandables;
                            let mut extra_per: libc::c_int =
                                total % expandables;
                            j = 0 as libc::c_int;
                            while j < (*m_0).colspan as libc::c_int {
                                if (*(*d).cold.offset((col_counter + j) as
                                                          isize)).expand != 0
                                   {
                                    let ref mut fresh0 =
                                        (*(*d).cold.offset((col_counter + j)
                                                               as isize)).min;
                                    *fresh0 =
                                        (*fresh0 as libc::c_int + per) as
                                            libc::c_uchar;
                                    if extra_per != 0 {
                                        let ref mut fresh1 =
                                            (*(*d).cold.offset((col_counter +
                                                                    j) as
                                                                   isize)).min;
                                        *fresh1 = (*fresh1).wrapping_add(1);
                                        extra_per -= 1
                                    }
                                }
                                j += 1
                            }
                        } else {
                            let mut per_0: libc::c_int =
                                total / (*m_0).colspan as libc::c_int;
                            let mut extra_per_0: libc::c_int =
                                total % (*m_0).colspan as libc::c_int;
                            j = 0 as libc::c_int;
                            while j < (*m_0).colspan as libc::c_int {
                                let ref mut fresh2 =
                                    (*(*d).cold.offset((col_counter + j) as
                                                           isize)).min;
                                *fresh2 =
                                    (*fresh2 as libc::c_int + per_0) as
                                        libc::c_uchar;
                                if extra_per_0 != 0 {
                                    let ref mut fresh3 =
                                        (*(*d).cold.offset((col_counter + j)
                                                               as isize)).min;
                                    *fresh3 = (*fresh3).wrapping_add(1);
                                    extra_per_0 -= 1
                                }
                                j += 1
                            }
                        }
                    }
                }
                col_counter += 1
            }
            row_counter += 1
        }
        i += 1
    }
    i = 1 as libc::c_int;
    while i <= max_rowspan {
        row_counter = 0 as libc::c_int;
        while row_counter < (*d).rows {
            col_counter = 0 as libc::c_int;
            while col_counter < (*d).cols {
                let mut m_1: *mut table_cell_data =
                    (*d).map[col_counter as usize][row_counter as usize];
                if !(m_1.is_null() || (*m_1).spanpadding as libc::c_int != 0
                         || (*m_1).rowspan as libc::c_int > i) {
                    let mut min_h: libc::c_int =
                        max((*(*m_1).w).min_h,
                            stfl_widget_getkv_int((*m_1).w,
                                                  (*::std::mem::transmute::<&[u8; 32],
                                                                            &[libc::c_int; 8]>(b".\x00\x00\x00h\x00\x00\x00e\x00\x00\x00i\x00\x00\x00g\x00\x00\x00h\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                                  1 as libc::c_int));
                    if row_counter == 0 as libc::c_int &&
                           (*m_1).mc_border_t as libc::c_int != 0 {
                        min_h += 1
                    }
                    if (*m_1).mc_border_b != 0 { min_h += 1 }
                    let mut total_0: libc::c_int = min_h;
                    j = 0 as libc::c_int;
                    while j < (*m_1).rowspan as libc::c_int {
                        total_0 -=
                            (*(*d).rowd.offset((row_counter + j) as
                                                   isize)).min as libc::c_int;
                        j += 1
                    }
                    if !(total_0 <= 0 as libc::c_int) {
                        let mut expandables_0: libc::c_int = 0 as libc::c_int;
                        j = 0 as libc::c_int;
                        while j < (*m_1).rowspan as libc::c_int {
                            if (*(*d).rowd.offset((row_counter + j) as
                                                      isize)).expand != 0 {
                                expandables_0 += 1
                            }
                            j += 1
                        }
                        if expandables_0 > 0 as libc::c_int {
                            let mut per_1: libc::c_int =
                                total_0 / expandables_0;
                            let mut extra_per_1: libc::c_int =
                                total_0 % expandables_0;
                            j = 0 as libc::c_int;
                            while j < (*m_1).rowspan as libc::c_int {
                                if (*(*d).rowd.offset((row_counter + j) as
                                                          isize)).expand != 0
                                   {
                                    let ref mut fresh4 =
                                        (*(*d).rowd.offset((row_counter + j)
                                                               as isize)).min;
                                    *fresh4 =
                                        (*fresh4 as libc::c_int + per_1) as
                                            libc::c_uchar;
                                    if extra_per_1 != 0 {
                                        let ref mut fresh5 =
                                            (*(*d).rowd.offset((row_counter +
                                                                    j) as
                                                                   isize)).min;
                                        *fresh5 = (*fresh5).wrapping_add(1);
                                        extra_per_1 -= 1
                                    }
                                }
                                j += 1
                            }
                        } else {
                            let mut per_2: libc::c_int =
                                total_0 / (*m_1).rowspan as libc::c_int;
                            let mut extra_per_2: libc::c_int =
                                total_0 % (*m_1).rowspan as libc::c_int;
                            j = 0 as libc::c_int;
                            while j < (*m_1).rowspan as libc::c_int {
                                let ref mut fresh6 =
                                    (*(*d).rowd.offset((row_counter + j) as
                                                           isize)).min;
                                *fresh6 =
                                    (*fresh6 as libc::c_int + per_2) as
                                        libc::c_uchar;
                                if extra_per_2 != 0 {
                                    let ref mut fresh7 =
                                        (*(*d).rowd.offset((row_counter + j)
                                                               as isize)).min;
                                    *fresh7 = (*fresh7).wrapping_add(1);
                                    extra_per_2 -= 1
                                }
                                j += 1
                            }
                        }
                    }
                }
                col_counter += 1
            }
            row_counter += 1
        }
        i += 1
    }
    (*w).min_w = 0 as libc::c_int;
    (*w).min_h = (*w).min_w;
    row_counter = 0 as libc::c_int;
    while row_counter < (*d).rows {
        (*w).min_h +=
            (*(*d).rowd.offset(row_counter as isize)).min as libc::c_int;
        row_counter += 1
    }
    col_counter = 0 as libc::c_int;
    while col_counter < (*d).cols {
        (*w).min_w +=
            (*(*d).cold.offset(col_counter as isize)).min as libc::c_int;
        col_counter += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn make_corner(mut win: *mut WINDOW, mut x: libc::c_int,
                                     mut y: libc::c_int,
                                     mut left: libc::c_int,
                                     mut right: libc::c_int,
                                     mut up: libc::c_int,
                                     mut down: libc::c_int) {
    match (if left != 0 { 0o1000 as libc::c_int } else { 0 as libc::c_int }) |
              (if right != 0 {
                   0o100 as libc::c_int
               } else { 0 as libc::c_int }) |
              (if up != 0 { 0o10 as libc::c_int } else { 0 as libc::c_int }) |
              (if down != 0 { 0o1 as libc::c_int } else { 0 as libc::c_int })
        {
        1 => {
            // LEFT-RIGHT-UP-DOWN
            if wmove(win, y, x) == -(1 as libc::c_int) {
            } else {
                waddch(win,
                       *acs_map.as_mut_ptr().offset('x' as i32 as
                                                        libc::c_uchar as
                                                        isize));
            };
        }
        8 => {
            // LEFT-RIGHT-UP-DOWN
            if wmove(win, y, x) == -(1 as libc::c_int) {
            } else {
                waddch(win,
                       *acs_map.as_mut_ptr().offset('x' as i32 as
                                                        libc::c_uchar as
                                                        isize));
            };
        }
        9 => {
            // LEFT-RIGHT-UP-DOWN
            if wmove(win, y, x) == -(1 as libc::c_int) {
            } else {
                waddch(win,
                       *acs_map.as_mut_ptr().offset('x' as i32 as
                                                        libc::c_uchar as
                                                        isize));
            };
        }
        64 => {
            // LEFT-RIGHT-UP-DOWN
            if wmove(win, y, x) == -(1 as libc::c_int) {
            } else {
                waddch(win,
                       *acs_map.as_mut_ptr().offset('q' as i32 as
                                                        libc::c_uchar as
                                                        isize));
            };
        }
        65 => {
            // LEFT-RIGHT-UP-DOWN
            if wmove(win, y, x) == -(1 as libc::c_int) {
            } else {
                waddch(win,
                       *acs_map.as_mut_ptr().offset('l' as i32 as
                                                        libc::c_uchar as
                                                        isize));
            };
        }
        72 => {
            // LEFT-RIGHT-UP-DOWN
            if wmove(win, y, x) == -(1 as libc::c_int) {
            } else {
                waddch(win,
                       *acs_map.as_mut_ptr().offset('m' as i32 as
                                                        libc::c_uchar as
                                                        isize));
            };
        }
        73 => {
            // LEFT-RIGHT-UP-DOWN
            if wmove(win, y, x) == -(1 as libc::c_int) {
            } else {
                waddch(win,
                       *acs_map.as_mut_ptr().offset('t' as i32 as
                                                        libc::c_uchar as
                                                        isize));
            };
        }
        512 => {
            // LEFT-RIGHT-UP-DOWN
            if wmove(win, y, x) == -(1 as libc::c_int) {
            } else {
                waddch(win,
                       *acs_map.as_mut_ptr().offset('q' as i32 as
                                                        libc::c_uchar as
                                                        isize));
            };
        }
        513 => {
            // LEFT-RIGHT-UP-DOWN
            if wmove(win, y, x) == -(1 as libc::c_int) {
            } else {
                waddch(win,
                       *acs_map.as_mut_ptr().offset('k' as i32 as
                                                        libc::c_uchar as
                                                        isize));
            };
        }
        520 => {
            // LEFT-RIGHT-UP-DOWN
            if wmove(win, y, x) == -(1 as libc::c_int) {
            } else {
                waddch(win,
                       *acs_map.as_mut_ptr().offset('j' as i32 as
                                                        libc::c_uchar as
                                                        isize));
            };
        }
        521 => {
            // LEFT-RIGHT-UP-DOWN
            if wmove(win, y, x) == -(1 as libc::c_int) {
            } else {
                waddch(win,
                       *acs_map.as_mut_ptr().offset('u' as i32 as
                                                        libc::c_uchar as
                                                        isize));
            };
        }
        576 => {
            // LEFT-RIGHT-UP-DOWN
            if wmove(win, y, x) == -(1 as libc::c_int) {
            } else {
                waddch(win,
                       *acs_map.as_mut_ptr().offset('q' as i32 as
                                                        libc::c_uchar as
                                                        isize));
            };
        }
        577 => {
            // LEFT-RIGHT-UP-DOWN
            if wmove(win, y, x) == -(1 as libc::c_int) {
            } else {
                waddch(win,
                       *acs_map.as_mut_ptr().offset('w' as i32 as
                                                        libc::c_uchar as
                                                        isize));
            };
        }
        584 => {
            // LEFT-RIGHT-UP-DOWN
            if wmove(win, y, x) == -(1 as libc::c_int) {
            } else {
                waddch(win,
                       *acs_map.as_mut_ptr().offset('v' as i32 as
                                                        libc::c_uchar as
                                                        isize));
            };
        }
        585 => {
            // LEFT-RIGHT-UP-DOWN
            if wmove(win, y, x) == -(1 as libc::c_int) {
            } else {
                waddch(win,
                       *acs_map.as_mut_ptr().offset('n' as i32 as
                                                        libc::c_uchar as
                                                        isize));
            };
        }
        0 | _ => { }
    };
}
unsafe extern "C" fn wt_table_draw(mut w: *mut stfl_widget,
                                   mut f: *mut stfl_form,
                                   mut win: *mut WINDOW) {
    let mut d: *mut table_data = (*w).internal_data as *mut table_data;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut extra: libc::c_int = 0;
    let mut extra_counter: libc::c_int = 0;
    extra = (*w).h - (*w).min_h;
    extra_counter = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*d).rows {
        if (*(*d).rowd.offset(i as isize)).expand != 0 { extra_counter += 1 }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < (*d).rows {
        if (*(*d).rowd.offset(i as isize)).expand != 0 {
            let fresh8 = extra_counter;
            extra_counter = extra_counter - 1;
            let mut e: libc::c_int = extra / fresh8;
            (*(*d).rowd.offset(i as isize)).size =
                ((*(*d).rowd.offset(i as isize)).min as libc::c_int + e) as
                    libc::c_uchar;
            extra -= e
        } else {
            (*(*d).rowd.offset(i as isize)).size =
                (*(*d).rowd.offset(i as isize)).min
        }
        i += 1
    }
    extra = (*w).w - (*w).min_w;
    extra_counter = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*d).cols {
        if (*(*d).cold.offset(i as isize)).expand != 0 { extra_counter += 1 }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < (*d).cols {
        if (*(*d).cold.offset(i as isize)).expand != 0 {
            let fresh9 = extra_counter;
            extra_counter = extra_counter - 1;
            let mut e_0: libc::c_int = extra / fresh9;
            (*(*d).cold.offset(i as isize)).size =
                ((*(*d).cold.offset(i as isize)).min as libc::c_int + e_0) as
                    libc::c_uchar;
            extra -= e_0
        } else {
            (*(*d).cold.offset(i as isize)).size =
                (*(*d).cold.offset(i as isize)).min
        }
        i += 1
    }
    let mut y: libc::c_int = (*w).y;
    j = 0 as libc::c_int;
    while j < (*d).rows {
        let mut x: libc::c_int = (*w).x;
        i = 0 as libc::c_int;
        while i < (*d).cols {
            if !(*d).map[i as usize][j as usize].is_null() &&
                   (*(*d).map[i as usize][j as usize]).spanpadding == 0 {
                let mut m: *mut table_cell_data =
                    (*d).map[i as usize][j as usize];
                let mut c: *mut stfl_widget = (*m).w;
                (*c).x = x;
                (*c).w = 0 as libc::c_int;
                (*c).y = y;
                (*c).h = 0 as libc::c_int;
                k = i;
                while k <
                          i +
                              (*(*d).map[i as usize][j as usize]).colspan as
                                  libc::c_int {
                    (*c).w +=
                        (*(*d).cold.offset(k as isize)).size as libc::c_int;
                    k += 1
                }
                k = j;
                while k <
                          j +
                              (*(*d).map[i as usize][j as usize]).rowspan as
                                  libc::c_int {
                    (*c).h +=
                        (*(*d).rowd.offset(k as isize)).size as libc::c_int;
                    k += 1
                }
                if (*m).mc_border_l as libc::c_int != 0 &&
                       i == 0 as libc::c_int {
                    (*c).x += 3 as libc::c_int;
                    (*c).w -= 3 as libc::c_int
                }
                if (*m).mc_border_t as libc::c_int != 0 &&
                       j == 0 as libc::c_int {
                    (*c).y += 1;
                    (*c).h -= 1
                }
                if (*m).mc_border_r != 0 { (*c).w -= 3 as libc::c_int }
                if (*m).mc_border_b != 0 { (*c).h -= 1 }
                let mut tie: *const wchar_t =
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
                (*(*c).type_0).f_draw.expect("non-null function pointer")(c,
                                                                          f,
                                                                          win);
            }
            x += (*(*d).cold.offset(i as isize)).size as libc::c_int;
            i += 1
        }
        y += (*(*d).rowd.offset(j as isize)).size as libc::c_int;
        j += 1
    }
    stfl_widget_style(w, f, win);
    y = (*w).y;
    j = 0 as libc::c_int;
    while j < (*d).rows {
        let mut x_0: libc::c_int = (*w).x;
        i = 0 as libc::c_int;
        while i < (*d).cols {
            if !(*d).map[i as usize][j as usize].is_null() {
                let mut m_0: *mut table_cell_data =
                    (*d).map[i as usize][j as usize];
                let mut box_x: libc::c_int = x_0;
                let mut box_w: libc::c_int =
                    (*(*d).cold.offset(i as isize)).size as libc::c_int;
                let mut box_y: libc::c_int = y;
                let mut box_h: libc::c_int =
                    (*(*d).rowd.offset(j as isize)).size as libc::c_int;
                if i == 0 as libc::c_int {
                    if (*m_0).border_l as libc::c_int > 1 as libc::c_int &&
                           box_h >
                               (if j != 0 {
                                    1 as libc::c_int
                                } else { 2 as libc::c_int }) {
                        wmove(win,
                              box_y +
                                  (if j != 0 {
                                       0 as libc::c_int
                                   } else { 1 as libc::c_int }),
                              box_x + 1 as libc::c_int);
                        wvline(win,
                               *acs_map.as_mut_ptr().offset('x' as i32 as
                                                                libc::c_uchar
                                                                as isize),
                               box_h -
                                   (if j != 0 {
                                        1 as libc::c_int
                                    } else { 2 as libc::c_int }));
                    }
                } else {
                    box_x -= 3 as libc::c_int;
                    box_w += 3 as libc::c_int
                }
                if j == 0 as libc::c_int {
                    if (*m_0).border_t as libc::c_int > 1 as libc::c_int &&
                           box_w > 4 as libc::c_int {
                        wmove(win, box_y, box_x + 2 as libc::c_int);
                        whline(win,
                               *acs_map.as_mut_ptr().offset('q' as i32 as
                                                                libc::c_uchar
                                                                as isize),
                               box_w - 4 as libc::c_int);
                    }
                } else { box_y -= 1; box_h += 1 }
                if (*m_0).border_r as libc::c_int > 1 as libc::c_int &&
                       box_h > 2 as libc::c_int {
                    wmove(win, box_y + 1 as libc::c_int,
                          box_x + box_w - 2 as libc::c_int);
                    wvline(win,
                           *acs_map.as_mut_ptr().offset('x' as i32 as
                                                            libc::c_uchar as
                                                            isize),
                           box_h - 2 as libc::c_int);
                }
                if (*m_0).border_b as libc::c_int > 1 as libc::c_int &&
                       box_w > 4 as libc::c_int {
                    wmove(win, box_y + box_h - 1 as libc::c_int,
                          box_x + 2 as libc::c_int);
                    whline(win,
                           *acs_map.as_mut_ptr().offset('q' as i32 as
                                                            libc::c_uchar as
                                                            isize),
                           box_w - 4 as libc::c_int);
                }
                let mut left: libc::c_int = 0;
                let mut right: libc::c_int = 0;
                let mut up: libc::c_int = 0;
                let mut down: libc::c_int = 0;
                let mut left_m: *mut table_cell_data =
                    if i > 0 as libc::c_int {
                        (*d).map[(i - 1 as libc::c_int) as usize][j as usize]
                    } else { 0 as *mut table_cell_data };
                let mut right_m: *mut table_cell_data =
                    if i < (*d).cols - 1 as libc::c_int {
                        (*d).map[(i + 1 as libc::c_int) as usize][j as usize]
                    } else { 0 as *mut table_cell_data };
                let mut up_m: *mut table_cell_data =
                    if j > 0 as libc::c_int {
                        (*d).map[i as usize][(j - 1 as libc::c_int) as usize]
                    } else { 0 as *mut table_cell_data };
                let mut down_m: *mut table_cell_data =
                    if j < (*d).rows - 1 as libc::c_int {
                        (*d).map[i as usize][(j + 1 as libc::c_int) as usize]
                    } else { 0 as *mut table_cell_data };
                // upper left corner
                if i == 0 as libc::c_int && j == 0 as libc::c_int {
                    left =
                        if !left_m.is_null() {
                            (*left_m).border_t as libc::c_int
                        } else { 0 as libc::c_int };
                    right = (*m_0).border_t as libc::c_int;
                    up =
                        if !up_m.is_null() {
                            (*up_m).border_l as libc::c_int
                        } else { 0 as libc::c_int };
                    down = (*m_0).border_l as libc::c_int;
                    make_corner(win, box_x + 1 as libc::c_int, box_y,
                                (left > 1 as libc::c_int) as libc::c_int,
                                (right > 1 as libc::c_int) as libc::c_int,
                                (up > 1 as libc::c_int) as libc::c_int,
                                (down > 1 as libc::c_int) as libc::c_int);
                }
                // lower left corner
                if i == 0 as libc::c_int {
                    left =
                        if !left_m.is_null() {
                            (*left_m).border_b as libc::c_int
                        } else { 0 as libc::c_int };
                    right = (*m_0).border_b as libc::c_int;
                    up = (*m_0).border_l as libc::c_int;
                    down =
                        if !down_m.is_null() {
                            (*down_m).border_l as libc::c_int
                        } else { 0 as libc::c_int };
                    make_corner(win, box_x + 1 as libc::c_int,
                                box_y + box_h - 1 as libc::c_int,
                                (left > 1 as libc::c_int) as libc::c_int,
                                (right > 1 as libc::c_int) as libc::c_int,
                                (up > 1 as libc::c_int) as libc::c_int,
                                (down > 1 as libc::c_int) as libc::c_int);
                }
                // upper right corner
                if j == 0 as libc::c_int {
                    left = (*m_0).border_t as libc::c_int;
                    right =
                        if !right_m.is_null() {
                            (*right_m).border_t as libc::c_int
                        } else { 0 as libc::c_int };
                    up =
                        if !up_m.is_null() {
                            (*up_m).border_r as libc::c_int
                        } else { 0 as libc::c_int };
                    down = (*m_0).border_r as libc::c_int;
                    make_corner(win, box_x + box_w - 2 as libc::c_int, box_y,
                                (left > 1 as libc::c_int) as libc::c_int,
                                (right > 1 as libc::c_int) as libc::c_int,
                                (up > 1 as libc::c_int) as libc::c_int,
                                (down > 1 as libc::c_int) as libc::c_int);
                }
                // lower right corner
                left = (*m_0).border_b as libc::c_int;
                right =
                    if !right_m.is_null() {
                        (*right_m).border_b as libc::c_int
                    } else { 0 as libc::c_int };
                up = (*m_0).border_r as libc::c_int;
                down =
                    if !down_m.is_null() {
                        (*down_m).border_r as libc::c_int
                    } else { 0 as libc::c_int };
                make_corner(win, box_x + box_w - 2 as libc::c_int,
                            box_y + box_h - 1 as libc::c_int,
                            (left > 1 as libc::c_int) as libc::c_int,
                            (right > 1 as libc::c_int) as libc::c_int,
                            (up > 1 as libc::c_int) as libc::c_int,
                            (down > 1 as libc::c_int) as libc::c_int);
            }
            x_0 += (*(*d).cold.offset(i as isize)).size as libc::c_int;
            i += 1
        }
        y += (*(*d).rowd.offset(j as isize)).size as libc::c_int;
        j += 1
    };
}
unsafe extern "C" fn wt_table_process(mut w: *mut stfl_widget,
                                      mut fw: *mut stfl_widget,
                                      mut f: *mut stfl_form, mut ch: wchar_t,
                                      mut isfunckey: libc::c_int)
 -> libc::c_int {
    let mut d: *mut table_data = (*w).internal_data as *mut table_data;
    let mut n: *mut stfl_widget = 0 as *mut stfl_widget;
    let mut c: *mut stfl_widget = 0 as *mut stfl_widget;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut event: libc::c_int = 0 as libc::c_int;
    if stfl_matchbind(w, ch, isfunckey,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_int; 5]>(b"l\x00\x00\x00e\x00\x00\x00f\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                          as *mut wchar_t,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_int; 5]>(b"L\x00\x00\x00E\x00\x00\x00F\x00\x00\x00T\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                          as *mut wchar_t) != 0 {
        event = 0o404 as libc::c_int
    } else if stfl_matchbind(w, ch, isfunckey,
                             (*::std::mem::transmute::<&[u8; 24],
                                                       &[libc::c_int; 6]>(b"r\x00\x00\x00i\x00\x00\x00g\x00\x00\x00h\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                                 as *mut wchar_t,
                             (*::std::mem::transmute::<&[u8; 24],
                                                       &[libc::c_int; 6]>(b"R\x00\x00\x00I\x00\x00\x00G\x00\x00\x00H\x00\x00\x00T\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                                 as *mut wchar_t) != 0 {
        event = 0o405 as libc::c_int
    } else if stfl_matchbind(w, ch, isfunckey,
                             (*::std::mem::transmute::<&[u8; 12],
                                                       &[libc::c_int; 3]>(b"u\x00\x00\x00p\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                                 as *mut wchar_t,
                             (*::std::mem::transmute::<&[u8; 12],
                                                       &[libc::c_int; 3]>(b"U\x00\x00\x00P\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                                 as *mut wchar_t) != 0 {
        event = 0o403 as libc::c_int
    } else if stfl_matchbind(w, ch, isfunckey,
                             (*::std::mem::transmute::<&[u8; 20],
                                                       &[libc::c_int; 5]>(b"d\x00\x00\x00o\x00\x00\x00w\x00\x00\x00n\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                                 as *mut wchar_t,
                             (*::std::mem::transmute::<&[u8; 20],
                                                       &[libc::c_int; 5]>(b"D\x00\x00\x00O\x00\x00\x00W\x00\x00\x00N\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                                 as *mut wchar_t) != 0 {
        event = 0o402 as libc::c_int
    } else { return 0 as libc::c_int }
    c = stfl_find_child_tree(w, fw);
    j = 0 as libc::c_int;
    while j < (*d).rows {
        i = 0 as libc::c_int;
        while i < (*d).cols {
            let mut m: *mut table_cell_data =
                (*d).map[i as usize][j as usize];
            if !(m.is_null() || (*m).w != c) {
                match event {
                    260 => {
                        k = i - 1 as libc::c_int;
                        while k >= 0 as libc::c_int {
                            m = (*d).map[k as usize][j as usize];
                            if !m.is_null() {
                                n = stfl_find_first_focusable((*m).w);
                                if !n.is_null() {
                                    stfl_switch_focus(fw, n, f);
                                    return 1 as libc::c_int
                                }
                            }
                            k -= 1
                        }
                    }
                    261 => {
                        k = i + 1 as libc::c_int;
                        while k < (*d).cols {
                            m = (*d).map[k as usize][j as usize];
                            if !m.is_null() {
                                n = stfl_find_first_focusable((*m).w);
                                if !n.is_null() {
                                    stfl_switch_focus(fw, n, f);
                                    return 1 as libc::c_int
                                }
                            }
                            k += 1
                        }
                    }
                    259 => {
                        k = j - 1 as libc::c_int;
                        while k >= 0 as libc::c_int {
                            m = (*d).map[i as usize][k as usize];
                            if !m.is_null() {
                                n = stfl_find_first_focusable((*m).w);
                                if !n.is_null() {
                                    stfl_switch_focus(fw, n, f);
                                    return 1 as libc::c_int
                                }
                            }
                            k -= 1
                        }
                    }
                    258 => {
                        k = j + 1 as libc::c_int;
                        while k < (*d).rows {
                            m = (*d).map[i as usize][k as usize];
                            if !m.is_null() {
                                n = stfl_find_first_focusable((*m).w);
                                if !n.is_null() {
                                    stfl_switch_focus(fw, n, f);
                                    return 1 as libc::c_int
                                }
                            }
                            k += 1
                        }
                    }
                    _ => { }
                }
            }
            i += 1
        }
        j += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut stfl_widget_type_table: stfl_widget_type =
    unsafe {
        {
            let mut init =
                stfl_widget_type{name:
                                     (*::std::mem::transmute::<&[u8; 24],
                                                               &[libc::c_int; 6]>(b"t\x00\x00\x00a\x00\x00\x00b\x00\x00\x00l\x00\x00\x00e\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                                         as *mut wchar_t,
                                 f_init: None,
                                 f_done:
                                     Some(wt_table_done as
                                              unsafe extern "C" fn(_:
                                                                       *mut stfl_widget)
                                                  -> ()),
                                 f_enter: None,
                                 f_leave: None,
                                 f_prepare:
                                     Some(wt_table_prepare as
                                              unsafe extern "C" fn(_:
                                                                       *mut stfl_widget,
                                                                   _:
                                                                       *mut stfl_form)
                                                  -> ()),
                                 f_draw:
                                     Some(wt_table_draw as
                                              unsafe extern "C" fn(_:
                                                                       *mut stfl_widget,
                                                                   _:
                                                                       *mut stfl_form,
                                                                   _:
                                                                       *mut WINDOW)
                                                  -> ()),
                                 f_process:
                                     Some(wt_table_process as
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
unsafe extern "C" fn wt_tablebr_prepare(mut w: *mut stfl_widget,
                                        mut f: *mut stfl_form) {
}
unsafe extern "C" fn wt_tablebr_draw(mut w: *mut stfl_widget,
                                     mut f: *mut stfl_form,
                                     mut win: *mut WINDOW) {
}
#[no_mangle]
pub static mut stfl_widget_type_tablebr: stfl_widget_type =
    unsafe {
        {
            let mut init =
                stfl_widget_type{name:
                                     (*::std::mem::transmute::<&[u8; 32],
                                                               &[libc::c_int; 8]>(b"t\x00\x00\x00a\x00\x00\x00b\x00\x00\x00l\x00\x00\x00e\x00\x00\x00b\x00\x00\x00r\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                                         as *mut wchar_t,
                                 f_init: None,
                                 f_done: None,
                                 f_enter: None,
                                 f_leave: None,
                                 f_prepare:
                                     Some(wt_tablebr_prepare as
                                              unsafe extern "C" fn(_:
                                                                       *mut stfl_widget,
                                                                   _:
                                                                       *mut stfl_form)
                                                  -> ()),
                                 f_draw:
                                     Some(wt_tablebr_draw as
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
