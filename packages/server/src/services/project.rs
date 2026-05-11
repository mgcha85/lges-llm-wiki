use std::fs;
use std::path::Path;

use chrono::Local;
use uuid::Uuid;

use crate::config::{data_dir, validate_path};
use crate::types::WikiProject;

pub fn list_projects() -> Result<Vec<WikiProject>, String> {
    let dir = data_dir();
    let mut projects = Vec::new();

    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read DATA_DIR: {}", e))?;

    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() && is_valid_project(&path) {
            let name = entry.file_name().to_string_lossy().to_string();
            let id = get_or_create_project_id(&path)?;
            projects.push(WikiProject {
                id,
                name,
                path: path.to_string_lossy().replace('\\', "/"),
            });
        }
    }

    projects.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(projects)
}

pub fn create_project(name: &str) -> Result<WikiProject, String> {
    let root = data_dir().join(name);

    if root.exists() {
        return Err(format!("Project '{}' already exists", name));
    }

    let dirs = [
        "raw/sources",
        "raw/assets",
        "wiki/entities",
        "wiki/concepts",
        "wiki/sources",
        "wiki/queries",
        "wiki/comparisons",
        "wiki/synthesis",
        ".llm-wiki",
    ];

    for dir in &dirs {
        fs::create_dir_all(root.join(dir))
            .map_err(|e| format!("Failed to create directory '{}': {}", dir, e))?;
    }

    let today = Local::now().format("%Y-%m-%d").to_string();
    let id = Uuid::new_v4().to_string();

    fs::write(root.join(".llm-wiki/project-id"), &id)
        .map_err(|e| format!("Failed to write project ID: {}", e))?;

    write_template_file(&root, "schema.md", &create_schema_content())?;
    write_template_file(&root, "purpose.md", PURPOSE_TEMPLATE)?;
    write_template_file(&root, "wiki/index.md", INDEX_TEMPLATE)?;
    write_template_file(&root, "wiki/log.md", &create_log_content(&today))?;
    write_template_file(&root, "wiki/overview.md", OVERVIEW_TEMPLATE)?;

    create_obsidian_config(&root)?;

    Ok(WikiProject {
        id,
        name: name.to_string(),
        path: root.to_string_lossy().replace('\\', "/"),
    })
}

pub fn open_project(name: &str) -> Result<WikiProject, String> {
    let root = data_dir().join(name);
    
    validate_path(&root)?;

    if !root.exists() {
        return Err(format!("Project '{}' does not exist", name));
    }

    if !is_valid_project(&root) {
        return Err(format!("'{}' is not a valid wiki project", name));
    }

    let id = get_or_create_project_id(&root)?;

    Ok(WikiProject {
        id,
        name: name.to_string(),
        path: root.to_string_lossy().replace('\\', "/"),
    })
}

fn is_valid_project(path: &Path) -> bool {
    path.join("schema.md").exists() && path.join("wiki").is_dir()
}

fn get_or_create_project_id(path: &Path) -> Result<String, String> {
    let id_file = path.join(".llm-wiki/project-id");
    
    if id_file.exists() {
        fs::read_to_string(&id_file)
            .map(|s| s.trim().to_string())
            .map_err(|e| format!("Failed to read project ID: {}", e))
    } else {
        let id = Uuid::new_v4().to_string();
        fs::create_dir_all(path.join(".llm-wiki"))
            .map_err(|e| format!("Failed to create .llm-wiki dir: {}", e))?;
        fs::write(&id_file, &id)
            .map_err(|e| format!("Failed to write project ID: {}", e))?;
        Ok(id)
    }
}

fn write_template_file(root: &Path, relative_path: &str, content: &str) -> Result<(), String> {
    let path = root.join(relative_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create parent dirs: {}", e))?;
    }
    fs::write(&path, content)
        .map_err(|e| format!("Failed to write '{}': {}", relative_path, e))
}

fn create_obsidian_config(root: &Path) -> Result<(), String> {
    let obsidian_dir = root.join(".obsidian");
    fs::create_dir_all(&obsidian_dir)
        .map_err(|e| format!("Failed to create .obsidian: {}", e))?;

    write_template_file(root, ".obsidian/app.json", OBSIDIAN_APP_CONFIG)?;
    write_template_file(root, ".obsidian/appearance.json", OBSIDIAN_APPEARANCE)?;
    write_template_file(root, ".obsidian/core-plugins.json", OBSIDIAN_CORE_PLUGINS)?;

    Ok(())
}

fn create_schema_content() -> String {
    r#"# Wiki Schema

## Page Types

| Type | Directory | Purpose |
|------|-----------|---------|
| entity | wiki/entities/ | Named things (models, companies, people, datasets) |
| concept | wiki/concepts/ | Ideas, techniques, phenomena |
| source | wiki/sources/ | Papers, articles, talks, blog posts |
| query | wiki/queries/ | Open questions under investigation |
| comparison | wiki/comparisons/ | Side-by-side analysis of related entities |
| synthesis | wiki/synthesis/ | Cross-cutting summaries and conclusions |

## Naming Conventions

- Files: `kebab-case.md`
- Entities: match official name where possible (e.g., `gpt-4.md`, `openai.md`)
- Concepts: descriptive noun phrases (e.g., `chain-of-thought.md`)
- Sources: `author-year-slug.md` (e.g., `wei-2022-chain-of-thought.md`)
- Queries: question as slug (e.g., `does-scale-improve-reasoning.md`)

## Frontmatter

All pages must include YAML frontmatter:

```yaml
---
type: entity | concept | source | query | comparison | synthesis | overview
title: Human-readable title
tags: []
related: []
created: YYYY-MM-DD
updated: YYYY-MM-DD
---
```

Source pages also include:
```yaml
authors: []
year: YYYY
url: ""
venue: ""
```

## Cross-referencing Rules

- Use `[[page-slug]]` syntax to link between wiki pages
- Every entity and concept should appear in `wiki/index.md`
- Queries link to the sources and concepts they draw on
- Synthesis pages cite all contributing sources via `related:`
"#.to_string()
}

fn create_log_content(today: &str) -> String {
    format!(
        r#"# Research Log

## {}

- Project created
"#,
        today
    )
}

const PURPOSE_TEMPLATE: &str = r#"# Project Purpose

## Goal

<!-- What are you trying to understand or build? -->

## Key Questions

<!-- List the primary questions driving this research -->

1.
2.
3.

## Scope

**In scope:**
-

**Out of scope:**
-

## Thesis

> TBD
"#;

const INDEX_TEMPLATE: &str = r#"# Wiki Index

## Entities

## Concepts

## Sources

## Queries

## Comparisons

## Synthesis
"#;

const OVERVIEW_TEMPLATE: &str = r#"---
type: overview
title: Project Overview
tags: []
related: []
---

# Overview

<!-- Provide a high-level summary of what this wiki covers and its current state. -->
"#;

const OBSIDIAN_APP_CONFIG: &str = r#"{
  "attachmentFolderPath": "raw/assets",
  "userIgnoreFilters": [".cache", ".llm-wiki"],
  "useMarkdownLinks": false,
  "newLinkFormat": "shortest",
  "showUnsupportedFiles": false
}"#;

const OBSIDIAN_APPEARANCE: &str = r#"{
  "baseFontSize": 16,
  "theme": "obsidian"
}"#;

const OBSIDIAN_CORE_PLUGINS: &str = r#"{
  "file-explorer": true,
  "global-search": true,
  "graph": true,
  "backlink": true,
  "tag-pane": true,
  "page-preview": true,
  "outgoing-link": true,
  "starred": true
}"#;
