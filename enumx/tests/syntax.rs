use enumx::*;

#[allow( dead_code )]
fn syntax() {
    fn foo( i: i32 ) -> Enum!( String, &'static str ) {
        if i >= 0 { String::from("positive").into_enum() } else { "negative".into_enum() }
    }

    #[enumx] fn bar( i: i32 ) -> Enum!( String, usize, &'static str ) {
        #[ty_pat] match foo(i) {
            String(s) => if s.len() % 2 == 0 {
                s.len().into_enum()
            } else {
                s.into_enum()
            },
            TyPat::<&'static str>(s) => s.into_enum(),
        }
    }

    #[enumx] fn bar_v2( i: i32 ) -> Enum!( String, usize, &'static str ) {
        #[ty_pat(gen_variants)] match foo(i) {
            String(s) => if s.len() % 2 == 0 {
                s.len().into_enum()
            } else {
                s.into_enum()
            },
        }
    }

    #[enumx] fn bar_v3( i: i32 ) -> Enum!( String, usize, &'static str ) {
        #[ty_pat(gen &'static str)] match foo(i) {
            String(s) => if s.len() % 2 == 0 {
                s.len().into_enum()
            } else {
                s.into_enum()
            },
        }
    }

    #[enumx] fn baz( i: i32 ) -> Enum!( String, &'static str ) {
        #[ty_pat(gen_variants)] match bar(i) {
            usize(v) => v.to_string().into_enum(),
        }
    }
}
