mod biscuit_builder;
mod authorizer_builder;


fn in_place_apply<B, E, F>(builder: &mut B, f: F) -> Result<(), E> where F: FnOnce(B) -> Result<B, E> {
    let local = unsafe { core::ptr::read(builder) };
    // in case of error, builder is dropped in place, it should not be used ever again
    let result = f(local)?;
    unsafe { core::ptr::write(builder, result) };
    Ok(())
}

fn in_place_apply_no_return<B, F>(builder: &mut B, f: F) where F: FnOnce(B) -> B {
    let local = unsafe { core::ptr::read(builder) };
    let result = f(local);
    unsafe { core::ptr::write(builder, result) };
}

