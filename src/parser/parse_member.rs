/* input must match a MEMBER rule. */
use crate::*;

pub fn parse_member(s: &mut ParspeccStruct, input: Pair<'_, Rule>) {
   let mut inner = input.into_inner(); 
   let mut member: Member = Member::default();
   let ty = inner.next().unwrap();
   member.ty = match ty.into_inner().next().unwrap().as_rule() {
       Rule::D16 => MemberType::DATA16, 
       Rule::D32 => MemberType::DATA32, 
       Rule::D64 => MemberType::DATA64,
       _ => unreachable!(),
    };
   let ident = inner.next().unwrap();
   member.ident = make_spanned(ident.as_str().into(), ident.as_span());
   member.max_size = match inner.next() {
       Some(x) => parse_number(x),
       None => None,
   };
}

