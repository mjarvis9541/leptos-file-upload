use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::error_template::{AppError, ErrorTemplate};
use crate::page::file_upload_page::FileUploadPage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-file-upload.css"/>
        <Title text="File Upload Example"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <nav class="flex">
                <a class="px-4 py-2" href="/">
                    "Home"
                </a>
                <a class="px-4 py-2" href="/file-upload">
                    "File Upload"
                </a>
            </nav>
            <Routes>
                <Route path="" view=HomePage/>
                <Route path="/file-upload" view=FileUploadPage/>
            </Routes>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Title text="Home"/>
        <main class="mx-auto border p-4 m-4 max-w-lg">
            <h1>"Home"</h1>
        </main>
    }
}
