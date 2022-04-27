use tiny_http::Request;

pub fn get_content_type(request: &'_ Request) -> &'_ str {
    let content_types: Vec<&str> = request.headers().iter()
        .filter(|header| {
            header.field.as_str().to_ascii_lowercase() == "content-type"
        })
        .map(|header| header.value.as_str())
        .collect();

    content_types.get(0).unwrap_or(&"")
}