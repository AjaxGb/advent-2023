/// Extract values from a string with a fixed pattern.
#[macro_export]
macro_rules! simple_parse {
    ($s:expr => $($pattern:tt)+) => {
        simple_parse!($s, () => $($pattern)+)
    };
    ($s:expr, $parsed:expr => _, $suffix:expr $(,)?) => {
        if let Some(_) = $s.strip_suffix($suffix) {
            Some($parsed)
        } else {
            None
        }
    };
    ($s:expr, $parsed:expr => _, $infix:expr, $($rest:tt)+) => {
        if let Some((_, s)) = $s.split_once($infix) {
            $crate::simple_parse!(s, $parsed => $($rest)+)
        } else {
            None
        }
    };
    ($s:expr, $parsed:expr => _ $(,)?) => {
        Some($parsed)
    };
    ($s:expr, $parsed:expr => parse $($type:ty)?, $suffix:expr $(,)?) => {
        if let Some(val) = $s.strip_suffix($suffix) {
            if let Ok(val) = val.parse$(::<$type>)?() {
                Some($crate::tuple_ops::TupleAppend::append($parsed, val))
            } else {
                None
            }
        } else {
            None
        }
    };
    ($s:expr, $parsed:expr => parse $($type:ty)?, $infix:expr, $($rest:tt)+) => {
        if let Some((val, s)) = $s.split_once($infix) {
            if let Ok(val) = val.parse$(::<$type>)?() {
                let parsed = $crate::tuple_ops::TupleAppend::append($parsed, val);
                $crate::simple_parse!(s, parsed => $($rest)+)
            } else {
                None
            }
        } else {
            None
        }
    };
    ($s:expr, $parsed:expr => parse $($type:ty)? $(,)?) => {
        if let Ok(val) = $s.parse$(::<$type>)?() {
            Some($crate::tuple_ops::TupleAppend::append($parsed, val))
        } else {
            None
        }
    };
    ($s:expr, $parsed:expr => str, $suffix:expr $(,)?) => {
        if let Some(val) = $s.strip_suffix($suffix) {
            Some($crate::tuple_ops::TupleAppend::append($parsed, val))
        } else {
            None
        }
    };
    ($s:expr, $parsed:expr => str, $infix:expr, $($rest:tt)+) => {
        if let Some((val, s)) = $s.split_once($infix) {
            let parsed = $crate::tuple_ops::TupleAppend::append($parsed, val);
            $crate::simple_parse!(s, parsed => $($rest)+)
        } else {
            None
        }
    };
    ($s:expr, $parsed:expr => str $(,)?) => {
        Some($crate::tuple_ops::TupleAppend::append($parsed, $s))
    };
    ($s:expr, $parsed:expr => $full:expr $(,)?) => {
        if $s == $full {
            Some($parsed)
        } else {
            None
        }
    };
    ($s:expr, $parsed:expr => $prefix:expr $(, $($rest:tt)*)?) => {
        if let Some(s) = $s.strip_prefix($prefix) {
            $crate::simple_parse!(s, $parsed => $($($rest)*)?)
        } else {
            None
        }
    };
    ($s:expr, $parsed:expr => $(,)?) => {
        if $s.is_empty() {
            Some($parsed)
        } else {
            None
        }
    };
}
