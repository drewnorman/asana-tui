use asana_sdk::models::Model;

pub struct User {
    pub gid: String,
    pub name: String,
    pub email: String,
}

pub struct Workspace {
    pub gid: String,
    pub name: String,
}

/// Responsible for asynchronous interaction with the Asana API including
/// transformation of response data into explicitly-defined types.
///
pub struct Asana {
    client: asana_sdk::Client,
}

impl Asana {
    /// Returns a new instance for the given access token.
    ///
    pub fn new(access_token: String) -> Asana {
        Asana {
            client: asana_sdk::Asana::connect(access_token),
        }
    }

    /// Returns a tuple containing the current user and the workspaces to which
    /// they have access.
    ///
    pub async fn me(&mut self) -> (User, Vec<Workspace>) {
        // Define data to request
        asana_sdk::model!(WorkspaceModel "workspaces" { name: String });
        asana_sdk::model!(UserModel "users" {
            email: String,
            name: String,
            workspaces: Vec<WorkspaceModel>,
        } WorkspaceModel);

        // Make request
        let user_data = self.client.get::<UserModel>("me").await;

        // Return new user and vector of workspaces
        (
            User {
                gid: user_data.gid,
                name: user_data.name,
                email: user_data.email,
            },
            user_data
                .workspaces
                .iter()
                .map(|w| Workspace {
                    gid: w.gid.to_owned(),
                    name: w.name.to_owned(),
                })
                .collect(),
        )
    }
}
