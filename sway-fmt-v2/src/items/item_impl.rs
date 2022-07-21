use crate::{
    fmt::{Format, FormattedCode, Formatter},
    utils::comments::{ByteSpan, CommentVisitor},
    FormatterError,
};
use sway_parse::ItemImpl;
use sway_types::Spanned;

impl Format for ItemImpl {
    fn format(
        &self,
        _formatted_code: &mut FormattedCode,
        _formatter: &mut Formatter,
    ) -> Result<(), FormatterError> {
        todo!()
    }
}
impl CommentVisitor for ItemImpl {
    fn collect_spans(&self) -> Vec<ByteSpan> {
        let mut collected_spans = vec![ByteSpan::from(self.impl_token.span())];
        if let Some(generic) = &self.generic_params_opt {
            collected_spans.push(ByteSpan::from(generic.parameters.span()));
        }
        if let Some(trait_tuple) = &self.trait_opt {
            collected_spans.append(&mut trait_tuple.collect_spans());
        }
        collected_spans.append(&mut self.ty.collect_spans());
        // TODO add where
        collected_spans.append(&mut self.contents.collect_spans());
        collected_spans
    }
}
