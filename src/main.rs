use fedimint_core::invite_code::InviteCode;
use leptos::prelude::*;
use std::str::FromStr;

fn main() {
    leptos::mount::mount_to_body(App)
}

#[derive(Clone)]
struct DecodedInviteCode {
    federation_id: String,
    primary_peer_id: String,
    primary_api_url: String,
    guardian_count: usize,
    has_api_secret: bool,
    normalized_code: String,
    peers: Vec<DecodedPeer>,
}

#[derive(Clone)]
struct DecodedPeer {
    peer_id: String,
    api_url: String,
}

#[derive(Clone)]
enum DecodeResult {
    InviteCode(DecodedInviteCode),
    Error(String),
}

#[component]
fn App() -> impl IntoView {
    let (input, set_input) = signal(String::new());
    let (result, set_result) = signal(None::<DecodeResult>);

    let decode = move |_| {
        let trimmed = input.get().trim().to_owned();

        if trimmed.is_empty() {
            set_result.set(Some(DecodeResult::Error(
                "Enter a Fedimint join code or token first.".to_owned(),
            )));
            return;
        }

        set_result.set(Some(match decode_invite_code(&trimmed) {
            Ok(invite_code) => DecodeResult::InviteCode(invite_code),
            Err(error) => DecodeResult::Error(error),
        }));
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
                        {move || {
                            result
                                .get()
                                .map(|result| {
                                    match result {
                                        DecodeResult::InviteCode(invite_code) => {
                                            view! { <InviteCodeResult invite_code=invite_code /> }.into_any()
                                        }
                                        DecodeResult::Error(message) => {
                                            view! { <p class="error-message">{message}</p> }.into_any()
                                        }
                                    }
                                })
                        }}
                    </section>
                </Show>
            </section>
        </main>
    }
}

#[component]
fn InviteCodeResult(invite_code: DecodedInviteCode) -> impl IntoView {
    let peers = invite_code.peers.clone();

    view! {
        <div class="summary-grid">
            <div class="summary-item">
                <span class="summary-label">"Type"</span>
                <strong>"Federation join code"</strong>
            </div>
            <div class="summary-item">
                <span class="summary-label">"Federation ID"</span>
                <strong class="mono">{invite_code.federation_id.clone()}</strong>
            </div>
            <div class="summary-item">
                <span class="summary-label">"Primary peer"</span>
                <strong>{invite_code.primary_peer_id.clone()}</strong>
            </div>
            <div class="summary-item">
                <span class="summary-label">"Primary API URL"</span>
                <strong class="mono">{invite_code.primary_api_url.clone()}</strong>
            </div>
            <div class="summary-item">
                <span class="summary-label">"Guardian APIs"</span>
                <strong>{invite_code.guardian_count}</strong>
            </div>
            <div class="summary-item">
                <span class="summary-label">"API secret"</span>
                <strong>{if invite_code.has_api_secret { "Present" } else { "Not included" }}</strong>
            </div>
        </div>

        <div class="detail-block">
            <h3>"Guardian endpoints"</h3>
            <div class="peer-list">
                <For
                    each=move || peers.clone()
                    key=|peer| peer.peer_id.clone()
                    children=move |peer| {
                        view! {
                            <div class="peer-row">
                                <span>"Peer " {peer.peer_id}</span>
                                <code>{peer.api_url}</code>
                            </div>
                        }
                    }
                />
            </div>
        </div>

        <div class="detail-block">
            <h3>"Normalized code"</h3>
            <code class="raw-code">{invite_code.normalized_code}</code>
        </div>
    }
}

fn decode_invite_code(input: &str) -> Result<DecodedInviteCode, String> {
    let invite_code =
        InviteCode::from_str(input).map_err(|error| format!("Could not decode join code: {error}"))?;

    let peers = invite_code
        .peers()
        .into_iter()
        .map(|(peer_id, api_url)| DecodedPeer {
            peer_id: peer_id.to_string(),
            api_url: api_url.to_string(),
        })
        .collect::<Vec<_>>();

    Ok(DecodedInviteCode {
        federation_id: invite_code.federation_id().to_string(),
        primary_peer_id: invite_code.peer().to_string(),
        primary_api_url: invite_code.url().to_string(),
        guardian_count: peers.len(),
        has_api_secret: invite_code.api_secret().is_some(),
        normalized_code: invite_code.to_string(),
        peers,
    })
}

