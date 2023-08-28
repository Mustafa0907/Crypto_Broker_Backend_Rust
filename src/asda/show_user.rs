use web::models::*;
use diesel::prelude::*;
use web::*;
use web::establish_connection;

fn asd() {
    use web::schema::user_info::dsl::*;

    let connection = &mut establish_connection();
    let results = user_info
        .limit(5)
        .load::<User>(connection)
        .expect("Error loading posts");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.user_name);
        println!("-----------\n");
        println!("{}", user.email);
    }
}