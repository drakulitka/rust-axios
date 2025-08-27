
#[derive(Clone)]
pub struct  InterceptorManager {
    pub request_handlers: Vec<RequestHandler>,
    pub response_handlers: Vec<ResponseHandler>,
}
impl InterceptorManager {
    pub fn new() -> Self {
        InterceptorManager {
            request_handlers: vec![],
            response_handlers: vec![],
        }
    }

    pub fn request(&mut self, fulfilled: impl FnMut(reqwest::Request) -> reqwest::Request) {
        self.request_handlers.push(RequestHandler {
            fulfilled,
        });
    }

    pub fn response(&mut self, fulfilled: impl FnMut(reqwest::Response) -> reqwest::Response) {
        self.response_handlers.push(ResponseHandler {
            fulfilled,
        });
    }
}

#[derive(Clone)]
pub struct RequestHandler {
    fulfilled: dyn FnMut(reqwest::Request) -> reqwest::Request,
}
impl RequestHandler {
    pub async fn execute(&self, req: reqwest::Request) -> reqwest::Request {
        (self.fulfilled)(req)
    }
}

#[derive(Clone)]
pub struct ResponseHandler {
    fulfilled: dyn FnMut(reqwest::Response) -> reqwest::Response,
}
impl ResponseHandler {
    pub async fn execute(&self, res: reqwest::Response) -> reqwest::Response {
        (self.fulfilled)(res)
    }
}


