use libc::*;
use crate::public::*;
//use crate::public::{pthread_mutex_t, malloc, strdup, strcmp, strlen, realloc, __errno_location, wcslen, pthread_mutex_init, pthread_mutex_lock, pthread_mutex_unlock, fprintf, __assert_fail, calloc, free, abort};
use crate::public::{malloc, wcslen, pthread_mutex_init, pthread_mutex_lock, pthread_mutex_unlock, fprintf, __assert_fail, calloc, free, abort};

extern "C" {
   #[no_mangle]
    static mut stfl_colorpair_counter: c_int;
    #[no_mangle]
    static mut stfl_widget_type_checkbox: stfl_widget_type;
    #[no_mangle]
    static mut stfl_widget_type_textedit: stfl_widget_type;
    #[no_mangle]
    static mut stfl_widget_type_textview: stfl_widget_type;
    #[no_mangle]
    static mut stfl_widget_type_listitem: stfl_widget_type;
    #[no_mangle]
    static mut stfl_widget_type_list: stfl_widget_type;
    #[no_mangle]
    static mut stfl_widget_type_tablebr: stfl_widget_type;
    #[no_mangle]
    static mut stfl_widget_type_table: stfl_widget_type;
    #[no_mangle]
    static mut stfl_widget_type_hbox: stfl_widget_type;
    #[no_mangle]
    static mut stfl_widget_type_vbox: stfl_widget_type;
    #[no_mangle]
    static mut stfl_widget_type_input: stfl_widget_type;
    #[no_mangle]
    static mut stfl_widget_type_label: stfl_widget_type;
    #[no_mangle]
    fn stfl_style(win: *mut WINDOW, style: *const wchar_t);
    #[no_mangle]
    fn stfl_keyname(ch: wchar_t, isfunckey: c_int) -> *mut wchar_t;
}
pub type size_t = c_ulong;
pub type wchar_t = c_int;
pub type wint_t = c_uint;
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

pub type attr_t = chtype;
pub type chtype = c_uint;
#[inline]
unsafe extern "C" fn compat_wcsdup(src: *const wchar_t) -> *mut wchar_t {
    let n: size_t =
        wcslen(src).wrapping_add(1 as c_int as
                                     c_ulong).wrapping_mul(::std::mem::size_of::<wchar_t>()
                                                                     as
                                                                     c_ulong);
    let dest: *mut wchar_t = malloc(n) as *mut wchar_t;
    memcpy(dest as *mut c_void, src as *const c_void, n as _);
    return dest;
}

#[no_mangle]
pub static mut stfl_widget_types: [*mut stfl_widget_type; 12] =
    unsafe {
        [&stfl_widget_type_label as *const stfl_widget_type as
             *mut stfl_widget_type,
         &stfl_widget_type_input as *const stfl_widget_type as
             *mut stfl_widget_type,
         &stfl_widget_type_vbox as *const stfl_widget_type as
             *mut stfl_widget_type,
         &stfl_widget_type_hbox as *const stfl_widget_type as
             *mut stfl_widget_type,
         &stfl_widget_type_table as *const stfl_widget_type as
             *mut stfl_widget_type,
         &stfl_widget_type_tablebr as *const stfl_widget_type as
             *mut stfl_widget_type,
         &stfl_widget_type_list as *const stfl_widget_type as
             *mut stfl_widget_type,
         &stfl_widget_type_listitem as *const stfl_widget_type as
             *mut stfl_widget_type,
         &stfl_widget_type_textview as *const stfl_widget_type as
             *mut stfl_widget_type,
         &stfl_widget_type_textedit as *const stfl_widget_type as
             *mut stfl_widget_type,
         &stfl_widget_type_checkbox as *const stfl_widget_type as
             *mut stfl_widget_type,
         0 as *const stfl_widget_type as *mut stfl_widget_type]
    };
