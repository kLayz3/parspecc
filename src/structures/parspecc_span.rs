/// This file handles the symbol spanning during the parsing.
/// Checking the visibility of different symbols in various scopes

use std::ops::Range;

pub type Span = Range<usize>;
pub type Spanned<T> = (T, Span);

/* All the parsed indicators (variables), have a span associated with them.
 * They should only be accepted in corresponding expressions contained within their span. */
pub fn make_spanned<T>(obj: T, span: pest::Span<'_>) -> Spanned<T> {
        (obj, Span {start: span.start(), end: span.end()})
}
