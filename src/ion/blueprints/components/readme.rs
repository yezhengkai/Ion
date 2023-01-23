use crate::blueprints::*;
use dialoguer::Input;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct Info;

#[derive(Debug, Deserialize)]
pub struct Readme {
    #[serde(default = "Readme::default_template")]
    template: TemplateFile,
    #[serde(default = "Readme::default_inline_badge")]
    pub inline_badge: bool,
}

impl Readme {
    pub fn default_template() -> TemplateFile {
        TemplateFile::from_str("./README.md.hbs")
    }

    pub fn default_inline_badge() -> bool {
        true
    }
}

impl Blueprint for Readme {
    fn render(&self, _t: &Template, ctx: &Context) -> RenderResult {
        self.template.render(ctx, "README.md")
    }

    fn prompt(&self, _t: &Template, ctx: &mut Context) -> RenderResult {
        let input = Input::<String>::new()
            .with_prompt("description of the project")
            .allow_empty(true)
            .interact_text()
            .expect("error reading description");
        ctx.project.description = Some(input);
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Badge {
    hover: String,
    image: String,
    link: String,
}

impl Badge {
    pub fn render(&self) -> String {
        format!("[![{}]({})]({})", self.hover, self.image, self.link)
    }
}
