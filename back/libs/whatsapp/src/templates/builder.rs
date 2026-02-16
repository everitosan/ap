use serde_json::{Value, json};

use crate::templates::parameters::Parameters;

#[derive(Debug, Clone)]
pub enum ComponentParam {
    Text(String),
    Url(String),
    Flow { token: String, data: Value },
}

pub fn build_components(params: &Parameters) -> Vec<Value> {
    let mut components = Vec::new();

    if !params.header.is_empty() {
        components.push(json!({
            "type": "header",
            "parameters": build_params(&params.header),
        }));
    }

    if !params.body.is_empty() {
        components.push(json!({
            "type": "body",
            "parameters": build_params(&params.body),
        }));
    }

    if !params.footer.is_empty() {
        components.push(json!({
            "type": "footer",
            "parameters": build_params(&params.footer),
        }));
    }

    for (index, button) in params.buttons.iter().enumerate() {
        match button {
            ComponentParam::Url(url) => {
                components.push(json!({
                    "type": "button",
                    "sub_type": "url",
                    "index": index,
                    "parameters": [{ "type": "text", "text": url }],
                }));
            }
            ComponentParam::Flow { token, data } => {
                components.push(json!({
                    "type": "button",
                    "sub_type": "flow",
                    "index": index,
                    "parameters": [{
                        "type": "action",
                        "action": {
                            "flow_token": token,
                            "flow_action_data": data,
                        }
                    }],
                }));
            }
            ComponentParam::Text(text) => {
                components.push(json!({
                    "type": "button",
                    "sub_type": "quick_reply",
                    "index": index,
                    "parameters": [{ "type": "text", "text": text }],
                }));
            }
        }
    }

    components
}

fn build_params(params: &[ComponentParam]) -> Vec<Value> {
    params
        .iter()
        .map(|p| match p {
            ComponentParam::Text(text) => json!({ "type": "text", "text": text }),
            ComponentParam::Url(url) => json!({ "type": "text", "text": url }),
            ComponentParam::Flow { token, data } => json!({
                "type": "action",
                "action": { "flow_token": token, "flow_action_data": data }
            }),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_body_only() {
        let params = Parameters::new().with_body(vec![
            ComponentParam::Text("Hello".into()),
            ComponentParam::Text("World".into()),
        ]);

        let components = build_components(&params);
        assert_eq!(components.len(), 1);
        assert_eq!(components[0]["type"], "body");

        let body_params = components[0]["parameters"].as_array().unwrap();
        assert_eq!(body_params.len(), 2);
        assert_eq!(body_params[0]["text"], "Hello");
        assert_eq!(body_params[1]["text"], "World");
    }

    #[test]
    fn build_header_body_and_buttons() {
        let params = Parameters::new()
            .with_header(vec![ComponentParam::Text("Header".into())])
            .with_body(vec![ComponentParam::Text("Body".into())])
            .with_buttons(vec![ComponentParam::Url("https://example.com".into())]);

        let components = build_components(&params);
        assert_eq!(components.len(), 3);
        assert_eq!(components[0]["type"], "header");
        assert_eq!(components[1]["type"], "body");
        assert_eq!(components[2]["type"], "button");
        assert_eq!(components[2]["sub_type"], "url");
        assert_eq!(components[2]["index"], 0);
    }

    #[test]
    fn build_flow_button() {
        let params = Parameters::new().with_buttons(vec![ComponentParam::Flow {
            token: "tok_123".into(),
            data: json!({"screen": "MAIN"}),
        }]);

        let components = build_components(&params);
        assert_eq!(components.len(), 1);
        assert_eq!(components[0]["sub_type"], "flow");

        let action = &components[0]["parameters"][0]["action"];
        assert_eq!(action["flow_token"], "tok_123");
        assert_eq!(action["flow_action_data"]["screen"], "MAIN");
    }

    #[test]
    fn build_empty_params() {
        let params = Parameters::new();
        let components = build_components(&params);
        assert!(components.is_empty());
    }
}
