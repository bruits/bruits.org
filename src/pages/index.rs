use maud::{html, PreEscaped};
use maudit::route::prelude::*;

#[route("/")]
pub struct Index;

impl Route for Index {
    fn render(&self, ctx: &mut PageContext) -> impl Into<RenderResult> {
        ctx.assets.include_style("src/prin.css");
        let logo = ctx.assets.add_image("src/logo.svg");

        let erika = ctx.assets.add_image("src/avatars/erika.webp");
        let goulven = ctx.assets.add_image("src/avatars/goulven.webp");

        let projects = [
        	(include_str!("../maudit.svg"), "Maudit", "Library to generate static websites", Some("https://maudit.org")),
         	(include_str!("../sampo.svg"), "Sampo", "Automate changelogs, versioning, and publishing—even for monorepos across multiple package registries", Some("https://github.com/bruits/sampo")),
          (include_str!("../game.svg"), "Unannounced Video Game Project", "A new video game by the creator of SinaRun", None)
        ];

        html! {
          html {
            head {
              meta charset="utf-8";
              meta name="viewport" content="width=device-width, initial-scale=1";
              title { "Bruits" }
              meta name="description" content="Software and game development collective";
              link rel="icon" type="image/svg+xml" href=("/favicon.svg");

              // Open graph
              meta property="og:title" content="Bruits";
              meta property="og:description" content="Software and game development collective";
              meta property="og:type" content="website";
              meta property="og:image" content=("https://bruits.org/social.png");
              meta property="og:site_name" content="bruits.org";

              meta name="twitter:card" content="summary";
            }
            body {
              div class="container" {
                header {
                  img src=(logo.url()) width="800" height="191" alt="Bruits logo";
                }
                main {
                  div.blurb {
                    p {
                        "Bruits is a software and game development collective focused on creating high-quality projects in Rust."
                    }
                    p {
                    "From "  a href="https://erika.florist/" { (erika.render("Erika's avatar, a bouquet of flower in an 90s anime visual style.")) span {"Erika"} } " and " a href="https://goulven-clech.dev/" {  (goulven.render("Goulven's avatar, a picture of himself")) span {"Goulven"} }
                    }
                  }
                  h2 { "Projects" }
                  ul {
                    @for (logo, name, description, link) in projects.iter() {
                      @if let Some(url) = link {
                        a href=(url) {
                          li {
                            (PreEscaped(logo))
                            section {
                                h3 { (name) }
                                p { (PreEscaped(description)) }
                            }
                          }
                        }
                      } @else {
                        li {
                          (PreEscaped(logo))
                          section {
                              h3 { (name) }
                              p { (PreEscaped(description)) }
                          }
                        }
                      }
                    }
                  }
                }
                footer {
                    p { "Copyright © 2025 Bruits." }
                }
              }
            }
          }
        }
    }
}
