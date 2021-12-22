use super::client::Client;
use super::resource::*;
use crate::model;
use anyhow::Result;

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
        Asana {
            client: Client::new(access_token, "https://app.asana.com/api/1.0"),
        }
    }

    /// Returns a tuple containing the current user and the workspaces to which
    /// they have access.
    ///
    pub async fn me(&mut self) -> Result<(User, Vec<Workspace>)> {
        model!(WorkspaceModel "workspaces" { name: String }); 
        model!(UserModel "users" {                            
            email: String,                                               
            name: String,                                                
            workspaces: Vec<WorkspaceModel>,                             
        } WorkspaceModel);

        // Make request
        let data = self.client.get::<UserModel>("me").await?;

        // Return new user and vector of workspaces
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

        let mut asana = Asana { client: Client::new(&token.to_string(), &server.base_url()) };
        asana.me().await?;
        mock.assert_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn me_unauthorized() {
        let server = MockServer::start();
        let mock = server.mock_async(|when, then| {
            when.method("GET")
                .path("/users/me");
            then.status(401);
        }).await;

        let mut asana = Asana { client: Client::new("", &server.base_url()) };
        assert!(asana.me().await.is_err());
        mock.assert_async().await;
    }
}
