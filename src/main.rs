use std::process::exit;

use convert_case::{Case, Casing};
use sniffer_rs::sniffer::Sniffer;
use tigris_rs::features::{
    actions::{CopyTextAction, ResultAction},
    api::{get_extension_request, send_search_results},
    search_results::SearchResult,
};

fn main() {
    let request = get_extension_request().get_results_request.unwrap();
    let search_text = &request.search_text;

    if search_text.is_empty() {
        send_search_results(&vec![]);
        exit(0);
    }

    let sniffer = Sniffer::new();

    let results = emojis::iter()
        .filter(|emoji| sniffer.matches(emoji.name(), search_text))
        .map(|emoji| {
            SearchResult::new(&format!(
                "{}    {}",
                &emoji,
                &emoji.name().to_case(Case::Title)
            ))
            .set_action(&ResultAction::new_copy_text_action(&CopyTextAction::new(
                emoji.as_str(),
            )))
        })
        .collect::<Vec<SearchResult>>();

    send_search_results(&results);
    exit(0);
}
