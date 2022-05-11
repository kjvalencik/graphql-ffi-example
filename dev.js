const Koa = require("koa");
const Router = require("koa-router");
const { default: playground } = require("graphql-playground-middleware-koa");

const { serverNew, serverExecute } = require("./bindings");

class Server {
    server = serverNew();

    execute(req) {
        return serverExecute.call(this.server, req);
    }
}

const app = new Koa();
const router = new Router();
const server = new Server();

router.all("/playground", playground({ endpoint: "/graphql" }));

router.post("/graphql", async (ctx, next) => {
    const bufs = [];

    for await (const buf of ctx.req) {
        bufs.push(buf);
    }

    ctx.response.body = server.execute(Buffer.concat(bufs));

    await next();
});

app
    .use(router.routes())
    .use(router.allowedMethods())
    .listen(4000);
