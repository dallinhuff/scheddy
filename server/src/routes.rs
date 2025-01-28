pub fn routes() -> axum::Router {
    let api_routes = axum::Router::new()
        .nest("/bookings", bookings::routes())
        .nest("/waivers", waivers::routes());

    axum::Router::new().nest("/api", api_routes)
}

mod bookings {
    use axum::routing;

    pub fn routes() -> axum::Router {
        axum::Router::new()
            .route(
                "/",
                routing::get(|| async { "Get a list of bookings that match search criteria" })
                    .post(|| async { "Create a new booking" }),
            )
            .route(
                "/b/{id}",
                routing::get(|| async { "Get a booking by id" })
                    .patch(|| async { "Update a booking" })
                    .delete(|| async { "Cancel a booking" }),
            )
            .route(
                "/rentals/{id}",
                routing::get(|| async { "Get a list of equipment rentals for a booking" })
                    .patch(|| async { "Update a booking's rentals" })
                    .delete(|| async { "Cancel one or more rentals on a booking" }),
            )
    }
}

mod waivers {
    use axum::routing;

    pub fn routes() -> axum::Router {
        axum::Router::new()
            .route(
                "/",
                routing::get(|| async { "Get a list of waivers that match search criteria" })
                    .post(|| async { "Create a waiver" }),
            )
            .route(
                "/w/{id}",
                routing::get(|| async { "View a waiver by id" })
                    .patch(|| async { "Update a waiver's content" })
                    .delete(|| async { "Delete a waiver" }),
            )
            .route(
                "/sign/{id}",
                routing::post(|| async { "Sign a waiver by id" }),
            )
    }
}
