import { snowflakeToBigint } from "./deps.ts";

export interface Config {
	port: number;
	token: string;
	api: string;
	id: bigint;
	channel: bigint;
}

export const config = <Config> {
	port: parseInt(Deno.env.get("PORT")!, 10),
	token: Deno.env.get("TOKEN")!,
	api: Deno.env.get("API")!,
	id: snowflakeToBigint(Deno.env.get("ID")!),
	channel: snowflakeToBigint(Deno.env.get("CHANNEL")!),
};
