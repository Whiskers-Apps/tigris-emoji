use convert_case::{Case, Casing};
use sniffer_rs::sniffer::Sniffer;
use tigris_core::features::{
    actions::{CopyTextAction, ResultAction},
    api::{get_request, return_search_results},
    search_results::SearchResult,
};

fn main() {
    let request = get_request().unwrap().get_results_request.unwrap();
    let search_text = &request.search_text;

    if search_text.is_empty() {
        return_search_results(&vec![]);
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

    return_search_results(&results);
}
