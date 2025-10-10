use crate::print_wasm;
use crate::builder::in_place_apply;
use crate::wasm_export;
#[cfg(feature = "ffi")]
use crate::wasm_result::ResultKind;
use crate::wasm_result::WasmResult;
use biscuit_auth::{Authorizer, AuthorizerBuilder, Biscuit};
#[cfg(feature = "ffi")]
use biscuit_auth::{
    builder,
    datalog::{ExternFunc, SymbolTable, TemporarySymbolTable},
    format::{
        convert::{proto_id_to_token_term, token_term_to_proto_id},
        schema,
    },
};

// create a new authorizer builder
// Output:
// returnArea { data, data_len=0, kind=Ok }
// data is a pointer to the authorizer builder allocated in the wasm memory
// data_len is 0 because of the opaque type
// kind is Ok because the function never fails
wasm_export!(
    fn authorizer_builder_new() -> Box<AuthorizerBuilder> {
        Box::new(AuthorizerBuilder::new())
    }
);

// drop the authorizer builder and free the memory allocated for it
// Input:
// builder is a pointer to the authorizer builder allocated in the wasm memory
// Output:
// returnArea { data, data_len=0, kind=Ok }
// data is 0
// data_len is 0
// kind is Ok because the function never fails
wasm_export!(
    fn authorizer_builder_drop(builder: Box<AuthorizerBuilder>) {
        drop(builder);
    }
);

// build the authorizer
// Input:
// builder is a pointer to the authorizer builder allocated in the wasm memory
// token is a pointer to the biscuit token
// Output:
// returnArea { data, data_len, kind }
//
// if kind = Ok
// data is a pointer to the authorizer allocated in the wasm memory
// data_len is 0 because of the opaque type
//
// if kind = ErrBiscuit or kind = ErrSerialization
// data is the pointer to the authorizer error message allocated in the wasm memory (JSON when ErrBiscuit, plain string when ErrSerialization)
// data_len is the length of the error message
wasm_export!(
    fn authorizer_builder_build(
        builder: Box<AuthorizerBuilder>,
        token: &Biscuit,
    ) -> Result<Box<Authorizer>, biscuit_auth::error::Token> {
        let authorizer = builder.build(token)?;
        Ok(Box::new(authorizer))
    }
);

// add a code to the authorizer builder
// Input:
// builder is a pointer to the authorizer builder allocated in the wasm memory
// code {ptr, len} is a string containing the code to add
// Output:
// returnArea { data, data_len, kind }
//
// if kind = Ok
// data is 0 because no data is returned
// data_len is 0 because no data is returned
//
// if kind = ErrBiscuit or kind = ErrSerialization
// data is the pointer to the authorizer error message allocated in the wasm memory (JSON when ErrBiscuit, plain string when ErrSerialization)
// data_len is the length of the error message
wasm_export!(
    fn authorizer_builder_add_code(
        builder: &mut AuthorizerBuilder,
        code: &str,
    ) -> Result<(), biscuit_auth::error::Token> {
        in_place_apply(builder, |builder| builder.code(code))
    }
);

#[cfg(feature = "ffi")]
wasm_export!(
    fn authorizer_builder_register_extern_func(
        builder: &mut AuthorizerBuilder,
        name: &str,
        user_data: u64,
    ) {
        let func = ExternFunc(std::sync::Arc::new(move |left, right| {
            call_extern(left, right, user_data)
        }));
        crate::builder::in_place_apply_no_return(builder, |builder| {
            builder.register_extern_func(name.to_owned(), func)
        });
    }
);

#[cfg(feature = "ffi")]
wasm_export!(
    fn symbol_table_get_symbol(table: &TemporarySymbolTable, i: u64) -> WasmResult {
        match table.get_symbol(i) {
            Some(s) => WasmResult {
                kind: ResultKind::Ok,
                ptr: s.as_ptr(), // symbol should be copied by caller
                len: s.len(),
            },
            None => WasmResult {
                kind: ResultKind::Ok,
                ptr: core::ptr::null(),
                len: 0,
            },
        }
    }
);

#[cfg(feature = "ffi")]
wasm_export!(
    fn symbol_table_insert(table: &mut TemporarySymbolTable, s: &str) -> u64 {
        table.insert(s)
    }
);

#[cfg(feature = "ffi")]
fn call_extern(
    left: builder::Term,
    right: Option<builder::Term>,
    user_data: u64,
) -> Result<builder::Term, String> {
    use prost::Message;
    const NOOP: &str = "ExternFunc did nothing";
    let mut ret = WasmResult {
        kind: ResultKind::ErrSerialization,
        ptr: NOOP.as_ptr(),
        len: NOOP.len(),
    };
    let base_table = SymbolTable::default();
    let mut tmp_table = TemporarySymbolTable::new(&base_table);
    print_wasm!("{left:?} {right:?}");
    let left = token_term_to_proto_id(&left.to_datalog(&mut tmp_table))
        .encode_to_vec()
        .into_boxed_slice();
    let right = right.map(|right| {
        token_term_to_proto_id(&right.to_datalog(&mut tmp_table))
            .encode_to_vec()
            .into_boxed_slice()
    });
    print_wasm!("{left:?} {right:?}");
    unsafe {
        crate::extern_func(
            &mut tmp_table as *mut _ as *mut _,
            left.as_ptr(),
            left.len(),
            right.as_ref().map_or(core::ptr::null(), |r| r.as_ptr()),
            right.as_ref().map_or(0, |r| r.len()),
            user_data,
            &mut ret,
        );
    }
    let bytes = unsafe { crate::HostBytes::new(ret.ptr, ret.len) };
    print_wasm!("{ret:?}");
    print_wasm!("{:?}", &*bytes);
    match ret.kind {
        ResultKind::Ok => {
            let term = schema::Term::decode(&*bytes);
            let term = term.map_err(|e| e.to_string())?;
            let term = proto_id_to_token_term(&term).map_err(|e| e.to_string())?;
            let term = builder::Term::from_datalog(term, &tmp_table).map_err(|e| e.to_string());
            print_wasm!("{term:?}");
            term
        }
        _ => Err(String::from_utf8_lossy(&bytes).into_owned()),
    }
}
