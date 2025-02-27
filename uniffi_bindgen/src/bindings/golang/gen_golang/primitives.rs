use paste::paste;

use crate::backend::{CodeOracle, CodeType, Literal};
use crate::interface::{Radix, types::Type};

fn render_literal(oracle: &dyn CodeOracle, literal: &Literal) -> String {
    fn typed_number(oracle: &dyn CodeOracle, type_: &Type, num_str: String) -> String {
        match type_ {
            // special case Int32.
            Type::Int32 => num_str,
            // otherwise use constructor e.g. UInt8(x)
            Type::Int8
            | Type::UInt8
            | Type::Int16
            | Type::UInt16
            | Type::UInt32
            | Type::Int64
            | Type::UInt64
            | Type::Float32
            | Type::Float64 =>
            // XXX we should pass in the codetype itself.
                {
                    format!("{}({num_str})", oracle.find(type_).type_label(oracle))
                }
            _ => panic!("Unexpected literal: {num_str} is not a number"),
        }
    }

    match literal {
        Literal::Boolean(v) => format!("{v}"),
        Literal::String(s) => format!("\"{s}\""),
        Literal::Int(i, radix, type_) => typed_number(
            oracle,
            type_,
            match radix {
                Radix::Octal => format!("0o{i:o}"),
                Radix::Decimal => format!("{i}"),
                Radix::Hexadecimal => format!("{i:#x}"),
            },
        ),
        Literal::UInt(i, radix, type_) => typed_number(
            oracle,
            type_,
            match radix {
                Radix::Octal => format!("0o{i:o}"),
                Radix::Decimal => format!("{i}"),
                Radix::Hexadecimal => format!("{i:#x}"),
            },
        ),
        Literal::Float(string, type_) => typed_number(oracle, type_, string.clone()),
        _ => unreachable!("Literal"),
    }
}

macro_rules! impl_code_type_for_primitive {
    ($T:ty, $class_name:literal) => {
        paste! {
            pub struct $T;

            impl CodeType for $T  {
                fn type_label(&self, _oracle: &dyn CodeOracle) -> String {
                    $class_name.into()
                }

                fn canonical_name(&self, oracle: &dyn CodeOracle) -> String {
                    oracle.class_name($class_name.into())
                }

                fn literal(&self, oracle: &dyn CodeOracle, literal: &Literal) -> String {
                    render_literal(oracle, &literal)
                }
            }
        }
    };
}

impl_code_type_for_primitive!(BooleanCodeType, "bool");
impl_code_type_for_primitive!(StringCodeType, "string");
impl_code_type_for_primitive!(Int8CodeType, "int8");
impl_code_type_for_primitive!(Int16CodeType, "int16");
impl_code_type_for_primitive!(Int32CodeType, "int32");
impl_code_type_for_primitive!(Int64CodeType, "int64");
impl_code_type_for_primitive!(UInt8CodeType, "uint8");
impl_code_type_for_primitive!(UInt16CodeType, "uint16");
impl_code_type_for_primitive!(UInt32CodeType, "uint32");
impl_code_type_for_primitive!(UInt64CodeType, "uint64");
impl_code_type_for_primitive!(Float32CodeType, "float");
impl_code_type_for_primitive!(Float64CodeType, "double");
