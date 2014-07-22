use {void_t, int_t, size_t, char_t};
use locale::{locale_t};

extern {
    pub fn memccpy(dest: *mut void_t, src: *const void_t, c: int_t, n: size_t) -> *mut void_t;
    pub fn memchr(s: *const void_t, c: int_t, n: size_t) -> *mut void_t;
    pub fn memcmp(s1: *const void_t, s2: *const void_t, n: size_t) -> int_t;
    pub fn memcpy(dest: *mut void_t, src: *const void_t, n: size_t) -> *mut void_t;
    pub fn memmove(dest: *mut void_t, src: *const void_t, n: size_t) -> *mut void_t;
    pub fn memset(s: *mut void_t, c: int_t, n: size_t) -> *mut void_t;
    pub fn stpcpy(dest: *mut char_t, src: *const char_t) -> *mut char_t;
    pub fn stpncpy(dest: *mut char_t, src: *const char_t, n: size_t) -> *mut char_t;
    pub fn strcasecmp(s1: *const char_t, s2: *const char_t) -> int_t;
    pub fn strcat(dest: *mut char_t, src: *const char_t) -> *mut char_t;
    pub fn strchr(s: *const char_t, c: int_t) -> *mut char_t;
    pub fn strcmp(s1: *const char_t, s2: *const char_t) -> int_t;
    pub fn strcoll(s1: *const char_t, s2: *const char_t) -> int_t;
    pub fn strcoll_l(s1: *const char_t, s2: *const char_t, l: locale_t) -> int_t;
    pub fn strcpy(dest: *mut char_t, src: *const char_t) -> *mut char_t;
    pub fn strcspn(s: *const char_t, reject: *const char_t) -> size_t;
    pub fn strdup(s: *const char_t) -> *mut char_t;
    pub fn strerror(errnum: int_t) -> *mut char_t;
    pub fn strerror_l(errnum: int_t, l: locale_t) -> *mut char_t;
    pub fn strerror_r(errnum: int_t, buf: *mut char_t, buflen: size_t) -> int_t;
    pub fn strlen(s: *const char_t) -> size_t;
    pub fn strncat(dest: *mut char_t, src: *const char_t, n: size_t) -> *mut char_t;
    pub fn strncmp(s1: *const char_t, s2: *const char_t, n: size_t) -> int_t;
    pub fn strncpy(dest: *mut char_t, src: *const char_t, n: size_t) -> *mut char_t;
    pub fn strndup(string: *const char_t, n: size_t) -> *mut char_t;
    pub fn strnlen(string: *const char_t, maxlen: size_t) -> size_t;
    pub fn strpbrk(s: *const char_t, accept: *const char_t) -> *mut char_t;
    pub fn strrchr(s: *const char_t, c: int_t) -> *mut char_t;
    pub fn strsignal(sig: int_t) -> *mut char_t;
    pub fn strspn(s: *const char_t, accept: *const char_t) -> size_t;
    pub fn strstr(haystack: *const char_t, needle: *const char_t) -> *mut char_t;
    pub fn strtok(s: *mut char_t, delim: *const char_t) -> *mut char_t;
    pub fn strtok_r(s: *mut char_t, delim: *const char_t, save_ptr: *mut *mut char_t) -> *mut char_t;
    pub fn strxfrm(dest: *mut char_t, src: *const char_t, n: size_t) -> size_t;
    pub fn strxfrm_l(dest: *mut char_t, src: *const char_t, n: size_t, l: locale_t) -> size_t;
}
