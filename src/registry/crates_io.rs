use std::{
    collections::{BTreeMap, VecDeque},
    path::PathBuf,
};

use cargo_toml::{Dependency, Manifest};
use serde::{Deserialize, Serialize};

use crate::fetcher::DepsFetcher;

use super::content::DependencyContent;

const REGISTRY: &str = "https://crates.io";

pub async fn read_content(cargo: PathBuf) -> String {
    let manifest = Manifest::from_path(cargo).expect("Could not parse Cargo.toml");
    let mut contents = vec![];

    if !manifest.dependencies.is_empty() {
        contents.push(
            generate_deps_content(&manifest.dependencies, "### Dependencies".to_owned()).await,
        );
    }
    if !manifest.dev_dependencies.is_empty() {
        contents.push(
            generate_deps_content(&manifest.dev_dependencies, "### DevDependencies".to_owned())
                .await,
        );
    }
    if !manifest.build_dependencies.is_empty() {
        contents.push(
            generate_deps_content(
                &manifest.build_dependencies,
                "### BuildDependencies".to_owned(),
            )
            .await,
        );
    }

    contents.push("".to_owned());

    contents.join("\n")
}

#[derive(Serialize, Deserialize)]
struct ExpectCrateJson {
    name: String,
    description: Option<String>,
    repository: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ExpectJson {
    #[serde(rename = "crate")]
    crate_: Option<ExpectCrateJson>,
}

async fn generate_deps_content(deps: &BTreeMap<String, Dependency>, title: String) -> String {
    let keys: Vec<_> = deps.keys().collect();
    let mut names = VecDeque::new();
    for name in &keys {
        names.push_back(name.to_string());
    }

    let label = title.replace(r"# ", "");
    let label = label.replace('#', "");

    println!("===== Start fetching {label} =====");

    let f = DepsFetcher::new(REGISTRY.to_owned());
    let result = f.fetch_all::<ExpectJson>("/api/v1/crates", names).await;

    println!("===== Finished fetching {label} =====");

    let mut contents = vec![title];
    for (i, item) in result.iter().enumerate() {
        let item = item.as_ref().expect("Failed to fetch");
        let content = match &item.crate_ {
            Some(crate_) => DependencyContent {
                name: crate_.name.clone(),
                description: crate_.description.clone(),
                repository: crate_.repository.clone(),
            },
            None => DependencyContent {
                name: keys.get(i).expect("Could not find crate name").to_string(),
                description: None,
                repository: None,
            }
        }
        .into_string();
        let content = content.replace('\n', " ");
        contents.push(format!("- {content}"));
    }

    contents.join("\n")
}
