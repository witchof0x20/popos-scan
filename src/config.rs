use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub codename: String,
    pub arch: String,
    pub keyserver: String,
    pub key: String,
    repos: Vec<Repo>,
}

#[derive(Debug, Deserialize)]
pub struct Repo {
    url: String,
    #[serde(default)]
    codename_component: Option<String>,
    components: Vec<String>,
}

impl Config {
    pub fn to_repos(self) -> HashMap<String, Vec<Component>> {
        let mut output = HashMap::new();
        for (url, components) in self.repos.into_iter().map(|repo| {
            let dist_folder = if let Some(codename_component) = repo.codename_component {
                format!("{}-{}", self.codename, codename_component)
            } else {
                self.codename.clone()
            };
            (
                repo.url,
                Component {
                    dist_folder,
                    components: repo.components,
                },
            )
        }) {
            output.entry(url).or_insert_with(Vec::new).push(components);
        }
        output
    }
}
pub struct Component {
    pub dist_folder: String,
    pub components: Vec<String>,
}
