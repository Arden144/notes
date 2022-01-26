import { loadEnvironment, snowflakeToBigint } from "./deps.ts";

export interface Config {
	port: number;
	token: string;
	api: string;
	id: bigint;
	channel: bigint;
}

const env = loadEnvironment();

export const config = <Config> {
	port: parseInt(env.PORT, 10),
	token: env.TOKEN,
	api: env.API,
	id: snowflakeToBigint(env.ID),
	channel: snowflakeToBigint(env.CHANNEL),
};
