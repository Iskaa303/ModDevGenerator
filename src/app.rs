use dioxus::prelude::*;

use crate::data::{LICENSES, LOADERS, Loader, VERSIONS};

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    let mut versions = use_signal(Vec::<&'static str>::new);
    let mut platforms = use_signal(Vec::<Loader>::new);
    let mut mod_name = use_signal(String::new);
    let mut mod_id = use_signal(String::new);
    let mut group = use_signal(String::new);
    let mut mod_version = use_signal(|| String::from("1.0.0"));
    let mut authors = use_signal(String::new);
    let license = use_signal(|| String::from("MIT"));
    let mut description = use_signal(String::new);

    let picked = versions();
    let chosen = platforms();
    let versions_text = if picked.is_empty() {
        String::from("none selected")
    } else {
        picked.join(", ")
    };
    let platforms_text = if chosen.is_empty() {
        String::from("none selected")
    } else {
        chosen.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")
    };
    let mod_text = match mod_name() {
        name if name.is_empty() => mod_version(),
        name => format!("{name} {}", mod_version()),
    };

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        header { class: "head",
            h1 { "Mod Dev Template Generator" }
            p {
                "Pick the Minecraft versions and the mod loaders to target, fill in the mod properties, then generate a project."
            }
        }

        main {
            div { class: "stack",
                section { class: "panel",
                    h2 { "Minecraft versions" }
                    div { class: "chips",
                        for version in VERSIONS {
                            button {
                                key: "{version.id}",
                                class: if picked.contains(&version.id) { "chip on" } else { "chip" },
                                onclick: move |_| toggle_version(&mut versions, &mut platforms, version.id),
                                "{version.id}"
                            }
                        }
                    }
                }

                section { class: "panel",
                    h2 { "Platforms" }
                    div { class: "chips",
                        for loader in LOADERS.iter().copied() {
                            button {
                                key: "{loader}",
                                class: if chosen.contains(&loader) { "chip on" } else { "chip" },
                                disabled: !platform_available(&picked, loader),
                                onclick: move |_| toggle_platform(&mut platforms, loader),
                                "{loader}"
                            }
                        }
                    }
                    p { class: "note", "Platforms that do not exist for any selected version are disabled." }
                }
            }

            div { class: "stack",
                section { class: "panel",
                    h2 { "Mod properties" }
                    div { class: "fields",
                        label { class: "field",
                            span { "Mod name" }
                            input {
                                value: mod_name(),
                                placeholder: "Example Mod",
                                oninput: move |event| mod_name.set(event.value()),
                            }
                        }
                        label { class: "field",
                            span { "Mod ID" }
                            input {
                                value: mod_id(),
                                placeholder: "examplemod",
                                oninput: move |event| mod_id.set(event.value()),
                            }
                        }
                        label { class: "field",
                            span { "Maven group" }
                            input {
                                value: group(),
                                placeholder: "com.example",
                                oninput: move |event| group.set(event.value()),
                            }
                        }
                        label { class: "field",
                            span { "Mod version" }
                            input {
                                value: mod_version(),
                                oninput: move |event| mod_version.set(event.value()),
                            }
                        }
                        label { class: "field",
                            span { "Authors" }
                            input {
                                value: authors(),
                                placeholder: "Your name",
                                oninput: move |event| authors.set(event.value()),
                            }
                        }
                        div { class: "field",
                            span { "License" }
                            LicensePicker { license }
                        }
                        label { class: "field wide",
                            span { "Description" }
                            textarea {
                                value: description(),
                                oninput: move |event| description.set(event.value()),
                            }
                        }
                    }
                }

                section { class: "panel",
                    h2 { "Selection" }
                    dl { class: "summary",
                        dt { "Minecraft" }
                        dd { "{versions_text}" }
                        dt { "Platforms" }
                        dd { "{platforms_text}" }
                        dt { "Mod" }
                        dd { "{mod_text}" }
                    }
                    div { class: "actions",
                        button { class: "primary", disabled: true, "Generate" }
                        span { class: "note", "Template generation is not implemented yet." }
                    }
                }
            }
        }
    }
}

