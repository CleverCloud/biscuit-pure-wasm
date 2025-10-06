use crate::builder::{in_place_apply, in_place_apply_no_return};
use crate::wasm_result::WasmResult;
use crate::{make_rng, wasm_export};
use biscuit_auth::datalog::SymbolTable;
use biscuit_auth::{Biscuit, BiscuitBuilder, KeyPair, PrivateKey};


// create a new biscuit builder
// Output:
// returnArea { data, data_len=0, is_ok = 1 }
// data is a pointer to the biscuit builder allocated in the wasm memory
// data_len is 0 because of the opaque type
// is_ok is 1 because the function never fails
wasm_export!(
    fn biscuit_builder_new() -> Box<BiscuitBuilder> {
        Box::new(BiscuitBuilder::new())
    }
);

// drop the biscuit builder
// Output:
// returnArea { data, data_len=0, is_ok = 1 }
// data is 0
// data_len is 0 because of the opaque type
// is_ok is 1 because the function never fails
wasm_export!(
    fn biscuit_builder_drop(builder: Box<BiscuitBuilder>) {
        drop(builder);
    }
);

// build a biscuit with a private key
// Input:
// builder: the biscuit builder pointer to the allocated wasm memory
// private_root_key: the private key pointer to the allocated wasm memory used as the root key
//
// Output:
// returnArea { data, data_len, is_ok }
//
// if is_ok = 1
// data is a pointer to the biscuit allocated in the wasm memory
// data_len is 0 because of the opaque type
//
// if is_ok = 0
// data is the pointer to the error message allocated in the wasm memory
// data_len is the length of the error message in bytes
wasm_export!(
    fn biscuit_builder_build_with_private_key(builder: Box<BiscuitBuilder>, private_root_key: &PrivateKey) -> Result<Box<Biscuit>, biscuit_auth::error::Token> {
        let root_keypair = KeyPair::from(private_root_key);
        let biscuit = builder.build_with_rng(&root_keypair, SymbolTable::default(), &mut make_rng())?;

        Ok(Box::new(biscuit))
    }
);

// build a biscuit with a keypair
// Input:
// builder: the biscuit builder pointer to the allocated wasm memory
// root_keypair: the keypair pointer to the allocated wasm memory used as the root key
//
// Output:
// returnArea { data, data_len, is_ok }
//
// if is_ok = 1
// data is a pointer to the biscuit allocated in the wasm memory
// data_len is 0 because of the opaque type
//
// if is_ok = 0
// data is the pointer to the error message allocated in the wasm memory
// data_len is the length of the error message in bytes
wasm_export!(
    fn biscuit_builder_build_with_key_pair(builder: Box<BiscuitBuilder>, root_keypair: &KeyPair) -> Result<Box<Biscuit>, biscuit_auth::error::Token> {
        let biscuit = builder.build_with_rng(root_keypair, SymbolTable::default(), &mut make_rng())?;

        Ok(Box::new(biscuit))
    }
);

// add a code to the biscuit builder
// Input:
// builder: the biscuit builder pointer to the allocated wasm memory
// code: the code to add
//
// Output:
// returnArea { data, data_len, is_ok }
//
// if is_ok = 1
// data is a pointer to the biscuit builder allocated in the wasm memory
// data_len is 0 because of the opaque type
//
// if is_ok = 0
// data is the pointer to the error message allocated in the wasm memory
// data_len is the length of the error message in bytes
wasm_export!(
    fn biscuit_builder_add_code(builder: &mut BiscuitBuilder, code: &str) -> Result<(), biscuit_auth::error::Token> {
        in_place_apply(builder, |builder| builder.code(code))
    }
);


// set the root key id
// Input:
// builder: the biscuit builder pointer to the allocated wasm memory
// root_key_id: the root key id
//
// Output:
// returnArea { data, data_len, is_ok }
//
// is_ok is 1 because the function never fails
// data is 0 because no data is returned
// data_len is 0 because no data is returned
wasm_export!(
    fn biscuit_builder_set_root_key_id(builder: &mut BiscuitBuilder, root_key_id: u32) -> Result<(), biscuit_auth::error::Token> {
        in_place_apply_no_return(builder, |builder| builder.root_key_id(root_key_id));
        Ok(())
    }
);

// get the biscuit builder as a string
// Input:
// builder: the biscuit builder pointer to the allocated wasm memory
//
// Output:
// returnArea { data, data_len, is_ok }
//
// data is a pointer to the biscuit builder allocated in the wasm memory
// data_len is the length of the string in bytes
// is_ok is 1 because the function never fails
wasm_export!(
    fn biscuit_builder_to_string(builder: &BiscuitBuilder) -> String {
        builder.to_string()
    }
);
