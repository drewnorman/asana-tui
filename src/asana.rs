use asana_sdk::models::Model;

pub struct Asana {
    client: asana_sdk::Client,
}

asana_sdk::model!(UserModel "users" {
    email: String,
    name: String,
});

impl Asana {
    pub fn new(access_token: String) -> Asana {
        Asana {
            client: asana_sdk::Asana::connect(access_token),
        }
    }

    pub async fn me(&mut self) -> UserModel {
        self.client.get::<UserModel>("me").await
    }
}
