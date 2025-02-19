use crate::models::projects::Project;
use serde::Serialize;
use time::OffsetDateTime;

#[derive(Serialize)]
struct DiscordEmbed {
    pub title: String,
    pub description: String,
    pub url: String,
    #[serde(with = "crate::util::time_ser")]
    pub timestamp: OffsetDateTime,
    pub color: u32,
    pub fields: Vec<DiscordEmbedField>,
    pub image: DiscordEmbedImage,
}

#[derive(Serialize)]
struct DiscordEmbedField {
    pub name: &'static str,
    pub value: String,
    pub inline: bool,
}

#[derive(Serialize)]
struct DiscordEmbedImage {
    pub url: Option<String>,
}

#[derive(Serialize)]
struct DiscordWebhook {
    pub embeds: Vec<DiscordEmbed>,
}

pub async fn send_discord_webhook(
    project: Project,
    webhook_url: String,
) -> Result<(), reqwest::Error> {
    let mut fields = vec![
        DiscordEmbedField {
            name: "id",
            value: project.id.to_string(),
            inline: true,
        },
        DiscordEmbedField {
            name: "project_type",
            value: project.project_type.clone(),
            inline: true,
        },
        DiscordEmbedField {
            name: "client_side",
            value: project.client_side.to_string(),
            inline: true,
        },
        DiscordEmbedField {
            name: "server_side",
            value: project.server_side.to_string(),
            inline: true,
        },
    ];

    if !project.categories.is_empty() {
        fields.push(DiscordEmbedField {
            name: "categories",
            value: project.categories.join(", "),
            inline: true,
        });
    }

    if let Some(ref slug) = project.slug {
        fields.push(DiscordEmbedField {
            name: "slug",
            value: slug.clone(),
            inline: true,
        });
    }

    let embed = DiscordEmbed {
        url: format!(
            "{}/{}/{}",
            dotenv::var("SITE_URL").unwrap_or_default(),
            project.project_type,
            project
                .clone()
                .slug
                .unwrap_or_else(|| project.id.to_string())
        ),
        title: project.title,
        description: project.description,
        timestamp: project.published,
        color: 0x1bd96a,
        fields,
        image: DiscordEmbedImage {
            url: project.icon_url,
        },
    };

    let client = reqwest::Client::new();

    client
        .post(&webhook_url)
        .json(&DiscordWebhook {
            embeds: vec![embed],
        })
        .send()
        .await?;

    Ok(())
}
