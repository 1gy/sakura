import { Hono } from "hono";
import { logger } from "hono/logger";
import interactions from "./interactions";

const app = new Hono();
app.use(logger());
app.get("/ping", (c) => c.json({ message: "pong" }));
app.route("/", interactions);

export default app;
