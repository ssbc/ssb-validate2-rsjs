// SPDX-License-Identifier: AGPL-3.0-only

// taken directly from ssb-keys-neon
//  - next: break this out into neon-helpers crate
//  - just using for now to test assumptions

//use arrayvec::ArrayVec;
use neon::handle::Managed;
//use neon::object::This;
use neon::prelude::*;

pub fn call_builtin<'a, T>(
    cx: &mut impl Context<'a>,
    module: &str,
    name: &str,
    args: impl IntoIterator<Item = Handle<'a, JsValue>>,
) -> JsResult<'a, T>
where
    T: Value + Managed,
{
    let func = cx
        .global()
        .get(cx, module)?
        .downcast::<JsObject>()
        .or_throw(cx)?
        .get(cx, name)?
        .downcast::<JsFunction>()
        .or_throw(cx)?;

    let null = cx.null();
    func.call(cx, null, args)?.downcast::<T>().or_throw(cx)
}

pub fn json_stringify<'a>(
    cx: &mut impl Context<'a>,
    args: impl IntoIterator<Item = Handle<'a, JsValue>>,
) -> JsResult<'a, JsString> {
    call_builtin(cx, "JSON", "stringify", args)
}
