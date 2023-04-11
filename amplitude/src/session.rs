pub struct Session {
    /// Platform specific things
    pub platform: SessionPlatform,
    /// Session token
    pub token: String,
    /// Amplify user id
    pub id: String,
    /// User's name
    pub name: String,
    /// URL to their avatar
    pub avatar: String,
    /// The time they signed up (epoch secs)
    pub signup: u64,
}

pub enum SessionPlatform {
    Github(GithubSession),
    Google(GoogleSession),
}

pub struct GoogleSession {
    pub google_id: String,
    pub access_token: String,
}

pub struct GithubSession {
    pub github_id: String,
    pub login: String,
    pub token: String,
}
