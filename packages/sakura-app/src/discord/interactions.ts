// https://discord.com/developers/docs/interactions/receiving-and-responding

type Snowflake = string;

export enum InteractionType {
	Ping = 1,
	ApplicationCommand = 2,
	MessageComponent = 3,
	ApplicationCommandAutocomplete = 4,
	ModalSubmit = 5,
}

export type PingInteraction = {
	type: InteractionType.Ping;
	id: Snowflake;
	application_id: Snowflake;
	token: string;
	version: number;
};

export type ApplicationCommandInteraction = {
	type: InteractionType.ApplicationCommand;
	id: Snowflake;
	application_id: Snowflake;
	token: string;
	version: number;
	// TODO
};

export type MessageComponentInteraction = {
	type: InteractionType.MessageComponent;
	id: Snowflake;
	application_id: Snowflake;
	token: string;
	version: number;
	// TODO
};

export type ApplicationCommandAutocompleteInteraction = {
	type: InteractionType.ApplicationCommandAutocomplete;
	id: Snowflake;
	application_id: Snowflake;
	token: string;
	version: number;
	// TODO
};

export type ModalSubmitInteraction = {
	type: InteractionType.ModalSubmit;
	id: Snowflake;
	application_id: Snowflake;
	token: string;
	version: number;
	// TODO
};

export type Interaction =
	| PingInteraction
	| ApplicationCommandInteraction
	| MessageComponentInteraction
	| ApplicationCommandAutocompleteInteraction
	| ModalSubmitInteraction;

export enum InteractionResponseType {
	Pong = 1,
	ChannelMessageWithSource = 4,
	DeferredChannelMessageWithSource = 5,
	DeferredUpdateMessage = 6,
	UpdateMessage = 7,
	ApplicationCommandAutocompleteResult = 8,
	Modal = 9,
	PremiumRequired = 10,
}

export type PongInteractionResponse = {
	type: InteractionResponseType.Pong;
};

export type ChannelMessageWithSourceInteractionResponse = {
	type: InteractionResponseType.ChannelMessageWithSource;
	// TODO
};

export type DeferredChannelMessageWithSourceInteractionResponse = {
	type: InteractionResponseType.DeferredChannelMessageWithSource;
	// TODO
};

export type DeferredUpdateMessageInteractionResponse = {
	type: InteractionResponseType.DeferredUpdateMessage;
	// TODO
};

export type UpdateMessageInteractionResponse = {
	type: InteractionResponseType.UpdateMessage;
	// TODO
};

export type ApplicationCommandAutocompleteResultInteractionResponse = {
	type: InteractionResponseType.ApplicationCommandAutocompleteResult;
	// TODO
};

export type ModalInteractionResponse = {
	type: InteractionResponseType.Modal;
	// TODO
};

export type PremiumRequiredInteractionResponse = {
	type: InteractionResponseType.PremiumRequired;
	// TODO
};

export type InteractionResponse =
	| PongInteractionResponse
	| ChannelMessageWithSourceInteractionResponse
	| DeferredChannelMessageWithSourceInteractionResponse
	| DeferredUpdateMessageInteractionResponse
	| UpdateMessageInteractionResponse
	| ApplicationCommandAutocompleteResultInteractionResponse
	| ModalInteractionResponse
	| PremiumRequiredInteractionResponse;
