use graphgarden_core::{
    build::build,
    config::{Config, OutputConfig, ParseConfig, SiteConfig},
};
use maudit::{content_sources, coronate, routes, BuildOptions, BuildOutput};

mod pages {
    mod chat;
    mod index;
    pub use chat::Chat;
    pub use index::Index;
}

fn main() -> Result<BuildOutput, Box<dyn std::error::Error>> {
    let output = coronate(
        routes![pages::Index, pages::Chat],
        content_sources![],
        BuildOptions::default(),
    )?;

    let gg_config = Config {
        site: SiteConfig {
            base_url: "https://bruits.org/".into(),
            title: "Bruits".into(),
            description: Some("Software and game development collective".into()),
            language: Some("en".into()),
        },
        friends: vec![
            "https://erika.florist".into(),
            "https://goulven-clech.dev".into(),
        ],
        output: OutputConfig {
            dir: "./dist".into(),
        },
        parse: ParseConfig {
            ..Default::default()
        },
    };

    let public_file = build(&gg_config)?;
    let json = public_file.to_json()?;

    let well_known_dir = std::path::Path::new("./dist/.well-known");
    std::fs::create_dir_all(well_known_dir)?;
    std::fs::write(well_known_dir.join("graphgarden.json"), json)?;

    Ok(output)
}
