// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn predefined( max_index: usize ) -> String {
    let mut unamed = String::new();
    let mut variant_indices = String::new();
    let mut impl_from_variant = String::new();

    let mut generics = "T0".to_string();
    let mut variants = "_0(T0)".to_string();

    for x in 1..max_index {
        variant_indices.push_str( &format!( "\npub struct V{};", x ));

        generics.push_str( &format!( ",T{}", x ));
        variants.push_str( &format!( ", _{x}(T{x})", x = x ));

        unamed.push_str( &format!( r#"
#[derive( Debug, PartialEq, Eq, PartialOrd, Ord )]
pub enum Enum{X}<{generics}> {{ {variants} }}"#,
            X = x+1, generics = generics, variants = variants
        ));

        for index_fv in 0..=x {
            impl_from_variant.push_str( &format!( r#"
impl<{generics}> FromVariant<T{i},V{i},VA> for Enum{X}<{generics}> {{
    fn from_variant( variant: T{i} ) -> Self {{
        Enum{X}::_{i}( variant )
    }}
}}
"#,
                generics = generics, i = index_fv, X = x+1
            ));
        }

        let impl_exchange_from_enum0 = format!( r#"
impl<{generics}> ExchangeFrom<Enum0,Nil,AA> for Enum{X}<{generics}> {{
    fn exchange_from( src: Enum0 ) -> Self {{ match src {{}} }}
}}"#,
             generics = generics, X = x+1
        );

        let impl_exchange_from_enum1 = format!( r#"
impl<Indices,S0,{generics}> ExchangeFrom<Enum1<S0>,Indices,AA> for Enum{X}<{generics}>
    where Self : FromVariant<S0,Indices,VA>
{{
    fn exchange_from( src: Enum1<S0> ) -> Self {{
        match src {{
            Enum1::_0(v) => <Self as FromVariant<S0,Indices,VA>>::from_variant(v),
        }}
    }}
}}"#,
             generics = generics, X = x+1
        );

        let mut impl_exchange_from = String::new();
        for src_idx in 2..=x+1 {
            let mut rest_arms = String::new();

            let src_generics = (2..src_idx).fold( "S1".to_string(), |acc,s| format!( "{},S{}", acc, s ));

            for v in 1..src_idx {
                rest_arms.push_str( &format!( r#"
            Enum{src_idx}::_{v}(v) => <Self as ExchangeFrom<Enum{descent_idx}<{src_generics}>,R,AA>>::exchange_from( Enum{descent_idx}::_{u}(v) ),"#,
                    src_generics = src_generics, v = v, u = v-1, src_idx = src_idx, descent_idx = src_idx-1,
                ));
            }

            impl_exchange_from.push_str( &format!( r#"
impl<L,R,S0,{src_generics},{generics}> ExchangeFrom<Enum{src_idx}<S0,{src_generics}>,LR<L,R>,AA> for Enum{dest_idx}<{generics}>
    where Self : FromVariant<S0,L,VA>
               + ExchangeFrom<Enum{descent_idx}<{src_generics}>,R,AA>
{{
    fn exchange_from( src: Enum{src_idx}<S0,{src_generics}> ) -> Self {{
        match src {{
            Enum{src_idx}::_0(v) => <Self as FromVariant<S0,L,VA>>::from_variant(v),{rest_arms}
        }}
    }}
}}
"#,
                src_generics = src_generics, generics = generics, src_idx = src_idx, dest_idx = x+1, descent_idx = src_idx-1,
                rest_arms = rest_arms
            ));
        }
        unamed.push_str( &format!( "\n{}\n{}\n{}", impl_exchange_from_enum0, impl_exchange_from_enum1, impl_exchange_from ));
    }

    let enum_macros = format!( r#"
#[macro_export]
macro_rules! Enum {{
{}
}}
"#,
        {
            let mut rules = String::new();
            let mut in_args = "$t0:ty".to_string();
            let mut out_args = "$t0".to_string();
            for i in 1..max_index {
                rules.push_str( &format!( "    ( {} ) => {{ Enum{}<{}> }};\n", in_args, i, out_args ));
                in_args.push_str( &format!( ", $t{}:ty", i ));
                out_args.push_str( &format!( ",$t{}", i ));
            }
            rules
        }
    );

    let prelude = format!( r#"
pub mod prelude {{
    pub use super::Exchange;
    pub use super::ExchangeFrom;
    pub use super::ExchangeInto;
    pub use super::FromVariant;
    pub use super::IntoEnum;
{}}}
"#,
        (0..=max_index).fold( String::new(), |acc,idx| format!( "{}    pub use super::Enum{};\n", acc, idx )));

    let license = r#"// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>"#;

    format!( "{}\n{}{}\n{}{}{}", license, unamed, variant_indices, impl_from_variant, enum_macros, prelude )
}

fn main() {
    let out_dir = env::var( "OUT_DIR" ).unwrap();
    let dest_path = Path::new( &out_dir ).join( "predefined.rs" );
    let mut f = File::create( &dest_path ).unwrap();

    let max_variants = env::var( "ENUMX_MAX_VARIANTS" )
        .map( |cnt| cnt.parse::<usize>() ).unwrap_or( Ok(16) ).unwrap();
    f.write_all( predefined( max_variants ).as_bytes() ).unwrap();
}
