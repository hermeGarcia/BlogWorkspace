pub fn header() -> String {
    r#"
    pub trait BinaryOp<L, R> {
        type Returns;
        fn compute(_: L, _: R) -> Self::Returns {
            panic!()
        }
    }
    pub trait UnaryOp<L> {
        type Returns;
        fn compute(_: L) -> Self::Returns {
            panic!()
        }
    }
    struct True;
    struct False;
    struct Not; 
    impl UnaryOp<True> for Not {
        type Returns = False;
    } 
    impl UnaryOp<False> for Not {
        type Returns = True;
    }
    struct And;
    impl BinaryOp<True, True> for And {
        type Returns = True;
    }
    impl BinaryOp<True, False> for And {
        type Returns = False;
    }
    impl<T> BinaryOp<False, T> for And {
        type Returns = False;
    }
    struct Or;
    impl BinaryOp<False, False> for Or {
        type Returns = False;
    }
    impl BinaryOp<True, False> for Or {
        type Returns = True;
    }
    impl<T> BinaryOp<T, True> for Or {
        type Returns = True;
    }
    "#
    .to_string()
}

pub fn circuit_start() -> String {
    r#"
    fn main() {
    "#
    .to_string()
}

pub fn circuit_end() -> String {
    r#"
    }
    "#
    .to_string()
}
