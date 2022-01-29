mod client;
mod models;
mod resource;

pub use resource::*;

use crate::model;
use anyhow::Result;
use chrono::prelude::*;
use client::Client;
use log::*;

/// Responsible for asynchronous interaction with the Asana API including
/// transformation of response data into explicitly-defined types.
///
pub struct Asana {
    client: Client,
}

impl Asana {
    /// Returns a new instance for the given access token.
    ///
    pub fn new(access_token: &str) -> Asana {
        info!("Initializing Asana client with personal access token...");
        Asana {
            client: Client::new(access_token, "https://app.asana.com/api/1.0"),
        }
    }

    /// Returns a tuple containing the current user and the workspaces to which
    /// they have access.
    ///
    pub async fn me(&mut self) -> Result<(User, Vec<Workspace>)> {
        info!("Fetching authenticated user details...");

        model!(WorkspaceModel "workspaces" { name: String });
        model!(UserModel "users" {
            email: String,
            name: String,
            workspaces: Vec<WorkspaceModel>,
        } WorkspaceModel);

        let data = self.client.get::<UserModel>("me").await?;
        info!("Received authenticated user details.");

        Ok((
            User {
                gid: data.gid,
                name: data.name,
                email: data.email,
            },
            data.workspaces
                .into_iter()
                .map(|w| Workspace {
                    gid: w.gid,
                    name: w.name,
                })
                .collect(),
        ))
    }

    /// Returns a vector of projects for the workspace.
    ///
    pub async fn projects(&mut self, workspace_gid: &str) -> Result<Vec<Project>> {
        info!("Fetching projects for the workspace...");

        model!(ProjectModel "projects" { name: String });

        let data: Vec<ProjectModel> = self
            .client
            .list::<ProjectModel>(Some(vec![("workspace", workspace_gid)]))
            .await?;
        info!("Received projects for the workspace.");

        Ok(data
            .into_iter()
            .map(|p| Project {
                gid: p.gid,
                name: p.name,
            })
            .collect())
    }

    /// Returns a vector of incomplete tasks assigned to the user.
    ///
    pub async fn my_tasks(&mut self, user_gid: &str, workspace_gid: &str) -> Result<Vec<Task>> {
        info!("Fetching tasks assigned to user...");

        model!(TaskModel "tasks" { name: String });

        let data: Vec<TaskModel> = self
            .client
            .list::<TaskModel>(Some(vec![
                ("assignee", user_gid),
                ("workspace", workspace_gid),
                (
                    "completed_since",
                    &Utc::now().format("%Y-%m-%dT%H:%M:%S%.fZ").to_string(),
                ),
            ]))
            .await?;
        info!("Received incomplete tasks assigned to user.");

        Ok(data
            .into_iter()
            .map(|t| Task {
                gid: t.gid,
                name: t.name,
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::uuid::UUIDv4;
    use fake::{Fake, Faker};
    use httpmock::MockServer;
    use serde_json::json;
    use uuid::Uuid;

    #[tokio::test]
    async fn me_success() -> Result<()> {
        let token: Uuid = UUIDv4.fake();
        let user: User = Faker.fake();
        let workspaces: [Workspace; 2] = [Faker.fake(), Faker.fake()];

        let server = MockServer::start();
        let mock = server.mock_async(|when, then| {
            when.method("GET")
                .path("/users/me")
                .header("Authorization", &format!("Bearer {}", &token));
            then.status(200).json_body(json!({
                "data": {
                    "gid": user.gid,
                    "name": user.name,
                    "email": user.email,
                    "resource_type": "user",
                    "workspaces": [
                        { "gid": workspaces[0].gid, "resource_type": "workspace", "name": workspaces[0].name },
                        { "gid": workspaces[1].gid, "resource_type": "workspace", "name": workspaces[1].name },
                    ]
                }
            }));
        }).await;

        let mut asana = Asana {
            client: Client::new(&token.to_string(), &server.base_url()),
        };
        asana.me().await?;
        mock.assert_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn me_unauthorized() {
        let server = MockServer::start();
        let mock = server
            .mock_async(|when, then| {
                when.method("GET").path("/users/me");
                then.status(401);
            })
            .await;

        let mut asana = Asana {
            client: Client::new("", &server.base_url()),
        };
        assert!(asana.me().await.is_err());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn projects_success() -> Result<()> {
        let token: Uuid = UUIDv4.fake();
        let workspace: Workspace = Faker.fake();
        let projects: [Project; 2] = Faker.fake();

        let server = MockServer::start();
        let mock = server
            .mock_async(|when, then| {
                when.method("GET")
                    .path("/projects/")
                    .header("Authorization", &format!("Bearer {}", &token))
                    .query_param("workspace", &workspace.gid);
                then.status(200).json_body(json!({
                    "data": [
                        {
                            "gid": projects[0].gid,
                            "resource_type": "task",
                            "name": projects[0].name,
                        },
                        {
                            "gid": projects[1].gid,
                            "resource_type": "task",
                            "name": projects[1].name,
                        }
                    ]
                }));
            })
            .await;

        let mut asana = Asana {
            client: Client::new(&token.to_string(), &server.base_url()),
        };
        asana.projects(&workspace.gid).await?;
        mock.assert_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn my_tasks_success() -> Result<()> {
        let token: Uuid = UUIDv4.fake();
        let user: User = Faker.fake();
        let workspace: Workspace = Faker.fake();
        let task: [Task; 2] = Faker.fake();

        let server = MockServer::start();
        let mock = server
            .mock_async(|when, then| {
                when.method("GET")
                    .path("/tasks/")
                    .header("Authorization", &format!("Bearer {}", &token))
                    .query_param("assignee", &user.gid)
                    .query_param("workspace", &workspace.gid)
                    .query_param_exists("completed_since");
                then.status(200).json_body(json!({
                    "data": [
                        {
                            "gid": task[0].gid,
                            "resource_type": "task",
                            "name": task[0].name,
                        },
                        {
                            "gid": task[1].gid,
                            "resource_type": "task",
                            "name": task[1].name,
                        }
                    ]
                }));
            })
            .await;

        let mut asana = Asana {
            client: Client::new(&token.to_string(), &server.base_url()),
        };
        asana.my_tasks(&user.gid, &workspace.gid).await?;
        mock.assert_async().await;
        Ok(())
    }
}
