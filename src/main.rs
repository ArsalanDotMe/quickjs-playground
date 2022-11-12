use std::collections::HashMap;
use quick_js::{Context, JsValue, console::Level};
use quick_js::JsValue::Undefined;

fn main() {
   let context = Context::builder()
        .console(|level: Level, args: Vec<JsValue>| {
            eprintln!("{}: {:?}", level, args);
        })
        .build().unwrap();

    let user_js = std::fs::read_to_string("./assets/user.js").unwrap();
    let wrapper_js = std::fs::read_to_string("./assets/wrapper.js").unwrap();

    // Add native_fetch to the context so that the user can make network requests
    context
        .add_callback("native_fetch", |_id: i32, url: String, _options: HashMap<String, JsValue>| {
            reqwest::blocking::get(&url)
                .map(|response| {
                    let mut headers = HashMap::new();
                    for (key, value) in response.headers().iter() {
                        headers.insert(key.to_string(),  JsValue::String(value.to_str().unwrap().to_string()));
                    }

                    let mut object = HashMap::new();
                    object.insert("status".to_string(), JsValue::Int(response.status().as_u16() as i32));
                    object.insert("ok".to_string(), JsValue::Bool(response.status().is_success()));
                    object.insert("headers".to_string(), quick_js::JsValue::Object(headers));
                    object.insert("raw_text".to_string(), response.text().map(|text | JsValue::String(text)).unwrap_or(Undefined));
                    JsValue::Object(object)
                })
                .unwrap_or_else(|_| JsValue::Null)
        })
        .unwrap();

    // Add log to the context so that the user can write to the console
    context
        .add_callback("log", |message: String| {
            println!("{}", &message);
            Undefined
        })
        .unwrap();

    // Append the user's code to the wrapper code
    let code = format!("{}\n{}", wrapper_js, user_js);

    context.eval(&code).unwrap();

    let user_response = context.call_function("callUserCode", [JsValue::Object(HashMap::from([
            ("params".to_string(), JsValue::Object(HashMap::from([
                ("id".to_string(), JsValue::Int(173)),
            ]))),
        ]))
    ]).unwrap();

    // parse the response as JSON
    println!("{:?}", user_response.into_string());
}
