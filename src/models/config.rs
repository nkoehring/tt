use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub default_project: Option<String>,
    pub default_currency: Option<String>,
    pub default_rate: Option<f64>,
    pub projects: Vec<ProjectCfg>
}

impl Config {
    pub fn project(&self) -> Option<&ProjectCfg> {
        Some(&self.projects[0])
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
