pub fn minify_html(html: &String) -> String {
    let cfg = &minify_html_onepass::Cfg {
        minify_css: false,
        minify_js: false,
    };

    match minify_html_onepass::copy(html.as_bytes(), cfg) {
        Ok(minified) => core::str::from_utf8(minified.as_slice())
            .unwrap()
            .to_string(),
        Err(minify_html_onepass::Error {
            error_type,
            position,
        }) => {
            panic!("Failed to minify HTML: {:?} at {}", error_type, position);
        }
    }
}
