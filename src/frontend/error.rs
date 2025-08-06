#![allow(unsafe_op_in_unsafe_fn)]
use crate::frontend::arena::*;
use crate::frontend::token::*;
use crate::utils::string::*;
use crate::utils::array::*;
use crate::dref;

pub struct SyntaxError {
    msg: CogString,
    hint: Option<CogString>,
    span: Span
}

pub unsafe fn new_syntax_error (span: Span, msg: &str, hint: Option<&str>, arena: *mut Arena) -> *mut SyntaxError {
    let err = arena_alloc_ty::<SyntaxError>(arena);
    dref!(err).msg = cogstr_new(msg, arena);
    if let Some(h) = hint {
	dref!(err).hint = Some(cogstr_new(h, arena));
    } else {
	dref!(err).hint = None;
    }    
    dref!(err).span = span;
    err
}

unsafe fn underline_span (span: Span, line: CogString) {
    let line_str = cogstr_to_str(line);
    for ch in line_str.chars() {
	if !ch.is_whitespace() {
	    break;
	}
	print!(" ");
    }

    for i in 0..line_str.len() {
	if i >= span.cols && i <= span.cole {
	    if i == span.cols {
		print!("^");
	    } else {
		print!("~");
	    }
	} else {
	    print!(" ");
	}
    }
    println!()
}

unsafe fn report_syntax_error (err: *mut SyntaxError, lines: *mut CogArray<CogString>) {
    let line = *cog_arr_get(lines, 0);
    println!("error: {}: {}", dref!(err).span.display(), cogstr_to_str(dref!(err).msg));
    println!("{:>5} | {}", dref!(err).span.line, cogstr_to_str(line));
    print!("{:>5} |", " ");
    underline_span(dref!(err).span, line);
    println!("{:>5} |", " ");
}

pub unsafe fn report_syntax_errors (errors: *mut CogArray<*mut SyntaxError>, count: usize, lines: *mut CogArray<CogString>) {
    for i in 0..count {
	let err = cog_arr_get(errors, i);
	report_syntax_error(*err, lines);
	println!()
    }
}
