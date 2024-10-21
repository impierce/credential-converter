use axum::{
    response::Html,
    // routing::post,
    // Router,
};

pub async fn root() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head><title>Open Badges to ELM converter</title></head>
            <body>
                <h1>Open Badges to ELM converter</h1>
                <p>
                    This from is part of a service to translate Open Badges to ELM European Digital Credentials.</br>
                    The code as two parts a webservice that is called through this form and a CLI.</br>
                    The CLI can be found at the <a href="http://github.com/educredentials/">Educredentials github respository</a> 
                </p>
                <form action="/translate_file" method="post" enctype="multipart/form-data">
                    <label>
                        File to translate:
                        <input type="file" name="input_file" multiple>
                    </label>
                    <p>
                    <label>
                        type of translation:
                            <select id="translation" name="translation">
                            <option value="">--Please choose an option--</option>
                            <option value="OBv3ToELM">OBv3ToELM</option>
                            <option value="ELMToOBv3">ELMToOBv3</option>
                            </select>
                    </label>
                    <p>
                    <input type="submit" value="Translate this file">
                </form>
            </body>
        </html>
        "#,
    )
}
