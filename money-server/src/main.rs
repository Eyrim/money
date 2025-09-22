use lib_money::{self, html, head, body};
use rocket::response::content;


#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> content::RawHtml<String> {
    let page = html! {
        head {
            "title" => "my page"
        },
        body {
            "<h1>hi!!!!</h1>"
        }
    };

    content::RawHtml(page)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}

/*
#[derive(HTMLElement)]
#[html(className = "myClassName", parent = HTMLElement::div)]
pub struct MyComponent {
    pub body: &str; 
}

impl HTMLElement for MyComponent {
    pub fn render(&self) -> String {
        let className = "myClassName";
        let body = self.body;

        format!("<div className=\"{}\">\n{}\n</div>", className, body)
    }
}

// impl HTMLBody for MyComponent {
//     pub fn render_body(&self) -> String {
//     }
// }
 */
