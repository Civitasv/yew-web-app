use std::error::Error;

use serde::de::DeserializeOwned;

#[derive(Debug)]
pub struct Document<T: DeserializeOwned> {
    pub metadata: T,
    pub content: String,
}

pub struct YamlFormatter;

impl YamlFormatter {
    pub fn parse<T: DeserializeOwned>(markdown: &str) -> Result<Document<T>, Box<dyn Error>> {
        let yaml = YamlFormatter::extract(markdown)?;
        let metadata = serde_yaml::from_str::<T>(yaml.0.as_str())?;
        Ok(Document {
            metadata,
            content: yaml.1,
        })
    }

    fn extract(markdown: &str) -> Result<(String, String), Box<dyn Error>> {
        let mut result = String::default();
        let mut has_start = false;
        let mut config_and_empty_lines = 0;
        let lines = markdown.lines();

        for line in lines.clone() {
            config_and_empty_lines += 1;
            if line.trim() == "---" {
                if has_start {
                    break;
                }

                has_start = true;
                continue;
            }
            if has_start {
                result.push_str(line);
                result.push('\n');
            }
        }

        Ok((
            result,
            lines
                .skip(config_and_empty_lines)
                .collect::<Vec<&str>>()
                .join("\n"),
        ))
    }
}

#[cfg(test)]
mod test {
    use std::fs::{self, File};

    use serde::Deserialize;

    use crate::utils::markdown_yaml::YamlFormatter;

    #[derive(Deserialize, Debug)]
    struct Metadata {
        title: String,
        author: String,
        outline: String,
    }
    #[test]
    fn test() {
        let mdstr = fs::read_to_string("./mdfiles/test.md").unwrap();
        println!("{}", mdstr);

        let result = YamlFormatter::parse::<Metadata>(&mdstr).unwrap();
        println!("{:?}", result.metadata);
    }
}
