use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next};

pub struct AuthMiddleware;

#[async_trait::async_trait]
impl Middleware for AuthMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut http::Extensions,
        next: Next<'_>
    ) -> reqwest_middleware::Result<Response> {
        
        next.run(req, extensions).await
        
    }
}