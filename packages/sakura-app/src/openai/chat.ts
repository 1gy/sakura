export type SystemMessage = {
	role: "system";
	content: string;
};

export type UserMessage = {
	role: "user";
	content: Array<
		| {
				type: "text";
				text: string;
		  }
		| {
				type: "image_url";
				media: {
					url: string;
				};
		  }
	>;
};

export type ChatRequest = {
	messages: Array<SystemMessage | UserMessage>;
	model: string;
	stream?: boolean;
};

export type ChatCompletionResponse = {
	id: string;
	choices: Array<{
		finish_reason: string;
		index: number;
		message: {
			role: string;
			content?: string;
		};
	}>;
};

export type ChatCompletionChunkResponse = {
	id: string;
	choices: Array<{
		delta: {
			role: string;
			content?: string;
		};
		finish_reason?: string;
		index: number;
	}>;
	model: string;
	object: "chat.completion.chunk";
};

const serviceUrl = "https://api.openai.com/";

export async function* streamingChat(
	apiKey: string,
	messages: Array<SystemMessage | UserMessage>,
): AsyncGenerator<ChatCompletionChunkResponse> {
	const url = `${serviceUrl}v1/chat/completions`;
	const response = await fetch(url, {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
			Authorization: `Bearer ${apiKey}`,
		},
		body: JSON.stringify({
			model: "gpt-4o",
			messages,
			stream: true,
		} satisfies ChatRequest),
	});
	if (!response.ok) {
		throw new Error(`Failed to fetch: ${response.statusText}`);
	}
	const reader = response.body?.getReader();
	if (!reader) {
		throw new Error("Failed to get reader");
	}
	const decoder = new TextDecoder();
	let finished = false;
	while (true) {
		let chunk = "";
		while (true) {
			const { done, value } = await reader.read();
			chunk += decoder.decode(value);
			if (done) {
				finished = true;
				break;
			}
			if (chunk.endsWith("\n")) {
				break;
			}
		}
		const lines = chunk
			.split("data: ")
			.map((line) => line.trim())
			.filter(Boolean);
		for (const line of lines) {
			if (line === "[DONE]") {
				break;
			}
			yield JSON.parse(line) as ChatCompletionChunkResponse;
		}
		if (finished) {
			break;
		}
	}
	reader.releaseLock();
}
