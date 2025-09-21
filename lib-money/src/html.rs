use strum_macros::{self, Display, EnumString};

#[derive(EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum HeadKeys {
    Title,
    // Base, //TODO: Need to write proc macro or smth for these where there are multiple args, just force dev to pass in a func with the named args
    Stylesheet,
    Script,
}

impl HeadKeys {
    //TODO: These should be compile time checked that they actually exist, and eventually that they contain valid stuff
    pub fn render(&self, value: &str) -> String {
        match self {
            Self::Title => format!(r#"<title>{}</title>"#, value).to_string(),
            Self::Stylesheet => format!(r#"<link rel="stylesheet" href="{}" />"#, value).to_string(),
            Self::Script => format!(r#"<script src="{}" />"#, value).to_string(),
        }
    }
}

#[macro_export]
macro_rules! head {
    ($($k:expr => $v:expr),+) => {
        {
            use std::str::FromStr;
            let mut output: Vec<String> = vec![];
            output.push("<head>".to_owned());

            $(
                let k: &str = $k;
                let key: $crate::html::HeadKeys = match $crate::html::HeadKeys::from_str(k) {
                    Ok(key) => key,
                    Err(e) => panic!("oh noes"), //TODO:
                };
                
                output.push(key.render($v));
            )+

            //TODO: This should be platform specific (or does html specify a standard?)
            output.push("</head>".to_owned());
            output.join("\n")
        }
    };
}

#[macro_export]
macro_rules! body {
    ($($c:expr),+) => {
        {
            let mut output: Vec<String> = vec![];

            output.push("<body>".to_owned());
            
            $(
                let content: &str = $c;
                output.push(content.to_owned());
            )+

            output.push("</body>".to_owned());
            output.join("\n") //TODO: proper newlines 
        }
    };
}

#[macro_export]
macro_rules! html {
    (head { $($k:expr => $v:expr),+ }, body { $b:expr }) => {
        {
            let mut output: Vec<String> = vec![];
            output.push("<!DOCTYPE html>".to_owned());
            output.push("<html>".to_owned());

            let head: String = head!($($k => $v)+);
            let body: String = body!($b);
            output.push(head);
            output.push(body);

            output.push("</html>".to_owned());
            output.join("\n")
        }
    };
}
