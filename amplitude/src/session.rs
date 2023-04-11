pub struct Session {
    platform: SessionPlatform,
    token: String,
    id: String,
    name: String,
    avatar: String,
    signup: u64,
}

enum SessionPlatform {
    Github(GithubSession),
    Google(GoogleSession),
}

struct GoogleSession {
    google_id: String,
    access_token: String,
}

struct GithubSession {
    github_id: String,
    login: String,
    token: String,
}
