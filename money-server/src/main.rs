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
