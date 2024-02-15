use crate::github::graphql::get_existing_pull_request::PullRequest;
use crate::github::graphql::get_pull_request_from_branch::PullRequestState;
use crate::types::package_identifier::PackageIdentifier;
use crate::types::package_version::PackageVersion;
use color_eyre::Result;
use crossterm::style::Stylize;
use futures_util::{stream, StreamExt, TryStreamExt};
use inquire::Confirm;
use std::env;
use std::path::Path;
use std::str::FromStr;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub fn prompt_existing_pull_request(
    identifier: &PackageIdentifier,
    version: &PackageVersion,
    pull_request: &PullRequest,
) -> Result<bool> {
    println!(
        "There is already {} pull request for {identifier} {version} that was created on {} at {}",
        match pull_request.state {
            PullRequestState::Merged => "a merged",
            PullRequestState::Open => "an open",
            _ => "a closed",
        },
        pull_request.created_at.date_naive(),
        pull_request.created_at.time()
    );
    println!("{}", pull_request.url.as_str().blue());
    let proceed = if env::var("CI").is_ok_and(|ci| bool::from_str(&ci) == Ok(true)) {
        false
    } else {
        Confirm::new("Would you like to proceed?").prompt()?
    };
    Ok(proceed)
}

pub async fn write_changes_to_dir(changes: &[(String, String)], output: &Path) -> Result<()> {
    fs::create_dir_all(output).await?;
    stream::iter(changes.iter())
        .map(|(path, content)| async move {
            if let Some(file_name) = Path::new(path).file_name() {
                let mut file = File::create(output.join(file_name)).await?;
                file.write_all(content.as_bytes()).await?;
            }
            Ok::<(), color_eyre::eyre::Error>(())
        })
        .buffer_unordered(2)
        .try_collect()
        .await
}