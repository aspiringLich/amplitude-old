import fs from "fs";
import toml from "toml";

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
    bot_auth: {
        app_id: string;
        app_secret: string;
    };
}

let buffer = fs.readFileSync("../auth.toml");
export let auth: AuthConfig = toml.parse(buffer.toString());
