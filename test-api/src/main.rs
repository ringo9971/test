use actix_cors::Cors;
use actix_web::{
    dev::Payload, http::header, web, App, FromRequest, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use anyhow::Context;
use diesel::Queryable;
use diesel::{Connection, PgConnection};
use futures::future::{err, ok, Ready};
use serde::{Deserialize, Serialize};

mod schema;

const DOTENV_PATH: &str = ".env";

pub fn establish_connection() -> anyhow::Result<PgConnection> {
    dotenv::from_filename(DOTENV_PATH).context("Failed to load .env file")?;

    let database_url =
        std::env::var("DATABASE_URL").context("DATABASE_URL not found in .env file")?;

    PgConnection::establish(&database_url).context("Error connecting to the database")
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct GetUsersQuery {
    pub name: Option<String>,
    pub email: Option<String>,
}

impl FromRequest for GetUsersQuery {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, actix_web::Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let query_string = req.query_string().to_owned();

        serde_qs::from_str::<GetUsersQuery>(&query_string)
            .map(ok)
            .map_err(actix_web::error::ErrorBadRequest)
            .unwrap_or_else(err)
    }
}

pub mod read {
    use anyhow::Error;
    use chrono::NaiveDateTime;
    use diesel::{ExpressionMethods, QueryDsl, Queryable, RunQueryDsl};
    use serde::Serialize;

    use crate::establish_connection;
    use crate::schema::users::dsl;
    use crate::GetUsersQuery;

    #[derive(Debug, Serialize, Queryable)]
    pub struct User {
        pub id: i32,
        pub user_uid: String,
        pub created_at: NaiveDateTime,
        pub name: String,
        pub email: Option<String>,
    }

    #[derive(Debug, Serialize)]
    pub struct GetUsersResponse {
        pub total: usize,
        pub data: Vec<User>,
    }

    pub fn get_users(query: GetUsersQuery) -> Result<GetUsersResponse, Error> {
        let mut connection = establish_connection().expect("error");

        let mut q = dsl::users.into_boxed();

        if let Some(name) = &query.name {
            q = q.filter(dsl::name.eq(name))
        };

        let res = match q.load::<User>(&mut connection) {
            Ok(res) => res,
            Err(err) => {
                return Err(Error::msg(err));
            }
        };

        let total = res.len();

        Ok(GetUsersResponse { total, data: res })
    }

    pub fn get_user(user_uid: String) -> Result<User, Error> {
        let mut connection = establish_connection().expect("error");

        let res = match dsl::users
            .filter(dsl::user_uid.eq(user_uid))
            .first::<User>(&mut connection)
        {
            Ok(res) => res,
            Err(err) => {
                return Err(Error::msg(err));
            }
        };

        Ok(res)
    }
}

pub mod create {
    use anyhow::Error;
    use diesel::Insertable;
    use diesel::RunQueryDsl;
    use serde::{Deserialize, Serialize};

    use crate::establish_connection;
    use crate::schema::users;
    use crate::schema::users::dsl;

    #[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
    #[diesel(table_name=users)]
    pub struct User {
        pub name: String,
        pub email: Option<String>,
    }

    pub fn create_user(user: User) -> Result<(), Error> {
        let mut connection = establish_connection().expect("error");

        diesel::insert_into(dsl::users)
            .values(&user)
            .execute(&mut connection)
            .expect("error");

        Ok(())
    }
}

pub mod update {
    use anyhow::Error;
    use diesel::Insertable;
    use diesel::{AsChangeset, ExpressionMethods, QueryDsl, RunQueryDsl};
    use serde::{Deserialize, Serialize};

    use crate::establish_connection;
    use crate::schema::users;
    use crate::schema::users::dsl;

    #[derive(Debug, Clone, Serialize, Deserialize, Insertable, AsChangeset)]
    #[diesel(table_name=users)]
    pub struct User {
        pub name: Option<String>,
        pub email: Option<String>,
    }

    pub fn update_user(user_uid: String, user: User) -> Result<(), Error> {
        let mut connection = establish_connection().expect("error");

        let target = dsl::users.filter(dsl::user_uid.eq(user_uid));

        diesel::update(target)
            .set(&user)
            .execute(&mut connection)
            .expect("error");

        Ok(())
    }
}

pub mod delete {
    use anyhow::Error;
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

    use crate::establish_connection;
    use crate::schema::users::dsl;

    pub fn delete_user(user_uid: String) -> Result<(), Error> {
        let mut connection = establish_connection().expect("error");

        let target = dsl::users.filter(dsl::user_uid.eq(user_uid));

        diesel::delete(target)
            .execute(&mut connection)
            .expect("error");

        Ok(())
    }
}

async fn get_users(query: GetUsersQuery) -> impl Responder {
    match read::get_users(query) {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

async fn get_user(user_uid: web::Path<String>) -> impl Responder {
    match read::get_user(user_uid.to_owned()) {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

async fn create_user(req: web::Json<create::User>) -> impl Responder {
    match create::create_user(req.clone()) {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

async fn update_user(user_uid: web::Path<String>, req: web::Json<update::User>) -> impl Responder {
    match update::update_user(user_uid.clone(), req.clone()) {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

async fn delete_user(user_uid: web::Path<String>) -> impl Responder {
    match delete::delete_user(user_uid.clone()) {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allow_any_method()
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .route("/users", web::get().to(get_users))
            .route("/users/{user_uid}", web::get().to(get_user))
            .route("/users", web::post().to(create_user))
            .route("/users/{user_uid}", web::put().to(update_user))
            .route("/users/{user_uid}", web::delete().to(delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
