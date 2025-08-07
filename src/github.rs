use dialoguer::FuzzySelect;
use octocrab::models::repos::Asset;

use crate::{AppImage, Error, Result};

pub async fn get_github_release_url(appimage: &AppImage) -> Result<String> {
    let octocrab = octocrab::instance();

    let (owner, repo) = appimage
        .source
        .meta
        .url
        .split_once('/')
        .ok_or_else(|| Error::InvalidSlug(appimage.source.meta.url.to_string()))?;

    let page = octocrab
        .repos(owner, repo)
        .releases()
        .list()
        .per_page(100)
        .send()
        .await?;

    let mut tags: Vec<String> = vec![];

    for releases in &page {
        for asset in &releases.assets {
            if asset.name.to_lowercase().ends_with(".appimage") {
                tags.push(releases.tag_name.to_string());
                break;
            }
        }
    }

    let tag_selection = FuzzySelect::new()
        .with_prompt("Choose a release")
        .items(&tags)
        .max_length(7)
        .vim_mode(true)
        .interact()?;

    let mut assets: Vec<Asset> = vec![];

    for releases in page {
        if releases.tag_name == tags[tag_selection] {
            for asset in releases.assets {
                if asset.name.to_lowercase().ends_with(".appimage") {
                    assets.push(asset);
                }
            }
        }
    }

    let mut asset_selection: usize = 0;

    if assets.len() > 1 {
        asset_selection = FuzzySelect::new()
            .with_prompt("Choose an asset")
            .items(
                &assets
                    .iter()
                    .map(|x| x.name.to_string())
                    .collect::<Vec<_>>(),
            )
            .max_length(7)
            .vim_mode(true)
            .interact()?;
    }

    let url = assets[asset_selection].browser_download_url.to_string();

    Ok(url)
}
