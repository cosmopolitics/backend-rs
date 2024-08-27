use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(serde::Deserialize, serde::Serialize)]
struct Wizard {
    name: String,
    level: u8,
}

struct Repo {
    wizards: HashMap<String, Wizard>,
}

impl Repo {
    fn new() -> Self { Self {wizards: HashMap::new()} }
}

type State = Arc<RwLock<Repo>>;

async fn create( mut req: tide::Request<State>) -> tide::Result{
    let wizard: Wizard = req.body_json().await?;
    let state = req.state();
    let mut repo = state.write();

    repo.unwrap().wizards.insert(wizard.name.clone(), wizard);

    Ok(tide::Response::new(200))
}

async fn get(req: tide::Request<State>) -> tide::Result {
    let state = req.state();
    let repo = &state.read();

    let resp = tide::Response::builder(200)
        .body(tide::Body::from_json(&repo.as_ref().unwrap().wizards)?)
        .build();
    Ok(resp)
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    femme::start();

    let mut repository = Arc::new(RwLock::new(Repo::new()));
    let mut app = tide::with_state(repository);
    app.with(tide::log::LogMiddleware::new());

    app.at("/").post(create);
    app.at("/").get(get);

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}


// impl Default for Animal {
//     fn default() -> Self {
//         Self { 
//             name: "world".to_string(),
//             legs: 2,
//         }
//     }
// }
