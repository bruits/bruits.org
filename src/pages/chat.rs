use maud::html;
use maudit::route::prelude::*;

const DISCORD_URL: &str = "https://discord.gg/BNdnEqpGjK";

#[route("/chat")]
pub struct Chat;

impl Route for Chat {
    fn render(&self, _ctx: &mut PageContext) -> impl Into<RenderResult> {
        Ok(html! {
          html {
            head {
              meta charset="utf-8";
              meta name="viewport" content="width=device-width, initial-scale=1";
              meta http-equiv="refresh" content=(format!("0; url={}", DISCORD_URL));
              title { "Redirecting to Discord…" }
              link rel="canonical" href=(DISCORD_URL);
              meta name="robots" content="noindex";
            }
            body {
              p { "Redirecting to " a href=(DISCORD_URL) { "the Bruits Discord server" } "." }
            }
          }
        })
    }
}
