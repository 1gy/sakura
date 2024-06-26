import { Hono } from "hono";
import { HTTPException } from "hono/http-exception";
import type { Interaction } from "./discord/interactions";
import type { AppBindings } from "./env";
import { processInteraction } from "./interactions.logic";
import { verifyDiscord } from "./verify";

const app = new Hono();

app.post(
	"/interactions",
	verifyDiscord<AppBindings>({
		publicKey: (b) => b.DISCORD_APP_PUBLIC_KEY,
	}),
	async (c) => {
		try {
			const waitUntil = (promise: Promise<unknown>) => {
				c.executionCtx.waitUntil(promise);
			};
			const interaction = (await c.req.json()) as Interaction;
			return c.json(await processInteraction(interaction, waitUntil, c.env));
		} catch (e) {
			console.error(e);
			throw new HTTPException(500, {
				message: "Internal Server Error",
				cause: e,
			});
		}
	},
);

export default app;
