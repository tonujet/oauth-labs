use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![all_options]
}




#[options("/<_..>")]
fn all_options() {}
