pub mod vault {
    pub fn create(name: &str, use_at_once: bool, remark: &Option<String>) -> String {
        let remark = if let Some(r) = remark {
            if r.is_empty() {
                "".to_string()
            } else {
                format!(" -r {}", r)
            }
        } else {
            "".to_string()
        };
        let use_at_once = if use_at_once { " -u" } else { "" };
        let arg = format!("{}{}{}", name, remark, use_at_once);
        arg
    }
}
