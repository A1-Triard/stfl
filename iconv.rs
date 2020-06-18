use libc::*;
use crate::public::{pthread_mutex_t, malloc, strdup, strcmp, strlen, realloc, __errno_location, wcslen, pthread_mutex_init, pthread_mutex_lock, pthread_mutex_unlock, free};

extern "C" {
    #[no_mangle]
    fn iconv_open(__tocode: *const c_char,
                  __fromcode: *const c_char) -> iconv_t;
    #[no_mangle]
    fn iconv(__cd: iconv_t, __inbuf: *mut *mut c_char,
             __inbytesleft: *mut size_t, __outbuf: *mut *mut c_char,
             __outbytesleft: *mut size_t) -> size_t;
    #[no_mangle]
    fn iconv_close(__cd: iconv_t) -> c_int;
}

pub type size_t = c_ulong;
pub type wchar_t = c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stfl_ipool {
    pub to_wc_desc: iconv_t,
    pub from_wc_desc: iconv_t,
    pub code: *mut c_char,
    pub list: *mut stfl_ipool_entry,
    pub mtx: pthread_mutex_t,
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
    pub data: *mut c_void,
    pub next: *mut stfl_ipool_entry,
}
pub type iconv_t = *mut c_void;
#[no_mangle]
pub unsafe extern "C" fn stfl_ipool_create(code: *const c_char)
 -> *mut stfl_ipool {
    let mut pool: *mut stfl_ipool =
        malloc(::std::mem::size_of::<stfl_ipool>() as c_ulong) as
            *mut stfl_ipool;
    pthread_mutex_init(&mut (*pool).mtx, 0 as *const pthread_mutexattr_t);
    (*pool).to_wc_desc = -(1 as c_int) as iconv_t;
    (*pool).from_wc_desc = -(1 as c_int) as iconv_t;
    (*pool).code = strdup(code);
    (*pool).list = 0 as *mut stfl_ipool_entry;
    return pool;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_ipool_add(pool: *mut stfl_ipool,
                                        data: *mut c_void)
 -> *mut c_void {
    let mut entry: *mut stfl_ipool_entry =
        malloc(::std::mem::size_of::<stfl_ipool_entry>() as c_ulong) as
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
                                         buf: *const c_char)
 -> *const wchar_t {
    if pool.is_null() || buf.is_null() { return 0 as *const wchar_t }
    pthread_mutex_lock(&mut (*pool).mtx);
    if strcmp(b"WCHAR_T\x00" as *const u8 as *const c_char,
              (*pool).code) == 0 {
        pthread_mutex_unlock(&mut (*pool).mtx);
        return buf as *mut wchar_t
    }
    if (*pool).to_wc_desc == -(1 as c_int) as iconv_t {
        (*pool).to_wc_desc =
            iconv_open(b"WCHAR_T\x00" as *const u8 as *const c_char,
                       (*pool).code)
    }
    if (*pool).to_wc_desc == -(1 as c_int) as iconv_t {
        pthread_mutex_unlock(&mut (*pool).mtx);
        return 0 as *const wchar_t
    }
    let mut inbuf: *mut c_char = buf as *mut c_char;
    let mut inbytesleft: size_t = strlen(buf);
    let mut buffer_size: c_int =
        inbytesleft.wrapping_mul(2 as c_int as
                                     c_ulong).wrapping_add(16 as
                                                                     c_int
                                                                     as
                                                                     c_ulong)
            as c_int;
    let mut buffer_pos: c_int = 0 as c_int;
    let mut buffer: *mut c_char = 0 as *mut c_char;
    let mut outbuf;
    let mut outbytesleft;
    let mut rc;
    's_74:
        loop  {
            buffer_size =
                (buffer_size as
                     c_ulong).wrapping_add(inbytesleft.wrapping_mul(2 as
                                                                              c_int
                                                                              as
                                                                              c_ulong))
                    as c_int as c_int;
            buffer =
                realloc(buffer as *mut c_void,
                        buffer_size as c_ulong) as *mut c_char;
            loop  {
                outbuf = buffer.offset(buffer_pos as isize);
                outbytesleft = (buffer_size - buffer_pos) as size_t;
                iconv((*pool).to_wc_desc, 0 as *mut *mut c_char,
                      0 as *mut size_t, 0 as *mut *mut c_char,
                      0 as *mut size_t);
                rc =
                    iconv((*pool).to_wc_desc, &mut inbuf, &mut inbytesleft,
                          &mut outbuf, &mut outbytesleft) as c_int;
                buffer_pos =
                    outbuf.wrapping_offset_from(buffer) as c_long as
                        c_int;
                if rc == -(1 as c_int) &&
                       *__errno_location() == 7 as c_int {
                    break ;
                }
                if !(rc == -(1 as c_int) &&
                         (*__errno_location() == 84 as c_int ||
                              *__errno_location() == 22 as c_int)) {
                    break 's_74 ;
                }
                // just copy this char as it is (e.g. when input is broken utf-8 with some latin1 chars)
                if outbytesleft <
                       ::std::mem::size_of::<wchar_t>() as c_ulong {
                    break ;
                }
                *(outbuf as *mut wchar_t) =
                    *(inbuf as *mut c_uchar) as wchar_t;
                buffer_pos =
                    (buffer_pos as
                         c_ulong).wrapping_add(::std::mem::size_of::<wchar_t>()
                                                         as c_ulong) as
                        c_int as c_int;
                inbuf = inbuf.offset(1);
                inbytesleft = inbytesleft.wrapping_sub(1)
            }
        }
    if rc == -(1 as c_int) {
        free(buffer as *mut c_void);
        pthread_mutex_unlock(&mut (*pool).mtx);
        return 0 as *const wchar_t
    }
    if outbytesleft < ::std::mem::size_of::<wchar_t>() as c_ulong {
        buffer =
            realloc(buffer as *mut c_void,
                    (buffer_size as
                         c_ulong).wrapping_add(::std::mem::size_of::<wchar_t>()
                                                         as c_ulong)) as
                *mut c_char
    }
    *(outbuf as *mut wchar_t) = 0 as c_int;
    pthread_mutex_unlock(&mut (*pool).mtx);
    return stfl_ipool_add(pool, buffer as *mut c_void) as
               *const wchar_t;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_ipool_fromwc(pool: *mut stfl_ipool,
                                           buf: *const wchar_t)
 -> *const c_char {
    if pool.is_null() || buf.is_null() { return 0 as *const c_char }
    pthread_mutex_lock(&mut (*pool).mtx);
    if strcmp(b"WCHAR_T\x00" as *const u8 as *const c_char,
              (*pool).code) == 0 {
        pthread_mutex_unlock(&mut (*pool).mtx);
        return buf as *mut c_char
    }
    if (*pool).from_wc_desc == -(1 as c_int) as iconv_t {
        (*pool).from_wc_desc =
            iconv_open((*pool).code,
                       b"WCHAR_T\x00" as *const u8 as *const c_char)
    }
    if (*pool).from_wc_desc == -(1 as c_int) as iconv_t {
        pthread_mutex_unlock(&mut (*pool).mtx);
        return 0 as *const c_char
    }
    let mut inbuf: *mut c_char = buf as *mut c_char;
    let mut inbytesleft: size_t =
        wcslen(buf).wrapping_mul(::std::mem::size_of::<wchar_t>() as
                                     c_ulong);
    let mut buffer_size: c_int =
        inbytesleft.wrapping_add(16 as c_int as c_ulong) as
            c_int;
    let mut buffer_pos: c_int = 0 as c_int;
    let mut buffer: *mut c_char = 0 as *mut c_char;
    let mut outbuf;
    let mut outbytesleft;
    let mut rc;
    's_74:
        loop  {
            buffer_size =
                (buffer_size as c_ulong).wrapping_add(inbytesleft) as
                    c_int as c_int;
            buffer =
                realloc(buffer as *mut c_void,
                        buffer_size as c_ulong) as *mut c_char;
            loop  {
                outbuf = buffer.offset(buffer_pos as isize);
                outbytesleft = (buffer_size - buffer_pos) as size_t;
                iconv((*pool).from_wc_desc, 0 as *mut *mut c_char,
                      0 as *mut size_t, 0 as *mut *mut c_char,
                      0 as *mut size_t);
                rc =
                    iconv((*pool).from_wc_desc, &mut inbuf, &mut inbytesleft,
                          &mut outbuf, &mut outbytesleft) as c_int;
                buffer_pos =
                    outbuf.wrapping_offset_from(buffer) as c_long as
                        c_int;
                if rc == -(1 as c_int) &&
                       *__errno_location() == 7 as c_int {
                    break ;
                }
                if !(rc == -(1 as c_int) &&
                         (*__errno_location() == 84 as c_int ||
                              *__errno_location() == 22 as c_int)) {
                    break 's_74 ;
                }
                // just copy a '?' to the output stream
                if outbytesleft < 1 as c_int as c_ulong {
                    break ;
                }
                *outbuf = '?' as i32 as c_char;
                buffer_pos += 1;
                inbuf =
                    inbuf.offset(::std::mem::size_of::<wchar_t>() as
                                     c_ulong as isize);
                inbytesleft =
                    (inbytesleft as
                         c_ulong).wrapping_sub(::std::mem::size_of::<wchar_t>()
                                                         as c_ulong) as
                        size_t as size_t
            }
        }
    if rc == -(1 as c_int) {
        free(buffer as *mut c_void);
        pthread_mutex_unlock(&mut (*pool).mtx);
        return 0 as *const c_char
    }
    if outbytesleft < 1 as c_int as c_ulong {
        buffer =
            realloc(buffer as *mut c_void,
                    (buffer_size + 1 as c_int) as c_ulong) as
                *mut c_char
    }
    *outbuf = 0 as c_int as c_char;
    pthread_mutex_unlock(&mut (*pool).mtx);
    return stfl_ipool_add(pool, buffer as *mut c_void) as
               *const c_char;
}
#[no_mangle]
pub unsafe extern "C" fn stfl_ipool_flush(pool: *mut stfl_ipool) {
    if pool.is_null() { return }
    pthread_mutex_lock(&mut (*pool).mtx);
    while !(*pool).list.is_null() {
        let l: *mut stfl_ipool_entry = (*pool).list;
        (*pool).list = (*l).next;
        free((*l).data);
        free(l as *mut c_void);
    }
    pthread_mutex_unlock(&mut (*pool).mtx);
}
#[no_mangle]
pub unsafe extern "C" fn stfl_ipool_destroy(pool: *mut stfl_ipool) {
    if pool.is_null() { return }
    stfl_ipool_flush(pool);
    free((*pool).code as *mut c_void);
    if (*pool).to_wc_desc != -(1 as c_int) as iconv_t {
        iconv_close((*pool).to_wc_desc);
    }
    if (*pool).from_wc_desc != -(1 as c_int) as iconv_t {
        iconv_close((*pool).from_wc_desc);
    }
    free(pool as *mut c_void);
}
