import fs from "fs";
import toml from "toml";
import { Octokit } from "octokit";
import { createAppAuth } from "@octokit/auth-app";

export class AuthConfig {
    github_oauth: {
        app_id: string;
        app_secret: string;
    };
    google_oauth: {
        client_id: string;
        client_secret: string;
        external_url: string;
    };
    bot: {
        app_id: number;
        app_secret: string;
        private_key: string;
        installation_id: number;
    };
}

let buffer = fs.readFileSync("../auth.toml");
export let auth: AuthConfig = toml.parse(buffer.toString());

export const octokit = new Octokit({
    authStrategy: createAppAuth,
    auth: {
        appId: auth.bot.app_id,
        privateKey: auth.bot.private_key,
        installationId: auth.bot.installation_id,
        clientSecret: auth.bot.app_secret,
    },
});
