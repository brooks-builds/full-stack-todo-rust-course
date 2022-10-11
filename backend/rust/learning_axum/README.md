# Next Steps

use the async trait macro from https://docs.rs/axum/latest/axum/extract/trait.FromRequest.html so that we can have our extractor cause our eyes to bleed less.


```
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
};

struct MyExtractor;

#[async_trait]
impl<B> FromRequest<B> for MyExtractor
where
    B: Send, // required by `async_trait`
{
    type Rejection = http::StatusCode;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // ...
    }
}
```

