import { createMiddleware } from "hono/factory";
import type { Bindings } from "hono/types";

const hexToBytes = (hex: string) => {
	return new Uint8Array(
		hex.match(/.{1,2}/g)?.map((byte) => Number.parseInt(byte, 16)) ?? [],
	);
};

const algorithm: SubtleCryptoImportKeyAlgorithm = {
	name: "ED25519",
	namedCurve: "ED25519",
};

const importKey = async (key: string) => {
	return await crypto.subtle.importKey(
		"raw",
		hexToBytes(key),
		algorithm,
		false,
		["verify"],
	);
};

const textEncoder = new TextEncoder();

const verify = async (
	key: CryptoKey,
	signature: string,
	timestamp: string,
	message: Uint8Array,
) => {
	const signatureData = hexToBytes(signature);
	const timestampData = textEncoder.encode(timestamp);

	const data = new Uint8Array(timestampData.length + message.length);
	data.set(timestampData, 0);
	data.set(message, timestampData.byteLength);

	const result = await crypto.subtle.verify(
		algorithm,
		key,
		signatureData,
		data,
	);

	return result;
};

// biome-ignore lint/suspicious/noExportsInTest: issue -> https://github.com/biomejs/biome/issues/2859
export const verifyDiscord = <B extends Bindings>(params: {
	publicKey: (env: B) => string;
}) => {
	return createMiddleware<{ Bindings: B }>(async (c, next) => {
		const key = await importKey(params.publicKey(c.env));
		const signature = c.req.header("x-signature-ed25519");
		const timestamp = c.req.header("x-signature-timestamp");
		const body = await c.req.arrayBuffer();

		const valid =
			signature &&
			timestamp &&
			(await verify(key, signature, timestamp, new Uint8Array(body)));

		if (!valid) {
			return c.text("invalid request signature", 401);
		}

		return await next();
	});
};

if (import.meta.vitest) {
	const { it, expect, describe } = import.meta.vitest;
	const { Hono } = await import("hono");

	describe("hexToBytes", () => {
		it("works", () => {
			expect(hexToBytes("")).toEqual(new Uint8Array([]));
			expect(hexToBytes("01ff")).toEqual(new Uint8Array([1, 255]));
		});
	});

	describe("verifyDiscord", async () => {
		const app = new Hono();
		app.use(
			await verifyDiscord({
				publicKey: () =>
					"cab6f2a127d35f663090a241582b13accee49c0ba46d6b35e6634ee05cc4e5d6",
			}),
		);
		app.post("/ping", (c) => c.text("pong"));

		it("should return 401 for invalid signature", async () => {
			const res = await app.request("/ping", {
				method: "POST",
				headers: {
					"x-signature-ed25519": "invalid",
					"x-signature-timestamp": "invalid",
				},
			});
			expect(res.status).toBe(401);
		});

		it("should return 200 for valid signature", async () => {
			const res = await app.request("/ping", {
				method: "POST",
				headers: {
					"x-signature-ed25519":
						"013dfed0ed000925c29071d5f83fc046ec029f97758c45fe972f525052f1ce342ae7da1f1344f15c223be98bce0e0911af7f779a605f6cb7b8cee19594b70809",
					"x-signature-timestamp": "1716632144",
				},
			});
			expect(res.status).toBe(200);
			expect(await res.text()).toBe("pong");
		});
	});
}
