/* input must match a BASIC rule. */
use crate::*;
pub fn parse_basic(p: Pair<'_, Rule>) -> Basic {
    assert_eq!(p.as_rule(), Rule::BASIC);

    let inside = p.into_inner().next().unwrap();
    {
        let name = p.into_inner().
    }
}
