import { Hono } from "hono";
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
		const interaction = (await c.req.json()) as Interaction;
		return c.json(await processInteraction(interaction));
	},
);

export default app;
