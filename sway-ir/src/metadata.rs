///! Associated metadata attached mostly to values.
///!
///! Each value (instruction, function argument or constant) has associated metadata which helps
///! describe properties which aren't required for code generation, but help with other
///! introspective tools (e.g., the debugger) or compiler error messages.
///!
///! The metadata themselves are opaque to `sway-ir` and are represented with simple value types;
///! integers, strings, symbols (tags) and lists.

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct MetadataIndex(pub generational_arena::Index);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Metadatum {
    Integer(u64),
    Index(MetadataIndex),
    String(String),
    Struct(String, Vec<Metadatum>), // XXX need to verify tag has no spaces content
    List(Vec<MetadataIndex>),
}

/// Combine two metadata indices into one.
///
/// When multiple indices are attached to an IR value or function they must go in a list.  It is
/// rare for `MetadataIndex` to exist outside of an `Option` though, so we may want to combine two
/// optional indices when we might end up with only one or the other, or maybe even None.
///
/// This function conveniently has all the logic to return the simplest combination of two
/// `Option<MetadataIndex>`s.
pub fn combine(
    context: &mut crate::context::Context,
    md_idx_a: &Option<MetadataIndex>,
    md_idx_b: &Option<MetadataIndex>,
) -> Option<MetadataIndex> {
    match (md_idx_a, md_idx_b) {
        (None, None) => None,
        (Some(_), None) => *md_idx_a,
        (None, Some(_)) => *md_idx_b,
        (Some(idx_a), Some(idx_b)) => {
            // Rather than potentially making lists of lists, if either are already list we can
            // merge them together.
            let mut new_list = Vec::new();
            if let Metadatum::List(lst_a) = &context.metadata[idx_a.0] {
                new_list.append(&mut lst_a.clone());
            } else {
                new_list.push(*idx_a);
            }
            if let Metadatum::List(lst_b) = &context.metadata[idx_b.0] {
                new_list.append(&mut lst_b.clone());
            } else {
                new_list.push(*idx_b);
            }
            Some(MetadataIndex(
                context.metadata.insert(Metadatum::List(new_list)),
            ))
        }
    }
}
