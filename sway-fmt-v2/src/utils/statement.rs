use crate::{
    fmt::*,
    utils::comments::{ByteSpan, CommentVisitor},
};
use std::fmt::Write;
use sway_parse::{Statement, StatementLet};
use sway_types::Spanned;

impl Format for Statement {
    fn format(
        &self,
        formatted_code: &mut FormattedCode,
        formatter: &mut Formatter,
    ) -> Result<(), FormatterError> {
        match self {
            Self::Let(let_stmt) => let_stmt.format(formatted_code, formatter)?,
            Self::Item(item) => item.format(formatted_code, formatter)?,
            Self::Expr {
                expr,
                semicolon_token_opt,
            } => {
                expr.format(formatted_code, formatter)?;
                if let Some(semicolon) = semicolon_token_opt {
                    write!(formatted_code, "{}", semicolon.span().as_str())?;
                }
            }
        }
        Ok(())
    }
}

impl Format for StatementLet {
    fn format(
        &self,
        formatted_code: &mut FormattedCode,
        formatter: &mut Formatter,
    ) -> Result<(), FormatterError> {
        // Add indent level + `let `
        write!(
            formatted_code,
            "{}{} ",
            formatter.shape.indent.to_string(formatter),
            self.let_token.span().as_str()
        )?;
        // pattern
        self.pattern.format(formatted_code, formatter)?;
        // `: Ty`
        if let Some(ty) = &self.ty_opt {
            write!(formatted_code, "{} ", ty.0.span().as_str())?;
            ty.1.format(formatted_code, formatter)?;
        }
        // ` = `
        write!(formatted_code, " {} ", self.eq_token.span().as_str())?;
        // expr
        self.expr.format(formatted_code, formatter)?;
        // `;\n`
        writeln!(formatted_code, "{}", self.semicolon_token.span().as_str())?;

        Ok(())
    }
}

impl CommentVisitor for Statement {
    fn collect_spans(&self) -> Vec<ByteSpan> {
        match self {
            Statement::Let(statement_let) => statement_let.collect_spans(),
            Statement::Item(item) => item.collect_spans(),
            Statement::Expr {
                expr,
                semicolon_token_opt,
            } => {
                let mut collected_spans = expr.collect_spans();
                if let Some(semicolon_token) = semicolon_token_opt {
                    collected_spans.push(ByteSpan::from(semicolon_token.span()));
                }
                collected_spans
            }
        }
    }
}

impl CommentVisitor for StatementLet {
    fn collect_spans(&self) -> Vec<ByteSpan> {
        // Add let token's ByteSpan
        let mut collected_spans = vec![ByteSpan::from(self.let_token.span())];
        // Add pattern's ByteSpan
        collected_spans.append(&mut self.pattern.collect_spans());
        // Add ty's ByteSpan if it exists
        if let Some(ty) = &self.ty_opt {
            collected_spans.push(ByteSpan::from(ty.0.span()));
            // TODO: determine if we are allowing comments between `:` and ty
            collected_spans.append(&mut ty.1.collect_spans());
        }
        // Add eq token's ByteSpan
        collected_spans.push(ByteSpan::from(self.eq_token.span()));
        // Add Expr's ByteSpan
        collected_spans.append(&mut self.expr.collect_spans());
        collected_spans.push(ByteSpan::from(self.semicolon_token.span()));
        collected_spans
    }
}
