use crate::parse_tree::Purity;

use sway_ir::{Context, MetadataIndex, Value};
use sway_types::Span;

#[derive(Default)]
pub(crate) struct MetadataManager {}

pub(crate) enum StorageOperation {
    Reads,
    Writes,
    ReadsWrites,
}

impl MetadataManager {
    pub(crate) fn md_to_span(
        &self,
        context: &Context,
        md_idx: Option<MetadataIndex>,
    ) -> Option<Span> {
        todo!()
    }

    pub(crate) fn md_to_storage_op(
        &self,
        context: &Context,
        md_idx: Option<MetadataIndex>,
    ) -> Option<StorageOperation> {
        todo!()
    }

    pub(crate) fn val_to_span(&self, context: &Context, value: Value) -> Option<Span> {
        todo!()
    }

    pub(crate) fn val_to_storage_key(&self, context: &Context, value: Value) -> Option<u64> {
        todo!()
    }

    pub(crate) fn span_to_md(
        &mut self,
        context: &mut Context,
        span: &Span,
    ) -> Option<MetadataIndex> {
        todo!()
    }

    pub(crate) fn state_idx_to_md(
        &mut self,
        context: &mut Context,
        state_idx: usize,
    ) -> Option<MetadataIndex> {
        todo!()
    }

    pub(crate) fn purity_to_md(
        &mut self,
        context: &mut Context,
        purity: Purity,
    ) -> Option<MetadataIndex> {
        todo!()
        //if purity == Purity::Pure {
        //    None
        //} else {
        //    Some(MetadataIndex::get_storage_index(
        //        context,
        //        match purity {
        //            Purity::Pure => unreachable!("Already checked for Pure above."),
        //            Purity::Reads => StorageOperation::Reads,
        //            Purity::Writes => StorageOperation::Writes,
        //            Purity::ReadsWrites => StorageOperation::ReadsWrites,
        //        },
        //    ))
        //};
    }
}
