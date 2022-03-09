import { Application, createBot, Router, sendMessage, startBot } from "./deps.ts";
import { config } from "./env.ts";

const server = new Application();
const router = new Router();

const bot = createBot({
	token: config.token,
	botId: config.id,
	events: { ready: () => console.log("ready") },
	intents: [],
});

router.post("/notes", async ({ request, response }) => {
	const auth = request.headers.get("Authorization");
	if (!auth) {
		response.status = 401;
		return;
	}

	const token = auth.split(" ")[1];
	if (token !== config.api) {
		response.status = 401;
		return;
	}

	const body = request.body();
	if (body.type !== "json") {
		response.status = 400;
		return;
	}

	const json = await body.value;
	await sendMessage(bot, config.channel, { content: json.message });
	response.status = 200;
});

server.use(router.routes());
server.use(router.allowedMethods());

await Promise.any([
	startBot(bot),
	server.listen({
		port: config.port,
		secure: true,
		certFile: "./***REMOVED***.pem",
		keyFile: "./***REMOVED***.key",
	}),
]);
