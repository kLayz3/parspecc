/* MEMBER statement types. */

use crate::parspecc_span::*;
use std::collections::HashMap;

#[derive(Debug,Clone)]
pub enum MemberType {
    INVALID,
    DATA16,
    DATA32,
    DATA64,
}

impl Default for MemberType {
    fn default() -> Self { MemberType::INVALID }
}

/* Primitive unpack field types. */
#[derive(Debug,Clone)]
pub enum DataType {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

/* Next enum encapsulates possible range limits, or values,
 * which can either be numbers evaluated, or a variable spanning to this point. */
#[derive(Debug,Clone)]
pub enum Any {
    Num(u32),
    Var(Spanned<String>),
}

/* ENCODE(member_id, (value = local_id)); */
#[derive(Debug,Clone)]
pub struct Encode {
    slice: Spanned<String>,
    member: Spanned<String>,
}

#[derive(Debug,Clone)]
pub enum BasicBlock {
    /* 0..5 => id = MATCH(other_id); 
     * 0..5 => id;
     * 0..5 => 0xfa; */
    Slice {
        left: Any,
        right: Any,
        ident: Any,
        match_value: Option<Any>,
    },
    
    Encode(Encode),
}

#[derive(Debug,Clone)]
pub struct Basic {
    ty: DataType,
    ident: Spanned<String>,
    block: Vec<BasicBlock>,
}

#[derive(Debug,Clone)]
pub struct Composite {
    ty: String,
    ident: Spanned<String>,
    params: HashMap<String, Any>,
}

/* Structures can hold different statements, and fall 
 * into one of the variants of the following enum. */
#[derive(Debug,Clone)]
pub enum StructStatement {
    /* dyn![max=10] {
    *      UINT32 name; 
    *  } */
    Dyn {
        max_dyn: Option<u32>,
        block: Vec<StructStatement>,
    },

    /* for(0 <= i <= 10) {
     *     UINT32 name;
     * } */
    For {
        left_bound: Any,
        right_bound: Any, 
        block: Vec<StructStatement>
    },

    Basic(Basic),
    Composite(Composite),
    Encode(Encode),
}

/* MEMBER(DATA32 name[500]); */
#[derive(Debug,Clone,Default)] 
pub struct Member {
    pub ty: MemberType,
    pub ident: Spanned<String>,
    pub max_size: Option<u32>,
}

#[derive(Debug,Clone,Default)] 
pub struct ParspeccStruct {
    pub name: Spanned<String>,
    pub params: Vec<String>,
    pub members: Vec<Member>,
    
    pub statements: Vec<StructStatement>,
}
