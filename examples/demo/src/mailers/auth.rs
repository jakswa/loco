// auth mailer
#![allow(non_upper_case_globals)]

use include_dir::{include_dir, Dir};
use serde_json::json;

use framework::{
    app::AppContext,
    mailer::{Args, Mailer},
    Result,
};

static welcome: Dir<'_> = include_dir!("src/mailers/auth/welcome");
// #[derive(Mailer)] // -- disabled for faster build speed. it works. but lets move on for now.
pub struct AuthMailer {}
impl Mailer for AuthMailer {}
impl AuthMailer {
    pub async fn send_welcome(ctx: &AppContext, _user_id: &str) -> Result<()> {
        Self::mail_template(
            ctx,
            &welcome,
            Args {
                to: "foo@example.com".to_string(),
                locals: json!({
                  "name": "joe"
                }),
                ..Default::default()
            },
        )
        .await?;
        Ok(())
    }
}