use serde::Serialize;
use String;
use rocket::{data::{Data, FromData, Outcome}, request::Request, serde::json::Json};
#[macro_use] extern crate rocket;

mod lexer;

#[derive(Serialize)]
struct defaultResponse {
    message: String,
}

#[derive(Debug, Serialize)]
struct Code {
    code: &'static str,
}

#[derive(Debug)]
struct CodeError;

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for CodeRequest<'r> {
//     type Error = CodeError;

//     async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
//         // let code = request
//         let code = "example_code";
//         Outcome::Success(CodeRequest(code))
//     }
// }

#[rocket::async_trait]
impl<'r> FromData<'r> for Code {
    type Error = CodeError;
    async fn from_data (request: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        // Here you can read the data and create a Code instance
        // For simplicity, we'll just return a static string
        let code: &str = "example_code";
        Outcome::Success(Code { code })
    }    
}

#[get("/")]
fn index<'r> () -> Json<defaultResponse> {
    let response = defaultResponse {
        message: String::from("Hello"),
    };
    Json(response)
}

#[post("/", data = "<data>")]
fn create(data: Code) -> &'static str {
    let Code { code } = data;
    let mut lexer = lexer::Lexer::new(code);
    lexer.exec();
    "done"
}

#[get("/b")]
fn indexb() -> &'static str {
    "Hello, worldb!"
}
// use rocket::tokio::time::{sleep, Duration};

// #[get("/delay/<seconds>")]
// async fn delay(seconds: u64) -> String {
//     sleep(Duration::from_secs(seconds)).await;
//     format!("Waited for {} seconds", seconds)
// }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, indexb, create])
}