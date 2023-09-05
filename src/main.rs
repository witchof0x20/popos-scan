mod config;

use crate::config::Config;
use apt_parser::{Packages, Release};
use std::fs;

#[tokio::main]
async fn main() {
    // Load config from file
    let config = fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(&config).unwrap();
    let codename = config.codename.clone();
    let arch = config.arch.clone();
    let repos = config.to_repos();
    let client = reqwest::Client::new();

    //TODO: talk to given key server and get the gpg keys used to verify

    let mut nvidia_drivers = Vec::new();
    let mut kernel_versions = Vec::new();

    for (url, component_lists) in repos.into_iter() {
        // First grab the package list
        let release_url = format!("{}/dists/{codename}/Release", url);
        let release = client
            .get(release_url)
            .send()
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
        // Then grab the gpg signature
        let release_gpg_url = format!("{}/dists/{codename}/Release.gpg", url);
        let release_gpg = client
            .get(release_gpg_url)
            .send()
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
        // TODO: gpg verification
        // Parse the package list to string
        let release1 = String::from_utf8(release.to_vec()).unwrap();
        // Parse the package list as apt package list
        let release = Release::from(&release1).unwrap();
        // Grab each of the component package lists
        for component in component_lists {
            for subcomponent in component.components {
                // Extract the strongest hash (sha256)
                let mut sha256 = None;
                let target_filename = format!("{subcomponent}/binary-{arch}/Packages");
                if let Some(ref release_hashes) = release.sha256sum {
                    for release_hash in release_hashes {
                        if release_hash.filename == target_filename {
                            sha256 = Some(release_hash.hash.clone());
                            break;
                        }
                    }
                }
                let sha256 = sha256.unwrap();
                let packages_url = format!(
                    "{url}/dists/{}/{}/binary-{arch}/Packages",
                    component.dist_folder, subcomponent
                );
                let packages = client
                    .get(packages_url)
                    .send()
                    .await
                    .unwrap()
                    .bytes()
                    .await
                    .unwrap();
                //TODO: hash it
                let packages = String::from_utf8(packages.to_vec()).unwrap();
                for package in Packages::from(&packages) {
                    if package.package.starts_with("nvidia-driver") {
                        nvidia_drivers.push((package.package.clone(), package.version.clone()));
                    }
                    if package.package.starts_with("linux-system76") {
                        kernel_versions.push((package.package, package.version));
                    }
                }
            }
        }
    }
    kernel_versions.sort_by_key(|(_, ver)| ver.clone());
    println!("Kernels:");
    for (name, version) in kernel_versions {
        println!("{name}: {version}");
    }

    nvidia_drivers.sort_by_key(|(_, ver)| ver.clone());
    println!("NVIDIA:");
    for (name, version) in nvidia_drivers {
        println!("{name}: {version}");
    }
}
