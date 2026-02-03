use actix_web::{ web, App, HttpResponse, HttpServer, Responder };
use sqlx::{ PgPool, postgres::PgPoolOptions };
use dotenvy::dotenv;
use std::env;
use serde::Deserialize;

#[derive(Deserialize)]
struct UsuarioForm {
    usuario: String,
    email: String,
    password: String,
}

// ============ VISTAS ============

async fn inicio() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(include_str!("templates/form.html"))
}

async fn pantalla1() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(include_str!("templates/pantalla1.html"))
}

async fn pantalla2() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(include_str!("templates/pantalla2.html"))
}

async fn pantalla3() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(include_str!("templates/pantalla3.html"))
}

// ============ INSERTAR USUARIO ============

async fn crear_usuario(pool: web::Data<PgPool>, form: web::Form<UsuarioForm>) -> impl Responder {
    println!("▶ Recibiendo datos: {} | {}", form.usuario, form.email);

    let resultado = sqlx
        ::query(
            r#"
        INSERT INTO usuarios (usuario, email, password)
        VALUES ($1, $2, $3)
        "#,
            form.usuario,
            form.email,
            form.password
        )
        .execute(pool.get_ref()).await;

    match resultado {
        Ok(_) => HttpResponse::Ok().body("Usuario registrado correctamente"),
        Err(e) => {
            eprintln!("❌ Error al insertar: {:?}", e);
            HttpResponse::InternalServerError().body("Error al registrar usuario")
        }
    }
}

// ============ MAIN ============

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("❌ DATABASE_URL no definida");

    // ✅ Pool recomendado para Neon
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await
        .expect("❌ Error conectando a Neon");

    let port: u16 = std::env
        ::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT inválido");

    println!("🚀 Servidor ejecutándose en http://0.0.0.0:{port}");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(inicio))
            .route("/pantalla1", web::get().to(pantalla1))
            .route("/pantalla2", web::get().to(pantalla2))
            .route("/pantalla3", web::get().to(pantalla3))
            .route("/crear_usuario", web::post().to(crear_usuario))
    })
        .bind(("0.0.0.0", port))?
        // HttpServer::new(move || {
        //     App::new()
        //         .app_data(web::Data::new(pool.clone()))
        //         .route("/", web::get().to(inicio))
        //         .route("/pantalla1", web::get().to(pantalla1))
        //         .route("/pantalla2", web::get().to(pantalla2))
        //         .route("/pantalla3", web::get().to(pantalla3))
        //         .route("/crear_usuario", web::post().to(crear_usuario))
        // })
        // .bind(("0.0.0.0", 8080))?
        .run().await
}
