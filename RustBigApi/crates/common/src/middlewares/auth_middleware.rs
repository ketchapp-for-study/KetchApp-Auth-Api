use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error};
use std::future::{ready, Ready};
use std::rc::Rc;
use std::task::{Context, Poll};
use chrono::Utc;
use crate::utils::extract_jwt_claims::extract_jwt_claims_from_request;

// AuthMiddleware struct: marker for our authentication middleware
pub struct AuthMiddleware;

// Implementation of the Transform trait for AuthMiddleware
// This allows us to wrap any service with our authentication logic
impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static + actix_web::body::MessageBody,
{
    // The response type after the middleware (always BoxBody for compatibility)
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    // Called when the middleware is created
    fn new_transform(&self, service: S) -> Self::Future {
        // Wrap the service in an Rc for shared ownership
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

// The actual middleware service that will handle requests
pub struct AuthMiddlewareService<S> {
    pub service: Rc<S>,
}

// Implementation of the Service trait for our middleware
impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static + actix_web::body::MessageBody,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + 'static>>;

    // Called to check if the service is ready to accept a request
    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    // Main logic for handling each incoming request
    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        // Try to extract the JWT token from the request using the shared utility function
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET env variable must be set");
        let claims = {
            // Convert ServiceRequest to HttpRequest reference for utility usage
            let http_req = req.request();
            extract_jwt_claims_from_request(http_req, &secret).ok().flatten()
        };
        let service = self.service.clone();
        Box::pin(async move {
            if let Some(claims) = claims {
                // Optionally print or use claims.sub (user UUID) here if needed
                // println!("[AuthMiddleware] user_uuid (sub): {}", claims.sub);
                // Check if the token is not expired (exp claim)
                let now = Utc::now().timestamp() as usize;
                if claims.exp > now {
                    // Token is valid, forward the request to the next service in the chain
                    return service.call(req).await.map(|res| res.map_into_boxed_body());
                }
            }
            // If no valid token, return 401 Unauthorized
            use actix_web::HttpResponse;
            let (req, _pl) = req.into_parts();
            Ok(ServiceResponse::new(req, HttpResponse::Unauthorized().finish()).map_into_boxed_body())
        })
    }
}
