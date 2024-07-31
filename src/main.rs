#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use chrono::{Duration, Utc};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use std::fs;
    use tryrust::app::*;
    use tryrust::fileserv::file_and_error_handler;
    use tryrust::redirect::redirect_www;
    use tokio_cron_scheduler::{Job, JobScheduler};
    use tower_http::trace::TraceLayer;

    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_max_level(tracing::Level::INFO)
        .init();

    let scheduler = JobScheduler::new().await.unwrap();
    let _ = scheduler.add(
        Job::new_async("0 * * * * *", |_uuid, _l| {
            Box::pin(async move {
                let paths = fs::read_dir("./sessions").unwrap();
                for path in paths {
                    let full_folder_path = path.unwrap().path();
                    let folder_last_part = full_folder_path.components().last().unwrap().as_os_str().to_str().unwrap();
                    let session_creation_time = folder_last_part.parse::<i64>().unwrap();
                    let three_days_old = (Utc::now() - Duration::days(3)).timestamp_millis();
                    if session_creation_time < three_days_old {
                        let _ = fs::remove_dir_all(full_folder_path);
                    }
                }
                tracing::info!("Cleaning folders older than 3 days...");
            })
        }).unwrap()
    ).await;
    let _ = scheduler.start().await;

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .layer(tower::ServiceBuilder::new().layer(TraceLayer::new_for_http()).layer(axum::middleware::from_fn(redirect_www)))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
