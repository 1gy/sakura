import {
	InteractionType,
	type ApplicationCommandInteraction,
	type DeferredChannelMessageWithSourceInteractionResponse,
	type Interaction,
	type InteractionResponse,
	type PongInteractionResponse,
} from "./discord/interactions";
import type { AppBindings } from "./env";
import { streamingChat } from "./openai/chat";

export const talk = async (
	openaiApiKey: string,
	message: string,
	callback: (content: string) => Promise<unknown>,
): Promise<void> => {
	const quoted = message
		.split("\n")
		.map((line) => `> ${line}`)
		.join("\n");

	await callback(`${quoted}\n...`);

	const intervalId = setInterval(async () => {
		await callback(`${quoted}\n\n${current}(...)`);
	}, 500);

	let current = "";
	for await (const data of streamingChat(openaiApiKey, [
		{
			role: "system",
			content: "NEW GAME!に登場する桜ねねの口調で話してください",
		},
		{
			role: "user",
			content: [
				{
					type: "text",
					text: message,
				},
			],
		},
	])) {
		current += data.choices[0]?.delta.content ?? "";
	}
	clearInterval(intervalId);
	await callback(`${quoted}\n\n${current}`);
};

export const processInteraction = async (
	interaction: Interaction,
	waitUntil: (promise: Promise<unknown>) => void,
	env: AppBindings,
): Promise<InteractionResponse> => {
	if (interaction.type === InteractionType.Ping) {
		return {
			type: 1,
		} satisfies PongInteractionResponse;
	}
	if (interaction.type === InteractionType.ApplicationCommand) {
		return await handleApplicationCommand(interaction, waitUntil, env);
	}
	throw new Error("invalid interaction type");
};

export const handleApplicationCommand = async (
	interaction: ApplicationCommandInteraction,
	waitUntil: (promise: Promise<unknown>) => void,
	env: AppBindings,
): Promise<InteractionResponse> => {
	switch (interaction.data.name) {
		case "talk": {
			waitUntil(handleTalk(interaction, env));
			return {
				type: 5,
			} as DeferredChannelMessageWithSourceInteractionResponse;
		}
	}
	throw new Error(`invalid command: ${interaction.data.name}`);
};

const handleTalk = async (
	interaction: ApplicationCommandInteraction,
	env: AppBindings,
): Promise<void> => {
	const message = interaction.data.options?.[0]?.value ?? "";

	const update = patchContent(env.DISCORD_APP_ID, interaction.token);

	await talk(env.OPENAI_API_TOKEN, message, (content) => update(content));
};

const patchContent = (applicationId: string, interactionToken: string) => {
	return async (content: string) => {
		const url = `https://discord.com/api/v10/webhooks/${applicationId}/${interactionToken}/messages/@original`;
		await fetch(url, {
			method: "PATCH",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({
				content,
			}),
		});
	};
};
