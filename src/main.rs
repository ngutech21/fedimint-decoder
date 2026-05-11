use fedimint_core::invite_code::InviteCode;
use fedimint_mint_client::OOBNotes;
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
struct DecodedOobToken {
    total_amount: String,
    total_amount_msats: u64,
    federation_id_prefix: String,
    note_count: usize,
    tier_count: usize,
    normalized_token: String,
    invite_code: Option<DecodedInviteCode>,
    denominations: Vec<DecodedDenomination>,
}

#[derive(Clone)]
struct DecodedDenomination {
    amount: String,
    amount_msats: u64,
    count: usize,
    subtotal: String,
    subtotal_msats: u64,
}

#[derive(Clone)]
enum DecodeResult {
    InviteCode(DecodedInviteCode),
    OobToken(DecodedOobToken),
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

        set_result.set(Some(match decode_input(&trimmed) {
            Ok(result) => result,
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
                                        DecodeResult::OobToken(oob_token) => {
                                            view! { <OobTokenResult oob_token=oob_token /> }.into_any()
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

#[component]
fn OobTokenResult(oob_token: DecodedOobToken) -> impl IntoView {
    let denominations = oob_token.denominations.clone();

    view! {
        <div class="summary-grid">
            <div class="summary-item">
                <span class="summary-label">"Type"</span>
                <strong>"OOB eCash token"</strong>
            </div>
            <div class="summary-item">
                <span class="summary-label">"Total amount"</span>
                <strong>{oob_token.total_amount.clone()}</strong>
            </div>
            <div class="summary-item">
                <span class="summary-label">"Total msats"</span>
                <strong>{oob_token.total_amount_msats}</strong>
            </div>
            <div class="summary-item">
                <span class="summary-label">"Federation ID prefix"</span>
                <strong class="mono">{oob_token.federation_id_prefix.clone()}</strong>
            </div>
            <div class="summary-item">
                <span class="summary-label">"Notes"</span>
                <strong>{oob_token.note_count}</strong>
            </div>
            <div class="summary-item">
                <span class="summary-label">"Denomination tiers"</span>
                <strong>{oob_token.tier_count}</strong>
            </div>
        </div>

        <div class="detail-block">
            <h3>"Denominations"</h3>
            <div class="denomination-list">
                <div class="denomination-row denomination-header">
                    <span>"Amount"</span>
                    <span>"Count"</span>
                    <span>"Subtotal"</span>
                </div>
                <For
                    each=move || denominations.clone()
                    key=|denomination| denomination.amount_msats
                    children=move |denomination| {
                        view! {
                            <div class="denomination-row">
                                <code>{denomination.amount} " (" {denomination.amount_msats} " msat)"</code>
                                <strong>{denomination.count}</strong>
                                <code>{denomination.subtotal} " (" {denomination.subtotal_msats} " msat)"</code>
                            </div>
                        }
                    }
                />
            </div>
        </div>

        <EmbeddedInviteCode invite_code=oob_token.invite_code.clone() />

        <div class="detail-block">
            <h3>"Normalized token"</h3>
            <code class="raw-code">{oob_token.normalized_token}</code>
        </div>
    }
}

#[component]
fn EmbeddedInviteCode(invite_code: Option<DecodedInviteCode>) -> impl IntoView {
    match invite_code {
        Some(invite_code) => view! {
            <div class="detail-block">
                <h3>"Embedded federation invite"</h3>
                <InviteCodeResult invite_code=invite_code />
            </div>
        }
        .into_any(),
        None => view! {}.into_any(),
    }
}

fn decode_input(input: &str) -> Result<DecodeResult, String> {
    match decode_invite_code(input) {
        Ok(invite_code) => return Ok(DecodeResult::InviteCode(invite_code)),
        Err(invite_error) => match decode_oob_token(input) {
            Ok(oob_token) => Ok(DecodeResult::OobToken(oob_token)),
            Err(oob_error) => Err(format!(
                "Could not decode input as a Fedimint join code or OOB token. Join code error: {invite_error}. OOB token error: {oob_error}"
            )),
        },
    }
}

fn decode_invite_code(input: &str) -> Result<DecodedInviteCode, String> {
    let invite_code = InviteCode::from_str(input)
        .map_err(|error| format!("Could not decode join code: {error}"))?;

    decoded_invite_code(invite_code)
}

fn decoded_invite_code(invite_code: InviteCode) -> Result<DecodedInviteCode, String> {
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

fn decode_oob_token(input: &str) -> Result<DecodedOobToken, String> {
    let oob_notes = OOBNotes::from_str(input)
        .map_err(|error| format!("Could not decode OOB token: {error}"))?;

    let denominations = oob_notes
        .notes()
        .summary()
        .iter()
        .map(|(amount, count)| {
            let subtotal = amount * (count as u64);

            DecodedDenomination {
                amount: amount.to_string(),
                amount_msats: amount.msats,
                count,
                subtotal: subtotal.to_string(),
                subtotal_msats: subtotal.msats,
            }
        })
        .collect::<Vec<_>>();

    let invite_code = oob_notes
        .federation_invite()
        .map(decoded_invite_code)
        .transpose()?;

    let total_amount = oob_notes.total_amount();

    Ok(DecodedOobToken {
        total_amount: total_amount.to_string(),
        total_amount_msats: total_amount.msats,
        federation_id_prefix: oob_notes.federation_id_prefix().to_string(),
        note_count: oob_notes.notes().count_items(),
        tier_count: oob_notes.notes().count_tiers(),
        normalized_token: oob_notes.to_string(),
        invite_code,
        denominations,
    })
}