#[component]
fn LicensePicker(mut license: Signal<String>) -> Element {
    let mut open = use_signal(|| false);
    let mut query = use_signal(String::new);
    let mut highlighted = use_signal(|| 0usize);

    let matches = license_matches(&query());
    let count = matches.len();
    let focused = highlighted().min(count.saturating_sub(1));
    let expanded = open() && count > 0;

    rsx! {
        div {
            class: "dropdown",
            onfocusout: move |_| open.set(false),
            input {
                class: "dropdown-input",
                r#type: "text",
                role: "combobox",
                aria_label: "License",
                aria_haspopup: "listbox",
                aria_controls: "license-listbox",
                aria_autocomplete: "list",
                aria_expanded: if expanded { "true" } else { "false" },
                aria_activedescendant: if expanded {
                    format!("license-option-{focused}")
                } else {
                    String::new()
                },
                value: license(),
                onfocus: move |_| {
                    query.set(String::new());
                    highlighted.set(selected_index(&license()));
                    open.set(true);
                },
                onclick: move |_| open.set(true),
                oninput: move |event| {
                    let text = event.value();
                    license.set(text.clone());
                    query.set(text);
                    highlighted.set(0);
                    open.set(true);
                },
                onkeydown: move |event| match event.key() {
                    Key::Escape => open.set(false),
                    Key::ArrowDown => {
                        event.prevent_default();
                        highlighted.set(focused.saturating_add(1).min(count.saturating_sub(1)));
                    }
                    Key::ArrowUp => {
                        event.prevent_default();
                        highlighted.set(focused.saturating_sub(1));
                    }
                    Key::Enter => {
                        event.prevent_default();
                        if expanded {
                            let options = license_matches(&query());
                            if let Some(pick) = options.get(focused) {
                                license.set(String::from(*pick));
                            }
                            query.set(String::new());
                            open.set(false);
                        }
                    }
                    _ => {}
                },
            }
            span { class: "dropdown-caret" }
            if expanded {
                div {
                    class: "dropdown-menu",
                    id: "license-listbox",
                    role: "listbox",
                    for (index, option) in matches.iter().copied().enumerate() {
                        div {
                            key: "{option}",
                            id: "license-option-{index}",
                            role: "option",
                            class: if index == focused { "dropdown-option active" } else { "dropdown-option" },
                            aria_selected: if license() == option { "true" } else { "false" },
                            onmousedown: move |event| event.prevent_default(),
                            onclick: move |_| {
                                license.set(String::from(option));
                                query.set(String::new());
                                open.set(false);
                            },
                            "{option}"
                        }
                    }
                }
            }
        }
    }
}

fn selected_index(license: &str) -> usize {
    LICENSES.iter().position(|option| *option == license).unwrap_or(0)
}

fn license_matches(query: &str) -> Vec<&'static str> {
    if query.is_empty() {
        return LICENSES.to_vec();
    }
    let needle = query.to_lowercase();
    LICENSES
        .iter()
        .copied()
        .filter(|option| option.to_lowercase().contains(&needle))
        .collect()
}

fn toggle_version(
    versions: &mut Signal<Vec<&'static str>>,
    platforms: &mut Signal<Vec<Loader>>,
    id: &'static str,
) {
    let mut picked = versions();
    match picked.iter().position(|picked_id| *picked_id == id) {
        Some(index) => {
            picked.remove(index);
        }
        None => picked.push(id),
    }
    versions.set(picked);

    let picked = versions();
    platforms.write().retain(|loader| platform_available(&picked, *loader));
}

fn toggle_platform(platforms: &mut Signal<Vec<Loader>>, loader: Loader) {
    let mut picked = platforms();
    match picked.iter().position(|picked_loader| *picked_loader == loader) {
        Some(index) => {
            picked.remove(index);
        }
        None => picked.push(loader),
    }
    platforms.set(picked);
}

fn platform_available(picked: &[&'static str], loader: Loader) -> bool {
    picked.is_empty() || picked.iter().any(|id| supports(id, loader))
}

fn supports(id: &str, loader: Loader) -> bool {
    VERSIONS
        .iter()
        .any(|version| version.id == id && version.loaders.contains(&loader))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_matches_known_loaders() {
        assert!(supports("1.7.10", Loader::Forge));
        assert!(!supports("1.7.10", Loader::Fabric));
        assert!(supports("1.12.2", Loader::Cleanroom));
        assert!(supports("1.20.1", Loader::NeoForge));
        assert!(supports("1.21.1", Loader::NeoForge));
        assert!(!supports("1.13.1", Loader::Forge));
    }

    #[test]
    fn matrix_is_sane() {
        let mut ids = VERSIONS.iter().map(|version| version.id).collect::<Vec<_>>();
        let count = ids.len();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), count);
        assert!(VERSIONS.iter().all(|version| !version.loaders.is_empty()));
    }

    #[test]
    fn platforms_follow_the_version_selection() {
        assert!(platform_available(&[], Loader::Cleanroom));
        assert!(platform_available(&["1.13.1", "1.12.2"], Loader::Cleanroom));
        assert!(!platform_available(&["1.13.1"], Loader::Cleanroom));
    }

    #[test]
    fn unknown_license_falls_back_to_the_first_option() {
        assert_eq!(selected_index("Apache-2.0"), 1);
        assert_eq!(selected_index("Not-A-License"), 0);
    }

    #[test]
    fn license_filter_narrows_by_substring() {
        let gpl = license_matches("gpl");
        assert!(gpl.contains(&"AGPL-3.0-only"));
        assert!(gpl.contains(&"LGPL-3.0-only"));
        assert!(gpl.contains(&"GPL-3.0-or-later"));
        assert!(!gpl.contains(&"MIT"));
        assert_eq!(license_matches("PolyForm-Small").len(), 1);
        assert_eq!(license_matches("").len(), LICENSES.len());
    }
}
