#[macro_use]
extern crate serde_derive;
extern crate graphql_parser;

use neon::prelude::*;
// use wasm_bindgen::prelude::*;
// 
pub mod lexer;
pub mod parser;
pub mod ast;

fn parse(mut cx: FunctionContext) -> JsResult<JsString> {
    let x = cx.argument::<JsString>(0)?.value();
  
    let ast = match graphql_parser::parse_query::<&str>(&x) {
        Ok(doc) => "success".to_string(),
        Err(e) => "error".to_string()
    };
    
    Ok(cx.string(ast))
}

fn parse2(mut cx: FunctionContext) -> JsResult<JsString> {
    let x = cx.argument::<JsString>(0)?.value();
    let mut lexer = lexer::Lexer::new(&x);
    let result = parser::parse(&mut lexer).unwrap();
    // JsValue::from_serde(&result).unwrap()
    // 
    Ok(cx.string("foo"))
}
register_module!(mut cx, {
    cx.export_function("parse", parse);
    cx.export_function("parse2", parse2)
});
