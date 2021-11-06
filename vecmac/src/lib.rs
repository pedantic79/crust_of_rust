#[macro_export]
macro_rules! avec {
    // () => {
    //     Vec::new()
    // };
    ( $($elem:expr),* $(,)? ) => {{
        const COUNT: usize = count_tts!($($elem)*);

        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity(COUNT);
        $( vs.push($elem); )*
        vs
    }};
    // ($elem:expr; $count:expr) => {{
    //     let count = $count;
    //     let mut vs = Vec::with_capacity(count);
    //     let x = $elem;
    //     vs.expand(::std::iter::repeat(x).take(count));
    //     vs
    // }}

    ($elem:expr; $count:expr) => {{
        let mut vs = Vec::new();
        vs.resize($count, $elem);
        vs
    }};
}

#[allow(unused_macros)]
macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}
#[allow(unused_macros)]
macro_rules! count_tts {
    ($($tts:tt)*) => {<[()]>::len(&[$(replace_expr!($tts ())),*])};
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty_vec() {
        let x: Vec<i32> = avec![];
        assert!(x.is_empty());
    }

    #[test]
    fn single() {
        let x: Vec<i32> = avec![42];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 1);
        assert_eq!(x[0], 42);
    }

    #[test]
    fn double() {
        let x: Vec<i32> = avec![42, 43];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 2);
        assert_eq!(x[0], 42);
        assert_eq!(x[1], 43);
    }

    #[test]
    fn trailing() {
        let x: Vec<i32> = avec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27,
        ];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 27);
    }

    #[test]
    fn clone_2() {
        let x: Vec<i32> = avec![42; 2];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 2);
        assert_eq!(x[0], 42);
        assert_eq!(x[1], 42);
    }

    #[test]
    fn clone_2_nonliteral() {
        let mut y = Some(42);
        let x: Vec<i32> = avec![y.take().unwrap(); 2];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 2);
        assert_eq!(x[0], 42);
        assert_eq!(x[1], 42);
    }
}

/// ```compile_fail
/// let x: Vec<u32> = vecmac::avec![42; "foo"];
/// ```
#[allow(dead_code)]
struct CompileFailTest;
