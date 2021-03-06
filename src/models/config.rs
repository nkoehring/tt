use std::path::PathBuf;
use toml::value::Table;
use tomlbox::*;

#[derive(Debug)]
pub struct Config {
    pub default_project: Option<String>,
    pub default_currency: Option<String>,
    pub default_rate: Option<f64>,
    pub projects: Vec<ProjectCfg>
}

impl Config {
    pub fn new() -> Self {
        Config {
            default_project: None,
            default_currency: None,
            default_rate: None,
            projects: vec![],
        }
    }

    pub fn from_toml(cfg: &Table) -> Self {
        let default_project = cfg_string("default_project", cfg);
        let default_currency = cfg_string("default_currency", cfg);
        let default_rate = cfg_number("default_rate", cfg);

        let default_keys = ["default_project", "default_currency", "default_rate"];
        let mut projects = vec![];

        for (key, value) in cfg {
            if default_keys.contains(&key.as_ref()) { continue; }

            let value = value.as_table().expect("Project entries need subkeys");
            let project = ProjectCfg {
                name: key.clone(),
                path: cfg_path("path", value).expect("Project path is obligatory!"),
                currency: cfg_string("currency", value).or(default_currency.clone()),
                rate: cfg_number("rate", value).or(default_rate.clone()),
            };
            projects.push(project);
        }

        Config {
            default_project: default_project,
            default_currency: default_currency,
            default_rate: default_rate,
            projects: projects,
        }
    }

    pub fn project(&self, project_name: Option<&str>) -> Option<&ProjectCfg> {
        let mut projects = self.projects.iter();

        if let Some(name) = project_name {
            projects.find(|p| p.name.as_str() == name)
        } else if let Some(ref name) = self.default_project {
            projects.find(|p| &p.name == name)
        } else {
            projects.next()
        }
    }
}

#[derive(Debug)]
pub struct ProjectCfg {
    pub name: String,
    pub path: PathBuf,
    pub currency: Option<String>,
    pub rate: Option<f64>,
    //TODO 'budget' could describe the project budget
    //TODO 'budget_type' could describe if it is a daily, weekly, monthly or total budget
    //TODO 'budget_unit' could describe if it is a money or time budget
}
