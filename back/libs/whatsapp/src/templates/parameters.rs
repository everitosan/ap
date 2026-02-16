use crate::templates::builder::ComponentParam;

#[derive(Debug, Default)]
pub struct Parameters {
    pub body: Vec<ComponentParam>,
    pub header: Vec<ComponentParam>,
    pub buttons: Vec<ComponentParam>,
    pub footer: Vec<ComponentParam>,
}

impl Parameters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_body(mut self, params: Vec<ComponentParam>) -> Self {
        self.body = params;
        self
    }

    pub fn with_header(mut self, params: Vec<ComponentParam>) -> Self {
        self.header = params;
        self
    }

    pub fn with_buttons(mut self, params: Vec<ComponentParam>) -> Self {
        self.buttons = params;
        self
    }

    pub fn with_footer(mut self, params: Vec<ComponentParam>) -> Self {
        self.footer = params;
        self
    }
}
