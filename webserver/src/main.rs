use anyhow::Result;
use sqlx::Row;
use axum::Extension;

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
struct BlogPost {
    id: i32,
    date: String,
    title: String,
    body: String,
    author: String,
}


#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    println!("Connected to: {database_url}");

    // Setup the database
    let pool = get_connection_pool(&database_url).await?; // this is an Arc, designed to be cloned.
    println!("Running migrations");
    run_migrations(pool.clone()).await?;

    // TCP Listener
    let listen_address = std::env::var("LISTEN_ADDRESS")?;
    println!("Listening on: {listen_address}");
    let listener = tokio::net::TcpListener::bind(&listen_address).await?;

    // Build an Axum Router and run it
    let app = axum::Router::new()
        .route("/hello", axum::routing::get(say_hello))
        .route("/", axum::routing::get(get_blog_posts_handler))
        .route("/:id", axum::routing::get(get_blog_post_handler))
        .route("/add", axum::routing::post(add_blog_post_handler))
        .route("/update/:id", axum::routing::post(update_blog_post_handler))
        .route("/delete/:id", axum::routing::post(delete_blog_post_handler))
        .layer(Extension(pool.clone()));
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_connection_pool(url: &str) -> Result<sqlx::SqlitePool> {
    let connection_pool = sqlx::SqlitePool::connect(url)
        .await?;
    Ok(connection_pool)
}

async fn run_migrations(pool: sqlx::SqlitePool) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(())
}

async fn get_blog_posts(pool: sqlx::SqlitePool) -> Result<Vec<BlogPost>> {
    let posts = sqlx::query_as::<_, BlogPost>("SELECT * FROM blog_posts")
        .fetch_all(&pool)
        .await?;
    Ok(posts)
}

async fn get_blog_post(pool: sqlx::SqlitePool, id: i32) -> Result<BlogPost> {
    let post = sqlx::query_as::<_, BlogPost>("SELECT * FROM blog_posts WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    Ok(post)
}

async fn add_blog_post(pool: sqlx::SqlitePool, date: String, title: String, body: String, author: String) -> Result<i32> {
    let id: i32 = sqlx::query("INSERT INTO blog_posts (date, title, body, author) VALUES (?, ?, ?, ?); SELECT last_insert_rowid();")
        .bind(date)
        .bind(title)
        .bind(body)
        .bind(author)
        .fetch_one(&pool)
        .await?
        .get(0);

    Ok(id)
}


async fn update_blog_post(pool: sqlx::SqlitePool, id: i32, date: String, title: String, body: String, author: String) -> Result<()> {
    sqlx::query("UPDATE blog_posts SET date = ?, title = ?, body = ?, author = ? WHERE id = ?")
        .bind(date)
        .bind(title)
        .bind(body)
        .bind(author)
        .bind(id)
        .execute(&pool)
        .await?;

    Ok(())
}

async fn delete_blog_post(pool: sqlx::SqlitePool, id: i32) -> Result<()> {
    sqlx::query("DELETE FROM blog_posts WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    Ok(())
}

async fn say_hello() -> &'static str {
    "Hello World!"
}

async fn get_blog_posts_handler(
    Extension(pool): Extension<sqlx::SqlitePool>,
) -> axum::Json<Vec<BlogPost>> {
    let posts = get_blog_posts(pool).await.unwrap();
    axum::Json(posts)
}

async fn get_blog_post_handler(
    Extension(pool): Extension<sqlx::SqlitePool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> axum::Json<BlogPost> {
    let post = get_blog_post(pool, id).await.unwrap();
    axum::Json(post)
}

async fn add_blog_post_handler(
    Extension(pool): Extension<sqlx::SqlitePool>,
    axum::extract::Json(post): axum::extract::Json<BlogPost>,
) -> axum::Json<i32> {
    let id = add_blog_post(pool, post.date, post.title, post.body, post.author).await.unwrap();
    axum::Json(id)
}

async fn update_blog_post_handler(
    Extension(pool): Extension<sqlx::SqlitePool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
    axum::extract::Json(post): axum::extract::Json<BlogPost>,
) -> axum::Json<()> {
    update_blog_post(pool, id, post.date, post.title, post.body, post.author).await.unwrap();
    axum::Json(())
}

async fn delete_blog_post_handler(
    Extension(pool): Extension<sqlx::SqlitePool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> axum::Json<()> {
    delete_blog_post(pool, id).await.unwrap();
    axum::Json(())
}
