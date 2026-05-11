use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (input, set_input) = signal(String::new());
    let (result, set_result) = signal(None::<String>);

    let decode = move |_| {
        let trimmed = input.get().trim().to_owned();

        if trimmed.is_empty() {
            set_result.set(Some("Enter a Fedimint join code or token first.".to_owned()));
            return;
        }

        set_result.set(Some(format!(
            "Decoding is ready. Input detected: {} characters.",
            trimmed.len()
        )));
    };

    view! {
        <main class="app-shell">
            <section class="decoder-panel" aria-labelledby="decoder-title">
                <div class="intro">
                    <p class="eyebrow">"Fedimint Decoder"</p>
                    <h1 id="decoder-title">"Inspect join codes and tokens"</h1>
                    <p class="lede">
                        "Paste a Fedimint join code or OOB token. Decoding will run fully locally in your browser."
                    </p>
                </div>

                <label class="input-label" for="decoder-input">
                    "Input"
                </label>
                <textarea
                    id="decoder-input"
                    class="decoder-input"
                    rows="10"
                    placeholder="Paste fed1... or an OOB token"
                    prop:value=input
                    on:input=move |event| {
                        set_input.set(event_target_value(&event));
                    }
                ></textarea>

                <div class="actions">
                    <button class="decode-button" type="button" on:click=decode>
                        "Decode"
                    </button>
                </div>

                <Show when=move || result.get().is_some()>
                    <section class="result-panel" aria-live="polite">
                        <h2>"Result"</h2>
                        <p>{move || result.get().unwrap_or_default()}</p>
                    </section>
                </Show>
            </section>
        </main>
    }
}