#[no_mangle]
pub static mut id_counter: c_int = 0 as c_int;
#[no_mangle]
pub static mut curses_active: c_int = 0 as c_int;
#[no_mangle]
pub unsafe extern "C" fn stfl_widget_new(mut type_0: *const wchar_t)
 -> *mut stfl_widget {
    let mut setfocus: c_int = 0 as c_int;
    while *type_0 == '!' as i32 {
        setfocus = 1 as c_int;
        type_0 = type_0.offset(1)
    }
    let mut i = 0 as c_int;
    let mut t;
    loop  {
        t = stfl_widget_types[i as usize];
        if t.is_null() { break ; }
        if wcscmp((*t).name, type_0) == 0 { break ; }
        i += 1
    }
    if t.is_null() { return 0 as *mut stfl_widget }
    let mut w: *mut stfl_widget =
        calloc(1 as c_int as c_ulong,
               ::std::mem::size_of::<stfl_widget>() as c_ulong) as
            *mut stfl_widget;
    id_counter += 1;
    (*w).id = id_counter;
    (*w).type_0 = t;
    (*w).setfocus = setfocus;
    if (*(*w).type_0).f_init.is_some() {
        (*(*w).type_0).f_init.expect("non-null function pointer")(w);
    }
    return w;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_widget_free(mut w: *mut stfl_widget) {
    while !(*w).first_child.is_null() { stfl_widget_free((*w).first_child); }
    if (*(*w).type_0).f_done.is_some() {
        (*(*w).type_0).f_done.expect("non-null function pointer")(w);
    }
    let mut kv: *mut stfl_kv = (*w).kv_list;
    while !kv.is_null() {
        let next: *mut stfl_kv = (*kv).next;
        free((*kv).key as *mut c_void);
        free((*kv).value as *mut c_void);
        if !(*kv).name.is_null() { free((*kv).name as *mut c_void); }
        free(kv as *mut c_void);
        kv = next
    }
    if !(*w).parent.is_null() {
        let mut pp: *mut *mut stfl_widget = &mut (*(*w).parent).first_child;
        while *pp != w { pp = &mut (**pp).next_sibling }
        *pp = (*w).next_sibling;
        if (*(*w).parent).last_child == w {
            let mut p: *mut stfl_widget = (*(*w).parent).first_child;
            (*(*w).parent).last_child = 0 as *mut stfl_widget;
            while !p.is_null() {
                (*(*w).parent).last_child = p;
                p = (*p).next_sibling
            }
        }
    }
    if !(*w).name.is_null() { free((*w).name as *mut c_void); }
    if !(*w).cls.is_null() { free((*w).cls as *mut c_void); }
    free(w as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn stfl_widget_setkv_int(w: *mut stfl_widget,
                                               key: *const wchar_t,
                                               value: c_int)
 -> *mut stfl_kv {
    let mut newtext: [wchar_t; 64] = [0; 64];
    swprintf(newtext.as_mut_ptr(), 64 as c_int as size_t,
             (*::std::mem::transmute::<&[u8; 12],
                                       &[c_int; 3]>(b"%\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
             value);
    return stfl_widget_setkv_str(w, key, newtext.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn stfl_widget_setkv_str(mut w: *mut stfl_widget,
                                               key: *const wchar_t,
                                               value: *const wchar_t)
 -> *mut stfl_kv {
    let mut kv: *mut stfl_kv = (*w).kv_list;
    while !kv.is_null() {
        if wcscmp((*kv).key, key) == 0 {
            free((*kv).value as *mut c_void);
            (*kv).value = compat_wcsdup(value);
            return kv
        }
        kv = (*kv).next
    }
    kv =
        calloc(1 as c_int as c_ulong,
               ::std::mem::size_of::<stfl_kv>() as c_ulong) as
            *mut stfl_kv;
    (*kv).widget = w;
    (*kv).key = compat_wcsdup(key);
    (*kv).value = compat_wcsdup(value);
    id_counter += 1;
    (*kv).id = id_counter;
    (*kv).next = (*w).kv_list;
    (*w).kv_list = kv;
    return kv;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_setkv_by_name_int(w: *mut stfl_widget,
                                                name: *const wchar_t,
                                                value: c_int)
 -> *mut stfl_kv {
    let mut newtext: [wchar_t; 64] = [0; 64];
    swprintf(newtext.as_mut_ptr(), 64 as c_int as size_t,
             (*::std::mem::transmute::<&[u8; 12],
                                       &[c_int; 3]>(b"%\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
             value);
    return stfl_setkv_by_name_str(w, name, newtext.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn stfl_setkv_by_name_str(w: *mut stfl_widget,
                                                name: *const wchar_t,
                                                value: *const wchar_t)
 -> *mut stfl_kv {
    let kv: *mut stfl_kv = stfl_kv_by_name(w, name);
    if kv.is_null() { return 0 as *mut stfl_kv }
    free((*kv).value as *mut c_void);
    (*kv).value = compat_wcsdup(value);
    return kv;
}
unsafe extern "C" fn stfl_widget_getkv_worker(w: *mut stfl_widget,
                                              key: *const wchar_t)
 -> *mut stfl_kv {
    let mut kv: *mut stfl_kv = (*w).kv_list;
    while !kv.is_null() {
        if wcscmp((*kv).key, key) == 0 { return kv }
        kv = (*kv).next
    }
    return 0 as *mut stfl_kv;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_widget_getkv(mut w: *mut stfl_widget,
                                           key: *const wchar_t)
 -> *mut stfl_kv {
    let mut kv: *mut stfl_kv = stfl_widget_getkv_worker(w, key);
    if !kv.is_null() { return kv }
    let key1_len: c_int =
        wcslen(key).wrapping_add(2 as c_int as c_ulong) as
            c_int;
    let vla = key1_len as usize;
    let mut key1: Vec<wchar_t> = ::std::vec::from_elem(0, vla);
    let key2_len: c_int =
        (key1_len as
             c_ulong).wrapping_add(wcslen((*(*w).type_0).name)).wrapping_add(1
                                                                                       as
                                                                                       c_int
                                                                                       as
                                                                                       c_ulong)
            as c_int;
    let vla_0 = key2_len as usize;
    let mut key2: Vec<wchar_t> = ::std::vec::from_elem(0, vla_0);
    let key3_len: c_int =
        if !(*w).cls.is_null() {
            (key1_len as
                 c_ulong).wrapping_add(wcslen((*w).cls)).wrapping_add(1
                                                                                as
                                                                                c_int
                                                                                as
                                                                                c_ulong)
        } else { 0 as c_int as c_ulong } as c_int;
    let vla_1 = key3_len as usize;
    let mut key3: Vec<wchar_t> = ::std::vec::from_elem(0, vla_1);
    swprintf(key1.as_mut_ptr(), key1_len as size_t,
             (*::std::mem::transmute::<&[u8; 20],
                                       &[c_int; 5]>(b"@\x00\x00\x00%\x00\x00\x00l\x00\x00\x00s\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
             key);
    swprintf(key2.as_mut_ptr(), key2_len as size_t,
             (*::std::mem::transmute::<&[u8; 36],
                                       &[c_int; 9]>(b"@\x00\x00\x00%\x00\x00\x00l\x00\x00\x00s\x00\x00\x00#\x00\x00\x00%\x00\x00\x00l\x00\x00\x00s\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
             (*(*w).type_0).name, key);
    if key3_len != 0 {
        swprintf(key3.as_mut_ptr(), key3_len as size_t,
                 (*::std::mem::transmute::<&[u8; 36],
                                           &[c_int; 9]>(b"@\x00\x00\x00%\x00\x00\x00l\x00\x00\x00s\x00\x00\x00#\x00\x00\x00%\x00\x00\x00l\x00\x00\x00s\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                 (*w).cls, key);
    }
    while !w.is_null() {
        if key3_len != 0 {
            kv = stfl_widget_getkv_worker(w, key3.as_mut_ptr());
            if !kv.is_null() { return kv }
        }
        kv = stfl_widget_getkv_worker(w, key2.as_mut_ptr());
        if !kv.is_null() { return kv }
        kv = stfl_widget_getkv_worker(w, key1.as_mut_ptr());
        if !kv.is_null() { return kv }
        w = (*w).parent
    }
    return 0 as *mut stfl_kv;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_widget_getkv_int(w: *mut stfl_widget,
                                               key: *const wchar_t,
                                               defval: c_int)
 -> c_int {
    let kv: *mut stfl_kv = stfl_widget_getkv(w, key);
    let mut ret: c_int = 0;
    if kv.is_null() || *(*kv).value.offset(0 as c_int as isize) == 0 {
        return defval
    }
    if swscanf((*kv).value,
               (*::std::mem::transmute::<&[u8; 12],
                                         &[c_int; 3]>(b"%\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
               &mut ret as *mut c_int) < 1 as c_int {
        return defval
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_widget_getkv_str(w: *mut stfl_widget,
                                               key: *const wchar_t,
                                               defval: *const wchar_t)
 -> *const wchar_t {
    let kv: *mut stfl_kv = stfl_widget_getkv(w, key);
    return if !kv.is_null() { (*kv).value } else { defval };
}
#[no_mangle]
pub unsafe extern "C" fn stfl_getkv_by_name_int(w: *mut stfl_widget,
                                                name: *const wchar_t,
                                                defval: c_int)
 -> c_int {
    let kv: *mut stfl_kv = stfl_kv_by_name(w, name);
    let mut ret: c_int = 0;
    if kv.is_null() || *(*kv).value.offset(0 as c_int as isize) == 0 {
        return defval
    }
    if swscanf((*kv).value,
               (*::std::mem::transmute::<&[u8; 12],
                                         &[c_int; 3]>(b"%\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
               &mut ret as *mut c_int) < 1 as c_int {
        return defval
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_getkv_by_name_str(w: *mut stfl_widget,
                                                name: *const wchar_t,
                                                defval: *const wchar_t)
 -> *const wchar_t {
    let kv: *mut stfl_kv = stfl_kv_by_name(w, name);
    return if !kv.is_null() { (*kv).value } else { defval };
}
#[no_mangle]
pub unsafe extern "C" fn stfl_widget_by_name(mut w: *mut stfl_widget,
                                             name: *const wchar_t)
 -> *mut stfl_widget {
    if !(*w).name.is_null() && wcscmp((*w).name, name) == 0 { return w }
    w = (*w).first_child;
    while !w.is_null() {
        let r: *mut stfl_widget = stfl_widget_by_name(w, name);
        if !r.is_null() { return r }
        w = (*w).next_sibling
    }
    return 0 as *mut stfl_widget;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_widget_by_id(mut w: *mut stfl_widget,
                                           id: c_int)
 -> *mut stfl_widget {
    if (*w).id == id { return w }
    w = (*w).first_child;
    while !w.is_null() {
        let r: *mut stfl_widget = stfl_widget_by_id(w, id);
        if !r.is_null() { return r }
        w = (*w).next_sibling
    }
    return 0 as *mut stfl_widget;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_kv_by_name(mut w: *mut stfl_widget,
                                         name: *const wchar_t)
 -> *mut stfl_kv {
    let mut kv: *mut stfl_kv = (*w).kv_list;
    while !kv.is_null() {
        if !(*kv).name.is_null() && wcscmp((*kv).name, name) == 0 {
            return kv
        }
        kv = (*kv).next
    }
    w = (*w).first_child;
    while !w.is_null() {
        let r: *mut stfl_kv = stfl_kv_by_name(w, name);
        if !r.is_null() { return r }
        w = (*w).next_sibling
    }
    return 0 as *mut stfl_kv;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_kv_by_id(mut w: *mut stfl_widget,
                                       id: c_int) -> *mut stfl_kv {
    let mut kv: *mut stfl_kv = (*w).kv_list;
    while !kv.is_null() { if (*kv).id == id { return kv } kv = (*kv).next }
    w = (*w).first_child;
    while !w.is_null() {
        let r: *mut stfl_kv = stfl_kv_by_id(w, id);
        if !r.is_null() { return r }
        w = (*w).next_sibling
    }
    return 0 as *mut stfl_kv;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_find_child_tree(w: *mut stfl_widget,
                                              mut c: *mut stfl_widget)
 -> *mut stfl_widget {
    while !c.is_null() { if (*c).parent == w { return c } c = (*c).parent }
    return 0 as *mut stfl_widget;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_find_first_focusable(w: *mut stfl_widget)
 -> *mut stfl_widget {
    if (*w).allow_focus != 0 &&
           stfl_widget_getkv_int(w,
                                 (*::std::mem::transmute::<&[u8; 40],
                                                           &[c_int; 10]>(b"c\x00\x00\x00a\x00\x00\x00n\x00\x00\x00_\x00\x00\x00f\x00\x00\x00o\x00\x00\x00c\x00\x00\x00u\x00\x00\x00s\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                 1 as c_int) != 0 &&
           stfl_widget_getkv_int(w,
                                 (*::std::mem::transmute::<&[u8; 36],
                                                           &[c_int; 9]>(b".\x00\x00\x00d\x00\x00\x00i\x00\x00\x00s\x00\x00\x00p\x00\x00\x00l\x00\x00\x00a\x00\x00\x00y\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                 1 as c_int) != 0 {
        return w
    }
    let mut c: *mut stfl_widget = (*w).first_child;
    while !c.is_null() {
        if stfl_widget_getkv_int(w,
                                 (*::std::mem::transmute::<&[u8; 36],
                                                           &[c_int; 9]>(b".\x00\x00\x00d\x00\x00\x00i\x00\x00\x00s\x00\x00\x00p\x00\x00\x00l\x00\x00\x00a\x00\x00\x00y\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                 1 as c_int) != 0 {
            let r: *mut stfl_widget = stfl_find_first_focusable(c);
            if !r.is_null() { return r }
        }
        c = (*c).next_sibling
    }
    return 0 as *mut stfl_widget;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_focus_prev(w: *mut stfl_widget,
                                         old_fw: *mut stfl_widget,
                                         mut f: *mut stfl_form)
 -> c_int {
    let mut stop: *mut stfl_widget = stfl_find_child_tree(w, old_fw);
    if !stop.is_null() {
    } else {
        __assert_fail(b"stop\x00" as *const u8 as *const c_char,
                      b"base.c\x00" as *const u8 as *const c_char,
                      362 as c_int as c_uint,
                      (*::std::mem::transmute::<&[u8; 84],
                                                &[c_char; 84]>(b"int stfl_focus_prev(struct stfl_widget *, struct stfl_widget *, struct stfl_form *)\x00")).as_ptr());
    }
    while (*w).first_child != stop {
        let mut c: *mut stfl_widget = (*w).first_child;
        while (*c).next_sibling != stop { c = (*c).next_sibling }
        let new_fw: *mut stfl_widget = stfl_find_first_focusable(c);
        if !new_fw.is_null() {
            if (*(*old_fw).type_0).f_leave.is_some() {
                (*(*old_fw).type_0).f_leave.expect("non-null function pointer")(old_fw,
                                                                                f);
            }
            if (*(*new_fw).type_0).f_enter.is_some() {
                (*(*new_fw).type_0).f_enter.expect("non-null function pointer")(new_fw,
                                                                                f);
            }
            (*f).current_focus_id = (*new_fw).id;
            return 1 as c_int
        }
        stop = c
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_focus_next(w: *mut stfl_widget,
                                         old_fw: *mut stfl_widget,
                                         mut f: *mut stfl_form)
 -> c_int {
    let mut c: *mut stfl_widget = stfl_find_child_tree(w, old_fw);
    if !c.is_null() {
    } else {
        __assert_fail(b"c\x00" as *const u8 as *const c_char,
                      b"base.c\x00" as *const u8 as *const c_char,
                      392 as c_int as c_uint,
                      (*::std::mem::transmute::<&[u8; 84],
                                                &[c_char; 84]>(b"int stfl_focus_next(struct stfl_widget *, struct stfl_widget *, struct stfl_form *)\x00")).as_ptr());
    }
    c = (*c).next_sibling;
    while !c.is_null() {
        let new_fw: *mut stfl_widget = stfl_find_first_focusable(c);
        if !new_fw.is_null() {
            if (*(*old_fw).type_0).f_leave.is_some() {
                (*(*old_fw).type_0).f_leave.expect("non-null function pointer")(old_fw,
                                                                                f);
            }
            if (*(*new_fw).type_0).f_enter.is_some() {
                (*(*new_fw).type_0).f_enter.expect("non-null function pointer")(new_fw,
                                                                                f);
            }
            (*f).current_focus_id = (*new_fw).id;
            return 1 as c_int
        }
        c = (*c).next_sibling
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_switch_focus(mut old_fw: *mut stfl_widget,
                                           new_fw: *mut stfl_widget,
                                           mut f: *mut stfl_form)
 -> c_int {
    if new_fw.is_null() || (*new_fw).allow_focus == 0 {
        return 0 as c_int
    }
    if old_fw.is_null() && (*f).current_focus_id != 0 {
        old_fw = stfl_widget_by_id((*f).root, (*f).current_focus_id)
    }
    if !old_fw.is_null() && (*(*old_fw).type_0).f_leave.is_some() {
        (*(*old_fw).type_0).f_leave.expect("non-null function pointer")(old_fw,
                                                                        f);
    }
    if (*(*new_fw).type_0).f_enter.is_some() {
        (*(*new_fw).type_0).f_enter.expect("non-null function pointer")(new_fw,
                                                                        f);
    }
    (*f).current_focus_id = (*new_fw).id;
    return 1 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_form_new() -> *mut stfl_form {
    let f: *mut stfl_form =
        calloc(1 as c_int as c_ulong,
               ::std::mem::size_of::<stfl_form>() as c_ulong) as
            *mut stfl_form;
    if !f.is_null() {
        pthread_mutex_init(&mut (*f).mtx, 0 as *const pthread_mutexattr_t);
    }
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_form_event(f: *mut stfl_form,
                                         event: *mut wchar_t) {
    let mut ep: *mut *mut stfl_event = &mut (*f).event_queue;
    let mut e: *mut stfl_event =
        calloc(1 as c_int as c_ulong,
               ::std::mem::size_of::<stfl_event>() as c_ulong) as
            *mut stfl_event;
    (*e).event = event;
    while !(*ep).is_null() { ep = &mut (**ep).next }
    *ep = e;
}
unsafe extern "C" fn stfl_gather_focus_widget(f: *mut stfl_form)
 -> *mut stfl_widget {
    let mut fw: *mut stfl_widget =
        stfl_widget_by_id((*f).root, (*f).current_focus_id);
    if fw.is_null() {
        fw = stfl_find_first_focusable((*f).root);
        if !fw.is_null() && (*(*fw).type_0).f_enter.is_some() {
            (*(*fw).type_0).f_enter.expect("non-null function pointer")(fw,
                                                                        f);
        }
    }
    return fw;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_form_run(f: *mut stfl_form,
                                       timeout: c_int) {
    let mut current_block: u64;
    let mut on_handler: *mut wchar_t = 0 as *mut wchar_t;
    pthread_mutex_lock(&mut (*f).mtx);
    if !(*f).event.is_null() { free((*f).event as *mut c_void); }
    (*f).event = 0 as *mut wchar_t;
    if !(timeout >= 0 as c_int && !(*f).event_queue.is_null()) {
        if !(timeout == -(2 as c_int)) {
            if (*f).root.is_null() {
                fprintf(stderr,
                        b"STFL Fatal Error: Called stfl_form_run() without root widget.\n\x00"
                            as *const u8 as *const c_char);
                abort();
            }
            if curses_active == 0 {
                initscr();
                cbreak();
                noecho();
                nonl();
                keypad(stdscr, 1 as c_int != 0);
                doupdate();
                start_color();
                use_default_colors();
                wbkgdset(stdscr, ' ' as i32 as chtype);
                curses_active = 1
            }
            stfl_colorpair_counter = 1;
            (*(*(*f).root).type_0).f_prepare.expect("non-null function pointer")((*f).root, f);
            let mut fw = stfl_gather_focus_widget(f);
            (*f).current_focus_id = if !fw.is_null() { (*fw).id } else { 0 };
            (*(*f).root).y = if !stdscr.is_null() { (*stdscr)._begy as c_int } else { -1 };
            (*(*f).root).x = if !stdscr.is_null() { (*stdscr)._begx as c_int } else { -1 };
            (*(*f).root).h = if !stdscr.is_null() { ((*stdscr)._maxy as c_int) + 1 } else { -1 };
            (*(*f).root).w = if !stdscr.is_null() { ((*stdscr)._maxx as c_int) + 1 } else { -1 };
            if timeout == -3 {
                let dummywin: *mut WINDOW = newwin(0, 0, 0, 0);
                if dummywin.is_null() {
                    panic!("STFL Fatal Error: stfl_form_run() got a NULL pointer from newwin(0, 0, 0, 0).");
                }
                (*(*(*f).root).type_0).f_draw.expect("non-null function pointer")((*f).root,
                                                                                  f,
                                                                                  dummywin);
                delwin(dummywin);
                pthread_mutex_unlock(&mut (*f).mtx);
                return
            }
            werase(stdscr);
            (*(*(*f).root).type_0).f_draw.expect("non-null function pointer")((*f).root,
                                                                              f,
                                                                              stdscr);
            if timeout == -1 &&
                   (*(*f).root).cur_y != -1 &&
                   (*(*f).root).cur_x != -1 {
                wmove(stdscr, (*(*f).root).cur_y, (*(*f).root).cur_x);
            }
            wrefresh(stdscr);
            if timeout < 0 as c_int {
                pthread_mutex_unlock(&mut (*f).mtx);
                return
            }
            wtimeout(stdscr,
                     if timeout == 0 as c_int {
                         -1
                     } else { timeout });
            wmove(stdscr, (*f).cursor_y, (*f).cursor_x);
            let mut wch = 0;
            pthread_mutex_unlock(&mut (*f).mtx);
            let rc = wget_wch(stdscr, &mut wch);
            pthread_mutex_lock(&mut (*f).mtx);
            /* fw may be invalid, regather it */
            fw = stfl_gather_focus_widget(f);
            (*f).current_focus_id =
                if !fw.is_null() { (*fw).id } else { 0 as c_int };
            let mut w = fw;
            if rc == -1 {
                stfl_form_event(f,
                                compat_wcsdup((*::std::mem::transmute::<&[u8; 32],
                                                                        &[c_int; 8]>(b"T\x00\x00\x00I\x00\x00\x00M\x00\x00\x00E\x00\x00\x00O\x00\x00\x00U\x00\x00\x00T\x00\x00\x00\x00\x00\x00\x00")).as_ptr()));
            } else {
                let on_event =
                    stfl_keyname(wch as wchar_t,
                                 (rc == 0o400 as c_int) as c_int);
                let on_handler_len =
                    wcslen(on_event).wrapping_add(4 as c_int as
                                                      c_ulong) as
                        c_int;
                on_handler =
                    malloc((on_handler_len as
                                c_ulong).wrapping_mul(::std::mem::size_of::<wchar_t>()
                                                                as
                                                                c_ulong))
                        as *mut wchar_t;
                swprintf(on_handler, on_handler_len as size_t,
                         (*::std::mem::transmute::<&[u8; 28],
                                                   &[c_int; 7]>(b"o\x00\x00\x00n\x00\x00\x00_\x00\x00\x00%\x00\x00\x00l\x00\x00\x00s\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                         on_event);
                free(on_event as *mut c_void);
                loop  {
                    if w.is_null() {
                        current_block = 1623252117315916725;
                        break ;
                    }
                    let event: *const wchar_t =
                        stfl_widget_getkv_str(w, on_handler,
                                              0 as *const wchar_t);
                    if !event.is_null() {
                        stfl_form_event(f, compat_wcsdup(event));
                        current_block = 2463987395154258233;
                        break ;
                    } else {
                        if (*(*w).type_0).f_process.is_some() &&
                               stfl_widget_getkv_int(w,
                                                     (*::std::mem::transmute::<&[u8; 32],
                                                                               &[c_int; 8]>(b"p\x00\x00\x00r\x00\x00\x00o\x00\x00\x00c\x00\x00\x00e\x00\x00\x00s\x00\x00\x00s\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                                     1 as c_int) != 0 &&
                               (*(*w).type_0).f_process.expect("non-null function pointer")(w,
                                                                                            fw,
                                                                                            f,
                                                                                            wch
                                                                                                as
                                                                                                wchar_t,
                                                                                            (rc
                                                                                                 ==
                                                                                                 0o400
                                                                                                     as
                                                                                                     c_int)
                                                                                                as
                                                                                                c_int)
                                   != 0 {
                            current_block = 2463987395154258233;
                            break ;
                        }
                        if stfl_widget_getkv_int(w,
                                                 (*::std::mem::transmute::<&[u8; 24],
                                                                           &[c_int; 6]>(b"m\x00\x00\x00o\x00\x00\x00d\x00\x00\x00a\x00\x00\x00l\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                                 0 as c_int) != 0 {
                            current_block = 4896165486537497759;
                            break ;
                        }
                        w = (*w).parent
                    }
                }
                match current_block {
                    2463987395154258233 => { }
                    _ => {
                        match current_block {
                            1623252117315916725 => {
                                if rc != 0o400 as c_int &&
                                       wch == '\t' as i32 as c_uint {
                                    fw =
                                        stfl_widget_by_id((*f).root,
                                                          (*f).current_focus_id);
                                    let old_fw: *mut stfl_widget = fw;
                                    if fw.is_null() {
                                        current_block = 4896165486537497759;
                                    } else {
                                        loop  {
                                            if !(*fw).first_child.is_null() {
                                                fw = (*fw).first_child
                                            } else if !(*fw).next_sibling.is_null()
                                             {
                                                fw = (*fw).next_sibling
                                            } else {
                                                while !(*fw).parent.is_null()
                                                          &&
                                                          (*(*fw).parent).next_sibling.is_null()
                                                      {
                                                    fw = (*fw).parent
                                                }
                                                fw =
                                                    if !(*fw).parent.is_null()
                                                       {
                                                        (*(*fw).parent).next_sibling
                                                    } else {
                                                        0 as *mut stfl_widget
                                                    }
                                            }
                                            if fw.is_null() &&
                                                   !old_fw.is_null() {
                                                fw = (*f).root
                                            }
                                            if !(!fw.is_null() &&
                                                     !((*fw).allow_focus != 0
                                                           &&
                                                           stfl_widget_getkv_int(fw,
                                                                                 (*::std::mem::transmute::<&[u8; 40],
                                                                                                           &[c_int; 10]>(b"c\x00\x00\x00a\x00\x00\x00n\x00\x00\x00_\x00\x00\x00f\x00\x00\x00o\x00\x00\x00c\x00\x00\x00u\x00\x00\x00s\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                                                                 1
                                                                                     as
                                                                                     c_int)
                                                               != 0)) {
                                                break ;
                                            }
                                        }
                                        if old_fw != fw {
                                            if !old_fw.is_null() &&
                                                   (*(*old_fw).type_0).f_leave.is_some()
                                               {
                                                (*(*old_fw).type_0).f_leave.expect("non-null function pointer")(old_fw,
                                                                                                                f);
                                            }
                                            if !fw.is_null() &&
                                                   (*(*fw).type_0).f_enter.is_some()
                                               {
                                                (*(*fw).type_0).f_enter.expect("non-null function pointer")(fw,
                                                                                                            f);
                                            }
                                            (*f).current_focus_id =
                                                if !fw.is_null() {
                                                    (*fw).id
                                                } else { 0 as c_int }
                                        }
                                        current_block = 2463987395154258233;
                                    }
                                } else if rc == 0o400 as c_int &&
                                              wch ==
                                                  0o541 as c_int as
                                                      c_uint {
                                    let mut old_fw_0: *mut stfl_widget =
                                        stfl_widget_by_id((*f).root,
                                                          (*f).current_focus_id);
                                    let mut tmp_fw: *mut stfl_widget =
                                        (*f).root;
                                    let mut fw_0: *mut stfl_widget =
                                        0 as *mut stfl_widget;
                                    loop  {
                                        while !tmp_fw.is_null() &&
                                                  tmp_fw != old_fw_0 {
                                            if (*tmp_fw).allow_focus != 0 &&
                                                   stfl_widget_getkv_int(tmp_fw,
                                                                         (*::std::mem::transmute::<&[u8; 40],
                                                                                                   &[c_int; 10]>(b"c\x00\x00\x00a\x00\x00\x00n\x00\x00\x00_\x00\x00\x00f\x00\x00\x00o\x00\x00\x00c\x00\x00\x00u\x00\x00\x00s\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                                                         1 as
                                                                             c_int)
                                                       != 0 {
                                                fw_0 = tmp_fw
                                            }
                                            if !(*tmp_fw).first_child.is_null()
                                               {
                                                tmp_fw = (*tmp_fw).first_child
                                            } else if !(*tmp_fw).next_sibling.is_null()
                                             {
                                                tmp_fw =
                                                    (*tmp_fw).next_sibling
                                            } else {
                                                while !(*tmp_fw).parent.is_null()
                                                          &&
                                                          (*(*tmp_fw).parent).next_sibling.is_null()
                                                      {
                                                    tmp_fw = (*tmp_fw).parent
                                                }
                                                tmp_fw =
                                                    if !(*tmp_fw).parent.is_null()
                                                       {
                                                        (*(*tmp_fw).parent).next_sibling
                                                    } else {
                                                        0 as *mut stfl_widget
                                                    }
                                            }
                                        }
                                        if !(fw_0.is_null() &&
                                                 !old_fw_0.is_null()) {
                                            break ;
                                        }
                                        old_fw_0 = (*(*f).root).last_child
                                    }
                                    if !fw_0.is_null() && old_fw_0 != fw_0 {
                                        if !old_fw_0.is_null() &&
                                               (*(*old_fw_0).type_0).f_leave.is_some()
                                           {
                                            (*(*old_fw_0).type_0).f_leave.expect("non-null function pointer")(old_fw_0,
                                                                                                              f);
                                        }
                                        if !fw_0.is_null() &&
                                               (*(*fw_0).type_0).f_enter.is_some()
                                           {
                                            (*(*fw_0).type_0).f_enter.expect("non-null function pointer")(fw_0,
                                                                                                          f);
                                        }
                                        (*f).current_focus_id =
                                            if !fw_0.is_null() {
                                                (*fw_0).id
                                            } else { 0 as c_int }
                                    }
                                    current_block = 2463987395154258233;
                                } else {
                                    current_block = 4896165486537497759;
                                }
                            }
                            _ => { }
                        }
                        match current_block {
                            2463987395154258233 => { }
                            _ => {
                                stfl_form_event(f,
                                                stfl_keyname(wch as wchar_t,
                                                             (rc ==
                                                                  0o400 as
                                                                      c_int)
                                                                 as
                                                                 c_int));
                            }
                        }
                    }
                }
            }
        }
    }
    let e: *mut stfl_event = (*f).event_queue;
    if !e.is_null() {
        (*f).event_queue = (*e).next;
        (*f).event = (*e).event;
        free(e as *mut c_void);
    }
    pthread_mutex_unlock(&mut (*f).mtx);
    free(on_handler as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn stfl_form_reset() {
    if curses_active != 0 { endwin(); curses_active = 0 as c_int };
}
#[no_mangle]
pub unsafe extern "C" fn stfl_form_redraw() {
    if curses_active != 0 { clearok(curscr, 1 as c_int != 0); };
}
#[no_mangle]
pub unsafe extern "C" fn stfl_form_free(f: *mut stfl_form) {
    pthread_mutex_lock(&mut (*f).mtx);
    if !(*f).root.is_null() { stfl_widget_free((*f).root); }
    if !(*f).event.is_null() { free((*f).event as *mut c_void); }
    pthread_mutex_unlock(&mut (*f).mtx);
    free(f as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn stfl_check_setfocus(mut f: *mut stfl_form,
                                             mut w: *mut stfl_widget) {
    if (*w).setfocus != 0 {
        (*f).current_focus_id = (*w).id;
        (*w).setfocus = 0 as c_int
    }
    w = (*w).first_child;
    while !w.is_null() { stfl_check_setfocus(f, w); w = (*w).next_sibling };
}
unsafe extern "C" fn compute_len_from_width(mut p: *const wchar_t,
                                            mut width: c_uint)
 -> c_uint {
    let mut len: c_uint = 0 as c_int as c_uint;
    let mut end_loop: c_uint = 0 as c_int as c_uint;
    while !p.is_null() && *p != 0 && end_loop == 0 {
        if wcwidth(*p) as c_uint > width {
            end_loop = 1 as c_int as c_uint
        } else {
            width = width.wrapping_sub(wcwidth(*p) as c_uint);
            p = p.offset(1);
            len = len.wrapping_add(1)
        }
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_print_richtext(w: *mut stfl_widget,
                                             win: *mut WINDOW,
                                             y: c_uint,
                                             mut x: c_uint,
                                             text: *const wchar_t,
                                             width: c_uint,
                                             style_normal: *const wchar_t,
                                             has_focus: c_int)
 -> c_uint {
    let mut p: *const wchar_t = text;
    let mut retval: c_uint = 0 as c_int as c_uint;
    let end_col: c_uint = x.wrapping_add(width);
    while *p != 0 {
        let mut len: c_uint =
            compute_len_from_width(p, end_col.wrapping_sub(x));
        let p1: *const wchar_t = wcschr(p, '<' as i32);
        if p1.is_null() {
            if wmove(win, y as c_int, x as c_int) ==
                   -1 {
            } else { waddnwstr(win, p, len as c_int); };
            retval = retval.wrapping_add(len);
            break ;
        } else {
            let p2: *const wchar_t =
                wcschr(p1.offset(1 as c_int as isize), '>' as i32);
            if len as c_long >
                   p1.wrapping_offset_from(p) as c_long {
                len =
                    p1.wrapping_offset_from(p) as c_long as c_uint
            }
            if wmove(win, y as c_int, x as c_int) ==
                   -1 {
            } else { waddnwstr(win, p, len as c_int); };
            retval = retval.wrapping_add(len);
            x = x.wrapping_add(wcswidth(p, len as size_t) as c_uint);
            if p2.is_null() { break ; }
            let vla = p2.wrapping_offset_from(p1) as c_long as usize;
            let mut stylename: Vec<wchar_t> = ::std::vec::from_elem(0, vla);
            wmemcpy(stylename.as_mut_ptr(),
                    p1.offset(1 as c_int as isize),
                    (p2.wrapping_offset_from(p1) as c_long -
                         1 as c_int as c_long) as size_t);
            *stylename.as_mut_ptr().offset((p2.wrapping_offset_from(p1) as
                                                c_long -
                                                1 as c_int as
                                                    c_long) as isize) =
                '\u{0}' as i32;
            if wcscmp(stylename.as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 4],
                                                &[c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr())
                   == 0 as c_int {
                if end_col.wrapping_sub(x) > 0 as c_int as c_uint
                   {
                    if wmove(win, y as c_int, x as c_int) ==
                           -1 {
                    } else {
                        waddnwstr(win,
                                  (*::std::mem::transmute::<&[u8; 8],
                                                            &[c_int; 2]>(b"<\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                  1 as c_int);
                    };
                    retval =
                        retval.wrapping_add(1 as c_int as c_uint);
                    x = x.wrapping_add(1)
                }
            } else if wcscmp(stylename.as_mut_ptr(),
                             (*::std::mem::transmute::<&[u8; 8],
                                                       &[c_int; 2]>(b"/\x00\x00\x00\x00\x00\x00\x00")).as_ptr())
                          == 0 as c_int {
                stfl_style(win, style_normal);
            } else {
                let mut lookup_stylename: [wchar_t; 128] = [0; 128];
                if has_focus != 0 {
                    swprintf(lookup_stylename.as_mut_ptr(),
                             (::std::mem::size_of::<[wchar_t; 128]>() as
                                  c_ulong).wrapping_div(::std::mem::size_of::<wchar_t>()
                                                                  as
                                                                  c_ulong),
                             (*::std::mem::transmute::<&[u8; 64],
                                                       &[c_int; 16]>(b"s\x00\x00\x00t\x00\x00\x00y\x00\x00\x00l\x00\x00\x00e\x00\x00\x00_\x00\x00\x00%\x00\x00\x00l\x00\x00\x00s\x00\x00\x00_\x00\x00\x00f\x00\x00\x00o\x00\x00\x00c\x00\x00\x00u\x00\x00\x00s\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                             stylename.as_mut_ptr());
                } else {
                    swprintf(lookup_stylename.as_mut_ptr(),
                             (::std::mem::size_of::<[wchar_t; 128]>() as
                                  c_ulong).wrapping_div(::std::mem::size_of::<wchar_t>()
                                                                  as
                                                                  c_ulong),
                             (*::std::mem::transmute::<&[u8; 68],
                                                       &[c_int; 17]>(b"s\x00\x00\x00t\x00\x00\x00y\x00\x00\x00l\x00\x00\x00e\x00\x00\x00_\x00\x00\x00%\x00\x00\x00l\x00\x00\x00s\x00\x00\x00_\x00\x00\x00n\x00\x00\x00o\x00\x00\x00r\x00\x00\x00m\x00\x00\x00a\x00\x00\x00l\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                             stylename.as_mut_ptr());
                }
                let style =
                    stfl_widget_getkv_str(w, lookup_stylename.as_mut_ptr(),
                                          (*::std::mem::transmute::<&[u8; 4],
                                                                    &[c_int; 1]>(b"\x00\x00\x00\x00")).as_ptr());
                stfl_style(win, style);
            }
            p = p2.offset(1 as c_int as isize)
        }
    }
    return retval;
}
