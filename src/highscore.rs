use crate::utils::format_duration;
use crate::utils::log;
use chrono::offset::Local;
use core::cmp::Ordering;
use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;

const STORAGE_KEY: &str = "highscore";
const MAX_ENTRIES: usize = 20;

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord)]
struct HighscoreEntry {
    name: String,
    score: i32,
    lines: i32,
    level: i32,
    #[serde(default = "default_duration")]
    duration: u32,
    time: String,
}

fn default_duration() -> u32 {
    (99 * 60 + 59) * 1000
}

impl PartialOrd for HighscoreEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.score == other.score {
            if self.lines == other.lines {
                if self.level == other.level {
                    if self.duration == other.duration {
                        return Some(other.time.cmp(&self.time));
                    }
                    // for duration the lower value is ranked higher
                    return Some(self.duration.cmp(&other.duration));
                }
                return Some(other.level.cmp(&self.level));
            }
            return Some(other.lines.cmp(&self.lines));
        }
        return Some(other.score.cmp(&self.score));
    }
}

pub fn add_score(name: &str, level: i32, lines: i32, score: i32, duration: u32) -> Option<String> {
    let window = web_sys::window().unwrap();
    let local_storage_opt = window.local_storage().unwrap();
    if local_storage_opt.is_some() {
        let local_storage = local_storage_opt.unwrap();
        let json_result = local_storage.get_item(STORAGE_KEY);
        let mut entries: Vec<HighscoreEntry>;
        if json_result.is_ok() {
            let json_opt = json_result.unwrap();
            if json_opt.is_some() {
                let json = json_opt.unwrap();
                let entries_result = serde_json::from_str(&json);
                if entries_result.is_ok() {
                    entries = entries_result.unwrap();
                } else {
                    entries = Vec::new();
                }
            } else {
                entries = Vec::new();
            }
        } else {
            entries = Vec::new();
        }

        let new_entry = HighscoreEntry {
            name: name.to_string(),
            score: score,
            lines: lines,
            level: level,
            duration: duration,
            time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
        let new_entry_time = new_entry.time.clone();
        entries.push(new_entry);
        entries.sort();
        while entries.len() > MAX_ENTRIES {
            entries.pop();
        }
        let json = serde_json::to_string(&entries).unwrap();
        let result = local_storage.set_item(STORAGE_KEY, &json);
        if result.is_err() {
            log!(
                "could not save highscore to local_storage: {}",
                result.err().unwrap().as_string().unwrap()
            );
        } else {
            return Some(new_entry_time);
        }
    }
    return None;
}

pub fn print_highscores(latest_timestamp: Option<String>) {
    let window = web_sys::window().unwrap();
    let local_storage_opt = window.local_storage().unwrap();
    if local_storage_opt.is_some() {
        let local_storage = local_storage_opt.unwrap();
        let document = window.document().unwrap();
        let table = document
            .get_element_by_id("highscores-table")
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        while table.child_element_count() > 1 {
            table.last_element_child().unwrap().remove();
        }
        let json_result = local_storage.get_item(STORAGE_KEY);
        let entries: Vec<HighscoreEntry>;
        if json_result.is_ok() {
            let json_opt = json_result.unwrap();
            if json_opt.is_some() {
                let json = json_opt.unwrap();
                let entries_result = serde_json::from_str(&json);
                if entries_result.is_ok() {
                    entries = entries_result.unwrap();
                    let mut i = 0;
                    for entry in entries {
                        let result = print_entry(
                            &document,
                            &table,
                            &entry,
                            i + 1,
                            latest_timestamp.is_some()
                                && latest_timestamp.as_ref().unwrap() == &entry.time,
                        );
                        if result.is_err() {
                            log!(
                                "could not crate highscore table elements: {}",
                                result.err().unwrap().as_string().unwrap()
                            );
                            break;
                        }
                        i += 1;
                    }
                }
            }
        }
    }
}
fn print_entry(
    document: &web_sys::Document,
    table: &web_sys::HtmlElement,
    entry: &HighscoreEntry,
    rank: u32,
    highlight: bool,
) -> Result<(), wasm_bindgen::JsValue> {
    let tr = document.create_element("tr").unwrap();
    table.append_child(&tr)?;
    if highlight {
        tr.set_class_name("latest");
    }
    let td_rank = document.create_element("td").unwrap();
    let td_name = document.create_element("td").unwrap();
    let td_score = document.create_element("td").unwrap();
    let td_lines = document.create_element("td").unwrap();
    let td_level = document.create_element("td").unwrap();
    let td_duration = document.create_element("td").unwrap();
    let td_time = document.create_element("td").unwrap();
    tr.append_child(&td_rank)?;
    tr.append_child(&td_name)?;
    tr.append_child(&td_score)?;
    tr.append_child(&td_lines)?;
    tr.append_child(&td_level)?;
    tr.append_child(&td_duration)?;
    tr.append_child(&td_time)?;
    td_rank.set_text_content(Some(&rank.to_string()));
    td_name.set_text_content(Some(&entry.name));
    td_score.set_text_content(Some(&entry.score.to_string()));
    td_lines.set_text_content(Some(&entry.lines.to_string()));
    td_level.set_text_content(Some(&entry.level.to_string()));
    td_duration.set_text_content(Some(&(format_duration(entry.duration)).to_string()));
    td_time.set_text_content(Some(&entry.time));
    Ok(())
}
