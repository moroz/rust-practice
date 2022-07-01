use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub struct User {
    id: i32,
    first_name: String,
    last_name: String,
}

impl User {
    fn new(id: i32, first_name: &str, last_name: &str) -> Self {
        User {
            id: id,
            first_name: String::from(first_name),
            last_name: String::from(last_name),
        }
    }

    fn build_claims(&self) -> impl Serialize {
        let mut claims = BTreeMap::new();
        claims.insert("sub", self.id);
        claims
    }

    fn display_name(&self) -> String {
        format!("{} {}", &self.first_name, &self.last_name)
    }
}

fn build_key() -> Hmac<Sha256> {
    Hmac::new_from_slice(b"test").unwrap()
}

fn issue_token(claims: &impl Serialize) -> Result<String, jwt::Error> {
    claims.sign_with_key(&build_key())
}

fn main() {
    let user = User::new(42, "Karol", "Moroz");
    let token = issue_token(&user.build_claims()).unwrap();
    println!("My token: {}", token);
    println!("My name: {}", &user.display_name());

    let json = "{\"id\":69,\"first_name\":\"Zbigniew\",\"last_name\":\"Wodecki\"}";
    let other_user: User = serde_json::from_str(json).unwrap();
    println!("Other user's full name: {}", &other_user.display_name());
    let other_token = issue_token(&other_user.build_claims()).unwrap();
    println!("Other user's token: {}", other_token);
}
