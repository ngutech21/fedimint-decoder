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
    total_amount_sats: String,
    total_amount_btc: String,
    federation_id_prefix: String,
    note_count: usize,
    tier_count: usize,
    normalized_token: String,
    invite_code: Option<DecodedInviteCode>,
    denominations: Vec<DecodedDenomination>,
    notes: Vec<DecodedNote>,
}

#[derive(Clone)]
struct DecodedDenomination {
    amount: String,
    count: usize,
    subtotal: String,
}

#[derive(Clone)]
struct DecodedNote {
    index: usize,
    amount: String,
    amount_sats: String,
    nonce: String,
}

#[derive(Clone)]
enum DecodeResult {
    InviteCode(DecodedInviteCode),
    OobToken(DecodedOobToken),
    Error(DecodeError),
}

#[derive(Clone)]
struct DecodeError {
    summary: String,
    invite_code_error: Option<String>,
    oob_token_error: Option<String>,
}

#[component]
fn App() -> impl IntoView {
    let (input, set_input) = signal(String::new());
    let (result, set_result) = signal(None::<DecodeResult>);

    let clear = move |_| {
        set_input.set(String::new());
        set_result.set(None);
    };

    let decode = move |_| {
        let trimmed = input.get().trim().to_owned();

        if trimmed.is_empty() {
            set_result.set(Some(DecodeResult::Error(DecodeError {
                summary: "Enter a Fedimint invite code or OOB token first.".to_owned(),
                invite_code_error: None,
                oob_token_error: None,
            })));
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
                    <div class="intro-meta">
                        <p class="eyebrow">"Fedimint Decoder"</p>
                        <span class="privacy-badge">"Local only"</span>
                    </div>
                    <div class="intro-copy">
                        <h1 id="decoder-title">"Inspect invite codes and tokens"</h1>
                        <p class="lede">
                            "Paste a Fedimint invite code or OOB token. Decoding runs in your browser and does not contact a federation."
                        </p>
                    </div>
                </div>

                <div class="input-panel">

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
                        <button class="clear-button" type="button" on:click=clear>
                            "Clear"
                        </button>
                        <button class="decode-button" type="button" on:click=decode>
                            "Decode"
                        </button>
                    </div>
                </div>

                <Show when=move || result.get().is_some()>
                    <section class="result-panel" aria-live="polite">
                        <div class="section-heading result-heading">
                            <h2>"Result"</h2>
                            <span>"Offline structure only"</span>
                        </div>
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
                                        DecodeResult::Error(error) => {
                                            view! { <DecodeErrorResult error=error /> }.into_any()
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
    let raw_note = if invite_code.has_api_secret {
        "This invite code includes an API secret. Reveal it only when you intend to inspect or copy the full code."
    } else {
        "Reveal the canonical invite code string used by Fedimint's parser."
    };

    view! {
        <div class="summary-grid invite-summary">
            <div class="summary-item">
                <span class="summary-label">"Type"</span>
                <strong>"Federation invite code"</strong>
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
            <For
                each=move || peers.clone()
                key=|peer| peer.peer_id.clone()
                children=move |peer| {
                    view! {
                        <div class="summary-item">
                            <span class="summary-label">"Peer " {peer.peer_id} " endpoint"</span>
                            <strong class="mono">{peer.api_url}</strong>
                        </div>
                    }
                }
            />
        </div>

        <RawDisclosure
            title="Normalized invite code"
            note=raw_note
            value=invite_code.normalized_code
        />
    }
}

#[component]
fn OobTokenResult(oob_token: DecodedOobToken) -> impl IntoView {
    let denominations = oob_token.denominations.clone();
    let notes = oob_token.notes.clone();
    let note_count = oob_token.note_count;
    let invite_code = oob_token.invite_code.clone();

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
                <span class="summary-label">"Total sats"</span>
                <strong class="mono">{oob_token.total_amount_sats.clone()}</strong>
            </div>
            <div class="summary-item">
                <span class="summary-label">"Total BTC"</span>
                <strong class="mono">{oob_token.total_amount_btc.clone()}</strong>
            </div>
            <div class="summary-item">
                <span class="summary-label">"Notes"</span>
                <strong>{oob_token.note_count}</strong>
            </div>
            <div class="summary-item">
                <span class="summary-label">"Denomination tiers"</span>
                <strong>{oob_token.tier_count}</strong>
            </div>
            <FederationSummary
                federation_id_prefix=oob_token.federation_id_prefix.clone()
                invite_code=invite_code
            />
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
                    key=|denomination| denomination.amount.clone()
                    children=move |denomination| {
                        view! {
                            <div class="denomination-row">
                                <code>{denomination.amount}</code>
                                <strong>{denomination.count}</strong>
                                <code>{denomination.subtotal}</code>
                            </div>
                        }
                    }
                />
            </div>
        </div>

        <NoteInventory notes=notes note_count=note_count />

        <RawDisclosure
            title="Raw normalized token"
            note="This token may contain spendable ecash data. Reveal it only when you intentionally need the raw representation."
            value=oob_token.normalized_token
        />
    }
}

#[component]
fn NoteInventory(notes: Vec<DecodedNote>, note_count: usize) -> impl IntoView {
    view! {
        <details class="note-inventory">
            <summary>"Show individual notes (" {note_count} ")"</summary>
            <p>"Shows denomination and public note nonce only. Spend keys and raw spendable note data are not shown here."</p>
            <div class="note-list">
                <div class="note-row note-header">
                    <span>"#"</span>
                    <span>"Amount"</span>
                    <span>"Sats"</span>
                    <span>"Public nonce"</span>
                </div>
                <For
                    each=move || notes.clone()
                    key=|note| note.index
                    children=move |note| {
                        view! {
                            <div class="note-row">
                                <strong>{note.index}</strong>
                                <code>{note.amount}</code>
                                <code>{note.amount_sats}</code>
                                <code>{note.nonce}</code>
                            </div>
                        }
                    }
                />
            </div>
        </details>
    }
}

#[component]
fn FederationSummary(
    federation_id_prefix: String,
    invite_code: Option<DecodedInviteCode>,
) -> impl IntoView {
    match invite_code {
        Some(invite_code) => {
            let peers = invite_code.peers.clone();
            let federation_id = invite_code.federation_id.clone();
            let observer_url = format!("https://observer.fedimint.org/federations/{federation_id}");
            let primary_peer_id = invite_code.primary_peer_id.clone();
            let primary_api_url = invite_code.primary_api_url.clone();
            let guardian_count = invite_code.guardian_count;
            let has_api_secret = invite_code.has_api_secret;
            let normalized_code = invite_code.normalized_code.clone();

            view! {
                <div class="summary-item federation-summary">
                    <span class="summary-label">"Federation"</span>
                    <div class="federation-value">
                        <strong class="mono">{federation_id}</strong>
                        <a class="observer-link" href=observer_url target="_blank" rel="noreferrer">
                            "Open in Fedimint Observer"
                        </a>
                        <details class="inline-details">
                            <summary>"Show federation details"</summary>
                            <div class="inline-detail-grid">
                                <div>
                                    <span class="summary-label">"Primary peer"</span>
                                    <strong>{primary_peer_id}</strong>
                                </div>
                                <div>
                                    <span class="summary-label">"Primary API URL"</span>
                                    <strong class="mono">{primary_api_url}</strong>
                                </div>
                                <div>
                                    <span class="summary-label">"Guardian APIs"</span>
                                    <strong>{guardian_count}</strong>
                                </div>
                                <div>
                                    <span class="summary-label">"API secret"</span>
                                    <strong>{if has_api_secret { "Present" } else { "Not included" }}</strong>
                                </div>
                                <For
                                    each=move || peers.clone()
                                    key=|peer| peer.peer_id.clone()
                                    children=move |peer| {
                                        view! {
                                            <div class="inline-detail-wide">
                                                <span class="summary-label">"Peer " {peer.peer_id} " endpoint"</span>
                                                <strong class="mono">{peer.api_url}</strong>
                                            </div>
                                        }
                                    }
                                />
                            </div>
                            <RawDisclosure
                                title="Normalized invite code"
                                note="This embedded invite code may include federation connection details. Reveal it only when you intentionally need the raw representation."
                                value=normalized_code
                            />
                        </details>
                    </div>
                </div>
            }
            .into_any()
        }
        None => view! {
            <div class="summary-item">
                <span class="summary-label">"Federation"</span>
                <div class="federation-value">
                    <strong class="mono">{federation_id_prefix}</strong>
                    <span class="subtle-note">"ID prefix only, no embedded invite"</span>
                </div>
            </div>
        }
        .into_any(),
    }
}

#[component]
fn RawDisclosure(title: &'static str, note: &'static str, value: String) -> impl IntoView {
    view! {
        <details class="raw-disclosure">
            <summary>{title}</summary>
            <p>{note}</p>
            <code class="raw-code">{value}</code>
        </details>
    }
}

#[component]
fn DecodeErrorResult(error: DecodeError) -> impl IntoView {
    let summary = error.summary;
    let invite_code_error = error.invite_code_error.unwrap_or_default();
    let oob_token_error = error.oob_token_error.unwrap_or_default();
    let has_invite_code_error = !invite_code_error.is_empty();
    let has_oob_token_error = !oob_token_error.is_empty();
    let has_parser_details = has_invite_code_error || has_oob_token_error;
    let parser_details = if has_parser_details {
        view! {
            <div class="error-grid">
                {if has_invite_code_error {
                    view! {
                        <div class="error-detail">
                            <span class="summary-label">"Invite code parser"</span>
                            <code>{invite_code_error}</code>
                        </div>
                    }
                    .into_any()
                } else {
                    view! {}.into_any()
                }}
                {if has_oob_token_error {
                    view! {
                        <div class="error-detail">
                            <span class="summary-label">"OOB token parser"</span>
                            <code>{oob_token_error}</code>
                        </div>
                    }
                    .into_any()
                } else {
                    view! {}.into_any()
                }}
            </div>
        }
        .into_any()
    } else {
        view! {}.into_any()
    };

    view! {
        <div class="error-panel" role="alert">
            <p class="error-message">{summary}</p>
            {parser_details}
        </div>
    }
}

fn decode_input(input: &str) -> Result<DecodeResult, DecodeError> {
    match decode_invite_code(input) {
        Ok(invite_code) => return Ok(DecodeResult::InviteCode(invite_code)),
        Err(invite_error) => match decode_oob_token(input) {
            Ok(oob_token) => Ok(DecodeResult::OobToken(oob_token)),
            Err(oob_error) => Err(DecodeError {
                summary: "Could not decode input as a Fedimint invite code or OOB token."
                    .to_owned(),
                invite_code_error: Some(invite_error),
                oob_token_error: Some(oob_error),
            }),
        },
    }
}

fn decode_invite_code(input: &str) -> Result<DecodedInviteCode, String> {
    let invite_code = InviteCode::from_str(input)
        .map_err(|error| format!("Could not decode invite code: {error}"))?;

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
                count,
                subtotal: subtotal.to_string(),
            }
        })
        .collect::<Vec<_>>();

    let notes = oob_notes
        .notes()
        .iter_items()
        .enumerate()
        .map(|(index, (amount, note))| DecodedNote {
            index: index + 1,
            amount: amount.to_string(),
            amount_sats: format_msats_as_sats(amount.msats),
            nonce: note.nonce().to_string(),
        })
        .collect::<Vec<_>>();

    let invite_code = oob_notes
        .federation_invite()
        .map(decoded_invite_code)
        .transpose()?;

    let total_amount = oob_notes.total_amount();

    Ok(DecodedOobToken {
        total_amount: total_amount.to_string(),
        total_amount_sats: format_msats_as_sats(total_amount.msats),
        total_amount_btc: format_msats_as_btc(total_amount.msats),
        federation_id_prefix: oob_notes.federation_id_prefix().to_string(),
        note_count: oob_notes.notes().count_items(),
        tier_count: oob_notes.notes().count_tiers(),
        normalized_token: oob_notes.to_string(),
        invite_code,
        denominations,
        notes,
    })
}

fn format_msats_as_sats(msats: u64) -> String {
    let whole_sats = msats / 1_000;
    let fractional_msats = msats % 1_000;

    if fractional_msats == 0 {
        whole_sats.to_string()
    } else {
        let fractional = format!("{fractional_msats:03}");
        format!("{whole_sats}.{}", fractional.trim_end_matches('0'))
    }
}

fn format_msats_as_btc(msats: u64) -> String {
    const MSATS_PER_BTC: u64 = 100_000_000_000;

    let whole_btc = msats / MSATS_PER_BTC;
    let fractional_msats = msats % MSATS_PER_BTC;

    if fractional_msats == 0 {
        whole_btc.to_string()
    } else {
        let fractional = format!("{fractional_msats:011}");
        format!("{whole_btc}.{}", fractional.trim_end_matches('0'))
    }
}
