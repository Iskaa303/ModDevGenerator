use Loader::*;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Loader {
    Forge,
    NeoForge,
    Fabric,
    Quilt,
    Cleanroom,
}

pub const LOADERS: &[Loader] = &[Forge, NeoForge, Fabric, Quilt, Cleanroom];

impl fmt::Display for Loader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Forge => "Forge",
            NeoForge => "NeoForge",
            Fabric => "Fabric",
            Quilt => "Quilt",
            Cleanroom => "Cleanroom",
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Version {
    pub id: &'static str,
    pub loaders: &'static [Loader],
}

/// Every Java Edition release from 1.7.10 up that at least one loader.
pub const VERSIONS: &[Version] = &[
    Version { id: "26.3", loaders: &[Forge, NeoForge, Fabric, Quilt] },
    Version { id: "26.2", loaders: &[Forge, NeoForge, Fabric, Quilt] },
    Version { id: "26.1.2", loaders: &[Forge, NeoForge, Fabric, Quilt] },
    Version { id: "26.1.1", loaders: &[Forge, NeoForge, Fabric, Quilt] },
    Version { id: "26.1", loaders: &[Forge, NeoForge, Fabric, Quilt] },
    Version { id: "1.21.11", loaders: &[Forge, NeoForge, Fabric, Quilt] },
    Version { id: "1.21.10", loaders: &[Forge, NeoForge, Fabric, Quilt] },
    Version { id: "1.21.9", loaders: &[Forge, NeoForge, Fabric, Quilt] },
    Version { id: "1.21.8", loaders: &[Forge, NeoForge, Fabric, Quilt] },
    Version { id: "1.21.7", loaders: &[Forge, NeoForge, Fabric, Quilt] },
    Version { id: "1.21.6", loaders: &[Forge, NeoForge, Fabric, Quilt] },
    Version { id: "1.21.5", loaders: &[Forge, NeoForge, Fabric, Quilt] },
    Version { id: "1.21.4", loaders: &[Forge, NeoForge, Fabric, Quilt] },
    Version { id: "1.21.3", loaders: &[Forge, NeoForge, Fabric, Quilt] },
    Version { id: "1.21.2", loaders: &[NeoForge, Fabric, Quilt] },
    Version { id: "1.21.1", loaders: &[Forge, NeoForge, Fabric, Quilt] },
    Version { id: "1.21", loaders: &[Forge, NeoForge, Fabric, Quilt] },
    Version { id: "1.20.6", loaders: &[Forge, NeoForge, Fabric, Quilt] },
    Version { id: "1.20.5", loaders: &[NeoForge, Fabric, Quilt] },
    Version { id: "1.20.4", loaders: &[Forge, NeoForge, Fabric, Quilt] },
    Version { id: "1.20.3", loaders: &[Forge, NeoForge, Fabric, Quilt] },
    Version { id: "1.20.2", loaders: &[Forge, NeoForge, Fabric, Quilt] },
    Version { id: "1.20.1", loaders: &[Forge, NeoForge, Fabric, Quilt] },
    Version { id: "1.20", loaders: &[Forge, Fabric, Quilt] },
    Version { id: "1.19.4", loaders: &[Forge, Fabric, Quilt] },
    Version { id: "1.19.3", loaders: &[Forge, Fabric, Quilt] },
    Version { id: "1.19.2", loaders: &[Forge, Fabric, Quilt] },
    Version { id: "1.19.1", loaders: &[Forge, Fabric, Quilt] },
    Version { id: "1.19", loaders: &[Forge, Fabric, Quilt] },
    Version { id: "1.18.2", loaders: &[Forge, Fabric, Quilt] },
    Version { id: "1.18.1", loaders: &[Forge, Fabric, Quilt] },
    Version { id: "1.18", loaders: &[Forge, Fabric, Quilt] },
    Version { id: "1.17.1", loaders: &[Forge, Fabric, Quilt] },
    Version { id: "1.17", loaders: &[Fabric, Quilt] },
    Version { id: "1.16.5", loaders: &[Forge, Fabric, Quilt] },
    Version { id: "1.16.4", loaders: &[Forge, Fabric, Quilt] },
    Version { id: "1.16.3", loaders: &[Forge, Fabric, Quilt] },
    Version { id: "1.16.2", loaders: &[Forge, Fabric, Quilt] },
    Version { id: "1.16.1", loaders: &[Forge, Fabric, Quilt] },
    Version { id: "1.16", loaders: &[Fabric, Quilt] },
    Version { id: "1.15.2", loaders: &[Forge, Fabric, Quilt] },
    Version { id: "1.15.1", loaders: &[Forge, Fabric, Quilt] },
    Version { id: "1.15", loaders: &[Forge, Fabric, Quilt] },
    Version { id: "1.14.4", loaders: &[Forge, Fabric, Quilt] },
    Version { id: "1.14.3", loaders: &[Forge, Fabric] },
    Version { id: "1.14.2", loaders: &[Forge, Fabric] },
    Version { id: "1.14.1", loaders: &[Fabric] },
    Version { id: "1.14", loaders: &[Fabric] },
    Version { id: "1.13.2", loaders: &[Forge] },
    Version { id: "1.12.2", loaders: &[Forge, Cleanroom] },
    Version { id: "1.12.1", loaders: &[Forge] },
    Version { id: "1.12", loaders: &[Forge] },
    Version { id: "1.11.2", loaders: &[Forge] },
    Version { id: "1.11", loaders: &[Forge] },
    Version { id: "1.10.2", loaders: &[Forge] },
    Version { id: "1.10", loaders: &[Forge] },
    Version { id: "1.9.4", loaders: &[Forge] },
    Version { id: "1.9", loaders: &[Forge] },
    Version { id: "1.8.9", loaders: &[Forge] },
    Version { id: "1.8.8", loaders: &[Forge] },
    Version { id: "1.8", loaders: &[Forge] },
    Version { id: "1.7.10", loaders: &[Forge] },
];

/// SPDX license identifiers.
pub const LICENSES: &[&str] = &[
    "MIT",
    "Apache-2.0",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "Zlib",
    "MPL-2.0",
    "GPL-2.0-only",
    "GPL-2.0-or-later",
    "GPL-3.0-only",
    "GPL-3.0-or-later",
    "LGPL-2.1-only",
    "LGPL-2.1-or-later",
    "LGPL-3.0-only",
    "LGPL-3.0-or-later",
    "AGPL-3.0-only",
    "AGPL-3.0-or-later",
    "EUPL-1.2",
    "Unlicense",
    "CC0-1.0",
    "CC-BY-4.0",
    "CC-BY-SA-4.0",
    "CC-BY-ND-4.0",
    "CC-BY-NC-4.0",
    "CC-BY-NC-SA-4.0",
    "CC-BY-NC-ND-4.0",
    "PolyForm-Noncommercial-1.0.0",
    "PolyForm-Small-Business-1.0.0",
];
