#[derive(PartialEq, Eq, Hash)]
pub struct DynamicPath(Vec<String>);

impl DynamicPath {
    pub fn parse(path: &str) -> DynamicPath {
        let mut new_path = Vec::new();

        for letter in path.split("/") {
            if letter.is_empty() {
                continue;
            }

            new_path.push(letter.to_string());
        }

        DynamicPath(new_path)
    }
}

impl ToString for DynamicPath {
    fn to_string(&self) -> String {
        format!("/{}", self.0.join("/"))
    }
}
