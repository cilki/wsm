use crate::cmd::RepoPattern;
use crate::Config;
use cmd_lib::*;
use log::debug;
use std::error::Error;

/// Open one or more repositories in the workspace
pub fn run_open(config: &Config, path: Option<String>) -> Result<(), Box<dyn Error>> {
    debug!("Open requested for: {:?}", &path);

    // Check the cache first
    if let Some(cache) = &config.cache {
        
    }

    let repos = match path {
        Some(p) => config.resolve_local(&RepoPattern::parse(&p)?), // TODO remote
        None => Vec::new(),
    };

    for repo in repos {
        let out = run_fun!(git -C $repo status --porcelain)?;
        println!("{}", out);
    }
    Ok(())
}
