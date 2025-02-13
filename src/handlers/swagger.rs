use spin_sdk::http::{IntoResponse, Params, Request, Response};

pub(crate) fn swagger_handler(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let ui = r##"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Swagger UI</title>
  <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/swagger-ui/5.18.2/swagger-ui.min.css">
  <script src="https://cdnjs.cloudflare.com/ajax/libs/swagger-ui/5.18.2/swagger-ui-bundle.min.js"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/swagger-ui/5.18.2/swagger-ui-standalone-preset.min.js"></script>
</head>
<body>
  <div id="app"></div>
  <script>
    window.onload = function() {
      const ui = SwaggerUIBundle({
        url: "/openapi.json",
        dom_id: "#app",
        presets: [
          SwaggerUIBundle.presets.apis,
          SwaggerUIStandalonePreset
        ],
        layout: "StandaloneLayout"
      });
      window.ui = ui;
    };
  </script>
</body>
</html>
"##;
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .header("Access-Control-Allow-Origin", "*")
        .body(ui)
        .build())
}
