import {
	InteractionType,
	type ApplicationCommandInteraction,
	type ChannelMessageWithSourceInteractionResponse,
	type Interaction,
	type InteractionResponse,
	type PongInteractionResponse,
} from "./discord/interactions";

export const processInteraction = async (
	interaction: Interaction,
): Promise<InteractionResponse> => {
	if (interaction.type === InteractionType.Ping) {
		return {
			type: 1,
		} satisfies PongInteractionResponse;
	}
	if (interaction.type === InteractionType.ApplicationCommand) {
		return await handleApplicationCommand(interaction);
	}
	throw new Error("invalid interaction type");
};

export const handleApplicationCommand = async (
	interaction: ApplicationCommandInteraction,
): Promise<InteractionResponse> => {
	switch (interaction.data.name) {
		case "talk": {
			const message = interaction.data.options?.[0]?.value ?? "";
			const quoted = message
				.split("\n")
				.map((line) => `> ${line}`)
				.join("\n");
			return {
				type: 4,
				data: {
					content: `${quoted}\nこんにちは！`,
				},
			} satisfies ChannelMessageWithSourceInteractionResponse;
		}
	}
	throw new Error(`invalid command: ${interaction.data.name}`);
};
