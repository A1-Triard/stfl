use ::libc;
use crate::public::*;
extern "C" {
    #[no_mangle]
    fn wcslen(_: *const libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn iconv_open(__tocode: *const libc::c_char,
                  __fromcode: *const libc::c_char) -> iconv_t;
    #[no_mangle]
    fn iconv(__cd: iconv_t, __inbuf: *mut *mut libc::c_char,
             __inbytesleft: *mut size_t, __outbuf: *mut *mut libc::c_char,
             __outbytesleft: *mut size_t) -> size_t;
    #[no_mangle]
    fn iconv_close(__cd: iconv_t) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn pthread_mutex_init(__mutex: *mut pthread_mutex_t,
                          __mutexattr: *const pthread_mutexattr_t)
     -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stfl_ipool {
    pub to_wc_desc: iconv_t,
    pub from_wc_desc: iconv_t,
    pub code: *mut libc::c_char,
    pub list: *mut stfl_ipool_entry,
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
 *  iconv.c: Helper functions for widechar conversion
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stfl_ipool_entry {
    pub data: *mut libc::c_void,
    pub next: *mut stfl_ipool_entry,
}
pub type iconv_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn stfl_ipool_create(code: *const libc::c_char)
 -> *mut stfl_ipool {
    let mut pool: *mut stfl_ipool =
        malloc(::std::mem::size_of::<stfl_ipool>() as libc::c_ulong) as
            *mut stfl_ipool;
    pthread_mutex_init(&mut (*pool).mtx, 0 as *const pthread_mutexattr_t);
    (*pool).to_wc_desc = -(1 as libc::c_int) as iconv_t;
    (*pool).from_wc_desc = -(1 as libc::c_int) as iconv_t;
    (*pool).code = strdup(code);
    (*pool).list = 0 as *mut stfl_ipool_entry;
    return pool;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_ipool_add(pool: *mut stfl_ipool,
                                        data: *mut libc::c_void)
 -> *mut libc::c_void {
    let mut entry: *mut stfl_ipool_entry =
        malloc(::std::mem::size_of::<stfl_ipool_entry>() as libc::c_ulong) as
            *mut stfl_ipool_entry;
    pthread_mutex_lock(&mut (*pool).mtx);
    (*entry).data = data;
    (*entry).next = (*pool).list;
    (*pool).list = entry;
    pthread_mutex_unlock(&mut (*pool).mtx);
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_ipool_towc(pool: *mut stfl_ipool,
                                         buf: *const libc::c_char)
 -> *const wchar_t {
    if pool.is_null() || buf.is_null() { return 0 as *const wchar_t }
    pthread_mutex_lock(&mut (*pool).mtx);
    if strcmp(b"WCHAR_T\x00" as *const u8 as *const libc::c_char,
              (*pool).code) == 0 {
        pthread_mutex_unlock(&mut (*pool).mtx);
        return buf as *mut wchar_t
    }
    if (*pool).to_wc_desc == -(1 as libc::c_int) as iconv_t {
        (*pool).to_wc_desc =
            iconv_open(b"WCHAR_T\x00" as *const u8 as *const libc::c_char,
                       (*pool).code)
    }
    if (*pool).to_wc_desc == -(1 as libc::c_int) as iconv_t {
        pthread_mutex_unlock(&mut (*pool).mtx);
        return 0 as *const wchar_t
    }
    let mut inbuf: *mut libc::c_char = buf as *mut libc::c_char;
    let mut inbytesleft: size_t = strlen(buf);
    let mut buffer_size: libc::c_int =
        inbytesleft.wrapping_mul(2 as libc::c_int as
                                     libc::c_ulong).wrapping_add(16 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
            as libc::c_int;
    let mut buffer_pos: libc::c_int = 0 as libc::c_int;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outbuf;
    let mut outbytesleft;
    let mut rc;
    's_74:
        loop  {
            buffer_size =
                (buffer_size as
                     libc::c_ulong).wrapping_add(inbytesleft.wrapping_mul(2 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong))
                    as libc::c_int as libc::c_int;
            buffer =
                realloc(buffer as *mut libc::c_void,
                        buffer_size as libc::c_ulong) as *mut libc::c_char;
            loop  {
                outbuf = buffer.offset(buffer_pos as isize);
                outbytesleft = (buffer_size - buffer_pos) as size_t;
                iconv((*pool).to_wc_desc, 0 as *mut *mut libc::c_char,
                      0 as *mut size_t, 0 as *mut *mut libc::c_char,
                      0 as *mut size_t);
                rc =
                    iconv((*pool).to_wc_desc, &mut inbuf, &mut inbytesleft,
                          &mut outbuf, &mut outbytesleft) as libc::c_int;
                buffer_pos =
                    outbuf.wrapping_offset_from(buffer) as libc::c_long as
                        libc::c_int;
                if rc == -(1 as libc::c_int) &&
                       *__errno_location() == 7 as libc::c_int {
                    break ;
                }
                if !(rc == -(1 as libc::c_int) &&
                         (*__errno_location() == 84 as libc::c_int ||
                              *__errno_location() == 22 as libc::c_int)) {
                    break 's_74 ;
                }
                // just copy this char as it is (e.g. when input is broken utf-8 with some latin1 chars)
                if outbytesleft <
                       ::std::mem::size_of::<wchar_t>() as libc::c_ulong {
                    break ;
                }
                *(outbuf as *mut wchar_t) =
                    *(inbuf as *mut libc::c_uchar) as wchar_t;
                buffer_pos =
                    (buffer_pos as
                         libc::c_ulong).wrapping_add(::std::mem::size_of::<wchar_t>()
                                                         as libc::c_ulong) as
                        libc::c_int as libc::c_int;
                inbuf = inbuf.offset(1);
                inbytesleft = inbytesleft.wrapping_sub(1)
            }
        }
    if rc == -(1 as libc::c_int) {
        free(buffer as *mut libc::c_void);
        pthread_mutex_unlock(&mut (*pool).mtx);
        return 0 as *const wchar_t
    }
    if outbytesleft < ::std::mem::size_of::<wchar_t>() as libc::c_ulong {
        buffer =
            realloc(buffer as *mut libc::c_void,
                    (buffer_size as
                         libc::c_ulong).wrapping_add(::std::mem::size_of::<wchar_t>()
                                                         as libc::c_ulong)) as
                *mut libc::c_char
    }
    *(outbuf as *mut wchar_t) = 0 as libc::c_int;
    pthread_mutex_unlock(&mut (*pool).mtx);
    return stfl_ipool_add(pool, buffer as *mut libc::c_void) as
               *const wchar_t;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_ipool_fromwc(pool: *mut stfl_ipool,
                                           buf: *const wchar_t)
 -> *const libc::c_char {
    if pool.is_null() || buf.is_null() { return 0 as *const libc::c_char }
    pthread_mutex_lock(&mut (*pool).mtx);
    if strcmp(b"WCHAR_T\x00" as *const u8 as *const libc::c_char,
              (*pool).code) == 0 {
        pthread_mutex_unlock(&mut (*pool).mtx);
        return buf as *mut libc::c_char
    }
    if (*pool).from_wc_desc == -(1 as libc::c_int) as iconv_t {
        (*pool).from_wc_desc =
            iconv_open((*pool).code,
                       b"WCHAR_T\x00" as *const u8 as *const libc::c_char)
    }
    if (*pool).from_wc_desc == -(1 as libc::c_int) as iconv_t {
        pthread_mutex_unlock(&mut (*pool).mtx);
        return 0 as *const libc::c_char
    }
    let mut inbuf: *mut libc::c_char = buf as *mut libc::c_char;
    let mut inbytesleft: size_t =
        wcslen(buf).wrapping_mul(::std::mem::size_of::<wchar_t>() as
                                     libc::c_ulong);
    let mut buffer_size: libc::c_int =
        inbytesleft.wrapping_add(16 as libc::c_int as libc::c_ulong) as
            libc::c_int;
    let mut buffer_pos: libc::c_int = 0 as libc::c_int;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outbuf;
    let mut outbytesleft;
    let mut rc;
    's_74:
        loop  {
            buffer_size =
                (buffer_size as libc::c_ulong).wrapping_add(inbytesleft) as
                    libc::c_int as libc::c_int;
            buffer =
                realloc(buffer as *mut libc::c_void,
                        buffer_size as libc::c_ulong) as *mut libc::c_char;
            loop  {
                outbuf = buffer.offset(buffer_pos as isize);
                outbytesleft = (buffer_size - buffer_pos) as size_t;
                iconv((*pool).from_wc_desc, 0 as *mut *mut libc::c_char,
                      0 as *mut size_t, 0 as *mut *mut libc::c_char,
                      0 as *mut size_t);
                rc =
                    iconv((*pool).from_wc_desc, &mut inbuf, &mut inbytesleft,
                          &mut outbuf, &mut outbytesleft) as libc::c_int;
                buffer_pos =
                    outbuf.wrapping_offset_from(buffer) as libc::c_long as
                        libc::c_int;
                if rc == -(1 as libc::c_int) &&
                       *__errno_location() == 7 as libc::c_int {
                    break ;
                }
                if !(rc == -(1 as libc::c_int) &&
                         (*__errno_location() == 84 as libc::c_int ||
                              *__errno_location() == 22 as libc::c_int)) {
                    break 's_74 ;
                }
                // just copy a '?' to the output stream
                if outbytesleft < 1 as libc::c_int as libc::c_ulong {
                    break ;
                }
                *outbuf = '?' as i32 as libc::c_char;
                buffer_pos += 1;
                inbuf =
                    inbuf.offset(::std::mem::size_of::<wchar_t>() as
                                     libc::c_ulong as isize);
                inbytesleft =
                    (inbytesleft as
                         libc::c_ulong).wrapping_sub(::std::mem::size_of::<wchar_t>()
                                                         as libc::c_ulong) as
                        size_t as size_t
            }
        }
    if rc == -(1 as libc::c_int) {
        free(buffer as *mut libc::c_void);
        pthread_mutex_unlock(&mut (*pool).mtx);
        return 0 as *const libc::c_char
    }
    if outbytesleft < 1 as libc::c_int as libc::c_ulong {
        buffer =
            realloc(buffer as *mut libc::c_void,
                    (buffer_size + 1 as libc::c_int) as libc::c_ulong) as
                *mut libc::c_char
    }
    *outbuf = 0 as libc::c_int as libc::c_char;
    pthread_mutex_unlock(&mut (*pool).mtx);
    return stfl_ipool_add(pool, buffer as *mut libc::c_void) as
               *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_ipool_flush(pool: *mut stfl_ipool) {
    if pool.is_null() { return }
    pthread_mutex_lock(&mut (*pool).mtx);
    while !(*pool).list.is_null() {
        let l: *mut stfl_ipool_entry = (*pool).list;
        (*pool).list = (*l).next;
        free((*l).data);
        free(l as *mut libc::c_void);
    }
    pthread_mutex_unlock(&mut (*pool).mtx);
}
#[no_mangle]
pub unsafe extern "C" fn stfl_ipool_destroy(pool: *mut stfl_ipool) {
    if pool.is_null() { return }
    stfl_ipool_flush(pool);
    free((*pool).code as *mut libc::c_void);
    if (*pool).to_wc_desc != -(1 as libc::c_int) as iconv_t {
        iconv_close((*pool).to_wc_desc);
    }
    if (*pool).from_wc_desc != -(1 as libc::c_int) as iconv_t {
        iconv_close((*pool).from_wc_desc);
    }
    free(pool as *mut libc::c_void);
}
