use graphql_ffi_example_server::{schema as create_schema, Context as ServerContext, Schema};
use juniper::http::GraphQLBatchRequest;

use neon::types::buffer::TypedArray;
use neon::{
    context::{Context, FunctionContext, ModuleContext},
    result::{JsResult, NeonResult},
    types::{Finalize, JsBuffer, JsBox},
};

struct Server {
    context: ServerContext,
    schema: Schema,
}

impl Finalize for Server {}

impl Server {
    fn this<'a>(cx: &mut FunctionContext<'a>) -> JsResult<'a, JsBox<Self>> {
        cx.this().downcast_or_throw(cx)
    }
}

// Exported
impl Server {
    fn new(mut cx: FunctionContext) -> JsResult<JsBox<Self>> {
        Ok(cx.boxed(Server {
            context: ServerContext::default(),
            schema: create_schema(),
        }))
    }

    fn execute(mut cx: FunctionContext) -> JsResult<JsBuffer> {
        let this = Self::this(&mut cx)?;
        let req = cx.argument::<JsBuffer>(0)?;
        let req = serde_json::from_slice::<GraphQLBatchRequest>(req.as_slice(&mut cx))
            .or_else(|err| cx.throw_error(err.to_string()))?;

        let res = req.execute_sync(&this.schema, &this.context);
        let res = serde_json::to_vec(&res).or_else(|err| cx.throw_error(err.to_string()))?;

        Ok(JsBuffer::external(&mut cx, res))
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("serverNew", Server::new)?;
    cx.export_function("serverExecute", Server::execute)?;

    Ok(())
}
