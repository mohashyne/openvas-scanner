// SPDX-FileCopyrightText: 2023 Greenbone AG
//
// SPDX-License-Identifier: GPL-2.0-or-later

use std::path::PathBuf;

use nasl_interpreter::FSPluginLoader;
use storage::Dispatcher;

use crate::CliError;

pub fn run<S>(storage: S, path: PathBuf) -> Result<(), CliError>
where
    S: Sync + Send + Dispatcher<String>,
{
    tracing::debug!("description run syntax in {path:?}.");
    // needed to strip the root path so that we can build a relative path
    // e.g. 2006/something.nasl
    let loader = FSPluginLoader::new(path);

    let verifier = feed::HashSumNameLoader::sha256(&loader)?;
    let updater = feed::Update::init("1", 5, loader.clone(), storage, verifier);

    for s in updater {
        let s = s?;
        tracing::trace!("updated {s}");
    }

    Ok(())
}
