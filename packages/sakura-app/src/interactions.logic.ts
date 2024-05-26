import type {
	Interaction,
	InteractionResponse,
	PongInteractionResponse,
} from "./discord/interactions";
import { InteractionType } from "./discord/interactions";

export const processInteraction = async (
	interaction: Interaction,
): Promise<InteractionResponse> => {
	if (interaction.type === InteractionType.Ping) {
		return {
			type: 1,
		} satisfies PongInteractionResponse;
	}
	throw new Error("invalid interaction type");
};
