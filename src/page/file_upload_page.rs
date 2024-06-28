use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use leptos::*;
use leptos_meta::Title;
use server_fn::codec::{MultipartData, MultipartFormData};

use chrono::Utc;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};

#[server(input = MultipartFormData)]
pub async fn upload_file_action(data: MultipartData) -> Result<(), ServerFnError> {
    let mut data = data.into_inner().unwrap();

    let current_dir = env::current_dir().map_err(|e| ServerFnError::new(e.to_string()))?;
    leptos::logging::log!("{:?}", &current_dir);

    let upload_dir = current_dir.join("public/static/images");
    if !upload_dir.exists() {
        std::fs::create_dir_all(upload_dir.clone())
            .map_err(|e| ServerFnError::new(e.to_string()))?;
    }

    while let Ok(Some(mut field)) = data.next_field().await {
        let file_name = field.file_name().unwrap_or_default();

        let file_extension = Path::new(&file_name)
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        let unique_id = Uuid::new_v4();
        let timestamp = Utc::now().format("%Y%m%dT%H%M%S").to_string();

        let new_file_name = format!("upload_{}_{}.{}", unique_id, timestamp, file_extension);

        let file_path = upload_dir.join(&new_file_name);
        let mut file = File::create(&file_path).map_err(|e| ServerFnError::new(e.to_string()))?;

        while let Ok(Some(chunk)) = field.chunk().await {
            file.write_all(&chunk)
                .map_err(|e| ServerFnError::new(e.to_string()))?;
        }
    }

    Ok(())
}

#[component]
pub fn FileUploadPage() -> impl IntoView {
    let upload_action = create_action(|data: &FormData| {
        let data = data.clone();
        // `MultipartData` implements `From<FormData>`
        upload_file_action(data.into())
    });

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
        leptos::logging::log!("target: {:?}", &target);
        let form_data = FormData::new_with_form(&target).unwrap();
        leptos::logging::log!("form_data: {:?}", &form_data);
        upload_action.dispatch(form_data);
    };

    view! {
        <Title text="File Upload"/>
        <main class="mx-auto border p-4 m-4 max-w-lg">
            <h1 class="mb-4">"File Upload"</h1>

            <form on:submit=on_submit class="space-y-4">
                <input
                    name="file_to_upload"
                    type="file"
                    class="w-full truncate text-sm border border-gray-300 cursor-pointer focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                />
                <button>"Submit"</button>
            </form>
        </main>
    }
}
