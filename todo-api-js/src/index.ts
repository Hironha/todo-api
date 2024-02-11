import { Elysia } from "elysia";

import * as Todo from "@framework/http/todo";

function main() {
    const port = 8000;

    new Elysia({ strictPath: true })
        .onResponse((res) => {
            const now = new Date().toISOString();
            console.debug(`[DEBUG] ${res.path} at ${now}`);
        })
        .group("/todos", Todo.createRouter)
        .listen(port, () => {
            console.debug(`Listening on port ${port}`);
        });
}

main();
