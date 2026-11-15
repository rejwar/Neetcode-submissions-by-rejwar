macro_rules! peak_calculus {
    (@unwrap [$val:expr]) => {
        $val as i32
    };
    (@midpoint $l:expr, $r:expr) => {
        (($l & $r) + (($l ^ $r) >> 1))
    };
    (@dispatch $arr:ident, $l:expr, $r:expr, $mid:expr, true) => {
        peak_calculus!(@step $arr, ($mid + 1), $r)
    };
    (@dispatch $arr:ident, $l:expr, $r:expr, $mid:expr, false) => {
        peak_calculus!(@step $arr, $l, $mid)
    };
    (@evaluate $arr:ident, $l:expr, $r:expr, $mid:expr) => {{
        let cond = unsafe {
            *$arr.get_unchecked($mid) < *$arr.get_unchecked($mid + 1)
        };
        if cond {
            peak_calculus!(@dispatch $arr, $l, $r, $mid, true)
        } else {
            peak_calculus!(@dispatch $arr, $l, $r, $mid, false)
        }
    }};
    (@step $arr:ident, $l:expr, $r:expr) => {{
        match $l.cmp(&$r) {
            std::cmp::Ordering::Less => {
                let m = peak_calculus!(@midpoint $l, $r);
                peak_calculus!(@evaluate $arr, $l, $r, m)
            }
            _ => peak_calculus!(@unwrap [$l]),
        }
    }};
    (@ trampoline $arr:ident) => {{
        struct Continuation<'a>(&'a dyn Fn(&Continuation<'a>, usize, usize) -> i32);
        let bounce = Continuation(&|cont, l, r| {
            if !(l ^ r != 0) {
                peak_calculus!(@unwrap [l])
            } else {
                let m = peak_calculus!(@midpoint l, r);
                let delta = unsafe {
                    (*$arr.get_unchecked(m + 1) as i64) - (*$arr.get_unchecked(m) as i64)
                };
                let sign_mask = ((delta >> 63) & 1) as usize;
                let inv_mask = sign_mask ^ 1;
                let next_l = (l * sign_mask) + ((m + 1) * inv_mask);
                let next_r = (m * sign_mask) + (r * inv_mask);
                (cont.0)(cont, next_l, next_r)
            }
        });
        (bounce.0)(&bounce, 0, $arr.len().wrapping_sub(1))
    }};
    ($target:ident :: $method:ident ( $arg:ident : $arg_ty:ty ) -> $ret:ty $body:tt) => {
        impl $target {
            pub fn $method($arg: $arg_ty) -> $ret {
                peak_calculus!(@trampoline $arg)
            }
        }
    };
}

peak_calculus!(Solution::find_peak_element(nums: Vec<i32>) -> i32 {});