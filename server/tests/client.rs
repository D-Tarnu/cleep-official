use std::fmt::format;

use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    PkceCodeChallenge,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl, HttpRequest
};
use oauth2::basic::BasicClient;
use reqwest::{Client, Url, StatusCode};
use anyhow::Result;
use serde::{Serialize, Deserialize};


enum GrantType {
   Password,
   PasswordRealm
}

impl Serialize for GrantType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let value = match self {
            GrantType::Password => "password",
            GrantType::PasswordRealm => "http://auth0.com/oauth/grant-type/password-realm"
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for GrantType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let value: String = Deserialize::deserialize(deserializer)?; 
        match value.as_str() {
            "password" => Ok(GrantType::Password),
            "http://auth0.com/oauth/grant-type/password-realm" => Ok(GrantType::Password),
            _ => Err(serde::de::Error::custom("Unknown variant"))
        }
    }
}

#[derive(Serialize, Deserialize)]
struct AuthenticationRequestBody {
    username: String,
    password: String,
    client_id: String,
    realm: String,
    grant_type: GrantType
}

#[derive(Serialize, Deserialize)]
struct AuthenticatonResponse {
    access_token: String,
    id_token: String,
    token_type: String,
    expires_in: u32,
    scope: String
}


#[derive(Serialize, Deserialize)]
struct CreateUserRequestBody {
    email: String,
    password: String,
    client_id: String,
    connection: String
}


#[derive(Serialize, Deserialize)]
struct CreateUserResponseBody {
    id: String,
    email_verified: String,
    email: String,
}



async fn create_user(domain: &str, client_id: &str) -> Result<()>{
    let http_client = reqwest::Client::builder()
        .build()?;
    let signup_endpoint = format!("https://{}/dbconnections/signup", domain);  
    let connection = "Username-Password-Authentication".to_string();
    let request_body = CreateUserRequestBody {
        email: "dtshub@gmail.com".to_string(),
        password: "CHANGME".to_string(),
        client_id: client_id.to_string(),
        connection
    };
    let raw_response = http_client.post(Url::parse(&signup_endpoint).unwrap())
        .json(&request_body)
        .send()
        .await?;


    let signup_response = match raw_response.status() {
        StatusCode::OK => Some(raw_response.json::<CreateUserResponseBody>().await?),
        _ => None
    };

    println!("{}", signup_response.map_or("Failed signup".to_string(), |res| res.email_verified));

    Ok(())
    
} 

#[tokio::test]
async fn oidc_client_test() -> Result<()> {
    let domain = "dev-rsy0vxztvb4ctf03.us.auth0.com".to_string();
    let client_id = "uiJ6gYgxREz0D90eakeZyh76Gvv1mtAR".to_string();
    let client = 
        BasicClient::new(
            ClientId::new(client_id.clone()),
            None,
            AuthUrl::new(format!("https://{}/authorize", domain))?,
            Some(TokenUrl::new(format!("https://{}/oauth/token", domain))?)
        )
        .set_redirect_uri(RedirectUrl::new("http://localhost:3000".to_string())?);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let username = "chance4rebholz@gmail.com".to_string();
    let password = "CHANGME".to_string();
    let realm = "Username-Password-Authentication".to_string();
    // let connection = "Username-Password-Authentication";

    let http_client = reqwest::Client::builder()
        .build()?;

    let auth_body = AuthenticationRequestBody {
        username,
        password,
        client_id: client_id.clone(),
        realm,
        grant_type: GrantType::PasswordRealm
    };
    let raw_response = http_client.post(Url::parse(format!("https://{}/oauth/token", domain).as_str())?)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(serde_urlencoded::to_string(auth_body).unwrap())
        .send().await?;

    let auth_response = match raw_response.status() {
        StatusCode::OK => Some(raw_response.json::<AuthenticatonResponse>().await?),
        _ =>  None
    };

    create_user(&domain, &client_id).await?;

    // println!("{}", response.text().await?);
    // let auth_response: AuthenticatonResponse = response.json().await?; 
    println!("{}", auth_response.map_or("Ahh".to_string(), |r| r.access_token));


    // let (auth_url, csrf_token) = client
    //     .authorize_url(CsrfToken::new_random)
    //     .add_scope(Scope::new("openid".to_string()))
    //     .set_pkce_challenge(pkce_challenge)
    //     .url();

    // webbrowser::open(auth_url.as_str());
    // let http_client = reqwest::Client::builder()
    //     .redirect(Policy::none())
    //     .build()?;
    // println!("URL: {}", &auth_url);
    // let response = http_client.get(auth_url.clone()).send().await?;
    // println!("{}, {:?}", response.status(), response.headers());

    Ok(())

}
