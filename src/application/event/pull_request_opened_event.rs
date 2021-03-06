use eventbus::Event;
use crate::application::command::pull_request_open_command::PullRequestOpenCommand;
use crate::domain::{
    installation::Installation, pull_request::PullRequest, user::User, repo::Repo
};

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct PullRequestOpenedEvent {
    pub action: String,
    pub number: u64,
    pub pull_request: PullRequest,
    pub repository: Repo,
    pub sender: User,
    pub installation: Installation,
}

impl Event for PullRequestOpenedEvent {}

impl PullRequestOpenedEvent {
    pub fn new(command: PullRequestOpenCommand) -> Self {
        Self {
            action: command.action,
            number: command.number,
            pull_request: command.pull_request,
            repository: command.repository,
            sender: command.sender,
            installation: command.installation,
        }
    }
}