//! Shared domain model and chain-finding logic for the SpacerFinder tools.
//!
//! Keeping this logic in one crate makes the CLI and both GUIs agree on thread
//! compatibility, validation rules, and the chains that are reported to users.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Maximum number of elements in a chain (keeps the backtracking search bounded).
pub const MAX_CHAIN_LENGTH: usize = 10;
const EPSILON: f64 = 1e-6;

/// A single spacer or required ("rig") element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Element {
    pub thread_in: String,
    pub thread_out: String,
    pub length: f64,
}

impl Element {
    pub fn new(thread_in: impl Into<String>, thread_out: impl Into<String>, length: f64) -> Self {
        Self {
            thread_in: thread_in.into(),
            thread_out: thread_out.into(),
            length,
        }
    }

    pub fn matches(&self, other: &Element) -> bool {
        self.thread_in == other.thread_in
            && self.thread_out == other.thread_out
            && (self.length - other.length).abs() < EPSILON
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub fn symbol(self) -> char {
        match self {
            Gender::Male => 'M',
            Gender::Female => 'F',
        }
    }
}

/// A parsed thread specification such as `M54M` (metric size 54, male).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Thread {
    pub size: String,
    pub gender: Gender,
}

impl Thread {
    /// Parse a `M<size><M|F>` thread specification.
    pub fn parse(value: &str) -> Option<Self> {
        let chars: Vec<char> = value.chars().collect();
        if chars.len() < 3 || chars[0] != 'M' {
            return None;
        }
        let gender = match chars[chars.len() - 1] {
            'M' => Gender::Male,
            'F' => Gender::Female,
            _ => return None,
        };
        let digits = &chars[1..chars.len() - 1];
        if digits.iter().any(|c| !c.is_ascii_digit()) {
            return None;
        }
        Some(Self {
            size: digits.iter().collect(),
            gender,
        })
    }

    pub fn render(&self) -> String {
        format!("M{}{}", self.size, self.gender.symbol())
    }
}

pub fn validate_thread(value: &str) -> Result<(), String> {
    if Thread::parse(value).is_some() {
        Ok(())
    } else {
        Err(format!(
            "Use the size+gender format, e.g. M54M (got \"{value}\")."
        ))
    }
}

pub fn validate_length(value: f64) -> Result<(), String> {
    if value.is_finite() && value >= 0.0 {
        Ok(())
    } else {
        Err("Length must be a non-negative number.".to_string())
    }
}

/// Two elements connect when their thread sizes match and genders are opposite.
pub fn can_connect(out_thread: &str, in_thread: &str) -> bool {
    match (Thread::parse(out_thread), Thread::parse(in_thread)) {
        (Some(out), Some(input)) => out.size == input.size && out.gender != input.gender,
        _ => false,
    }
}

pub fn format_length(value: f64) -> String {
    let formatted = format!("{value:.3}");
    let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
    if trimmed.is_empty() || trimmed == "-0" {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}

// --- Pure collection operations -------------------------------------------------

pub fn add_element(elements: &[Element], element: Element) -> Vec<Element> {
    let mut result = elements.to_vec();
    result.push(element);
    result
}

pub fn update_element(elements: &[Element], index: usize, element: Element) -> Vec<Element> {
    elements
        .iter()
        .enumerate()
        .map(|(i, existing)| {
            if i == index {
                element.clone()
            } else {
                existing.clone()
            }
        })
        .collect()
}

pub fn delete_element(elements: &[Element], index: usize) -> Vec<Element> {
    elements
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != index)
        .map(|(_, element)| element.clone())
        .collect()
}

pub fn move_element(elements: &[Element], from: usize, to: usize) -> Vec<Element> {
    if from == to || from >= elements.len() || to >= elements.len() {
        return elements.to_vec();
    }
    let mut result = elements.to_vec();
    let element = result.remove(from);
    result.insert(to, element);
    result
}

pub fn move_up(elements: &[Element], index: usize) -> Vec<Element> {
    if index == 0 || index >= elements.len() {
        elements.to_vec()
    } else {
        move_element(elements, index, index - 1)
    }
}

pub fn move_down(elements: &[Element], index: usize) -> Vec<Element> {
    if index + 1 >= elements.len() {
        elements.to_vec()
    } else {
        move_element(elements, index, index + 1)
    }
}

// --- Persistence ----------------------------------------------------------------

pub fn load_elements(path: impl AsRef<Path>) -> Result<Vec<Element>, String> {
    let path = path.as_ref();
    let data = fs::read_to_string(path)
        .map_err(|error| format!("Could not read '{}': {error}", path.display()))?;
    serde_json::from_str(&data)
        .map_err(|error| format!("Invalid JSON in '{}': {error}", path.display()))
}

pub fn save_elements(path: impl AsRef<Path>, elements: &[Element]) -> Result<(), String> {
    let path = path.as_ref();
    let data = serde_json::to_string_pretty(elements).map_err(|error| error.to_string())?;
    fs::write(path, data).map_err(|error| format!("Could not write '{}': {error}", path.display()))
}

// --- Chain finding --------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct ChainStep {
    pub element: Element,
    pub cumulative_length: f64,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChainResult {
    pub steps: Vec<ChainStep>,
    pub total_length: f64,
}

impl ChainResult {
    pub fn required_count(&self) -> usize {
        self.steps.iter().filter(|step| step.required).count()
    }

    pub fn render(&self) -> String {
        let mut output = format!(
            "Found chain with {} elements (total {} mm):",
            self.steps.len(),
            format_length(self.total_length)
        );
        for (index, step) in self.steps.iter().enumerate() {
            output.push_str(&format!(
                "\n  {}. {} -> {}   {} mm   (cumulative {} mm){}",
                index + 1,
                step.element.thread_in,
                step.element.thread_out,
                format_length(step.element.length),
                format_length(step.cumulative_length),
                if step.required { "  [required]" } else { "" }
            ));
        }
        output
    }
}

#[derive(Clone)]
struct Candidate<'a> {
    element: &'a Element,
    required: bool,
}

/// Find the shortest chain that connects `start` to `end`, sums to `target`,
/// and contains all `must_have` elements in the given order.
///
/// Threads normally connect when sizes match and genders are opposite. To
/// accommodate both common ways of naming chain endpoints, `start` and `end`
/// may also name the thread on the first/last spacer itself (in which case an
/// exact thread match is accepted).
pub fn find_minimal_chain(
    spacers: &[Element],
    must_have: &[Element],
    start: &str,
    end: &str,
    target: f64,
) -> Option<ChainResult> {
    let mut context = SearchContext {
        spacers,
        must_have,
        end,
        target,
        must_index: 0,
        current_sum: 0.0,
        chain: Vec::new(),
        best: None,
    };
    context.search(start);
    context
        .best
        .map(|candidates| build_result(candidates, target))
}

struct SearchContext<'a> {
    spacers: &'a [Element],
    must_have: &'a [Element],
    end: &'a str,
    target: f64,
    must_index: usize,
    current_sum: f64,
    chain: Vec<Candidate<'a>>,
    best: Option<Vec<Candidate<'a>>>,
}

impl SearchContext<'_> {
    fn search(&mut self, current_thread: &str) {
        if self.chain.len() >= MAX_CHAIN_LENGTH {
            return;
        }
        if self
            .best
            .as_ref()
            .is_some_and(|shortest| self.chain.len() >= shortest.len())
        {
            return;
        }

        for spacer in self.spacers {
            // The start may name either the mating thread or the thread on the
            // first spacer; accept both conventions.
            let connects = can_connect(current_thread, &spacer.thread_in)
                || (self.chain.is_empty() && current_thread == spacer.thread_in);
            if !connects {
                continue;
            }
            let new_sum = self.current_sum + spacer.length;
            if new_sum > self.target + EPSILON {
                continue;
            }
            let required = self.must_index < self.must_have.len()
                && spacer.matches(&self.must_have[self.must_index]);
            let next_index = if required {
                self.must_index + 1
            } else {
                self.must_index
            };

            self.chain.push(Candidate {
                element: spacer,
                required,
            });
            // The end may name the mating thread or the last spacer's thread.
            let ends_chain =
                spacer.thread_out == self.end || can_connect(&spacer.thread_out, self.end);
            let is_solution = (new_sum - self.target).abs() < EPSILON
                && ends_chain
                && next_index == self.must_have.len();
            if is_solution
                && self
                    .best
                    .as_ref()
                    .is_none_or(|shortest| self.chain.len() < shortest.len())
            {
                self.best = Some(self.chain.clone());
            }

            let previous_sum = self.current_sum;
            let previous_index = self.must_index;
            self.current_sum = new_sum;
            self.must_index = next_index;
            self.search(&spacer.thread_out);
            self.current_sum = previous_sum;
            self.must_index = previous_index;
            self.chain.pop();
        }
    }
}

fn build_result(candidates: Vec<Candidate<'_>>, target: f64) -> ChainResult {
    let mut cumulative = 0.0;
    let mut steps = Vec::with_capacity(candidates.len());
    for candidate in candidates {
        cumulative += candidate.element.length;
        steps.push(ChainStep {
            element: candidate.element.clone(),
            cumulative_length: cumulative,
            required: candidate.required,
        });
    }
    // Use the exact target for the summary: chains are accepted within EPSILON.
    let total_length = if (cumulative - target).abs() < EPSILON {
        target
    } else {
        cumulative
    };
    ChainResult {
        steps,
        total_length,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn element(in_thread: &str, out_thread: &str, length: f64) -> Element {
        Element::new(in_thread, out_thread, length)
    }

    #[test]
    fn parses_threads() {
        assert_eq!(
            Thread::parse("M54M"),
            Some(Thread {
                size: "54".into(),
                gender: Gender::Male
            })
        );
        assert_eq!(
            Thread::parse("M54F"),
            Some(Thread {
                size: "54".into(),
                gender: Gender::Female
            })
        );
        assert!(Thread::parse("M54").is_none());
        assert!(Thread::parse("F54M").is_none());
        assert!(Thread::parse("M5xM").is_none());
    }

    #[test]
    fn connection_requires_same_size_and_opposite_gender() {
        assert!(can_connect("M54M", "M54F"));
        assert!(can_connect("M54F", "M54M"));
        assert!(!can_connect("M54M", "M54M"));
        assert!(!can_connect("M54M", "M42F"));
        assert!(!can_connect("bad", "M54F"));
    }

    #[test]
    fn validation_rejects_bad_values() {
        assert!(validate_thread("M54F").is_ok());
        assert!(validate_thread("54F").is_err());
        assert!(validate_length(0.0).is_ok());
        assert!(validate_length(-1.0).is_err());
        assert!(validate_length(f64::NAN).is_err());
    }

    #[test]
    fn finds_minimal_chain() {
        let spacers = vec![
            element("M54F", "M54M", 5.0),
            element("M54F", "M54F", 3.0),
            element("M54M", "M54F", 2.0),
        ];
        let result = find_minimal_chain(&spacers, &[], "M54M", "M54F", 8.0).unwrap();
        assert_eq!(result.steps.len(), 2);
        assert_eq!(result.total_length, 8.0);
        assert_eq!(result.steps[0].cumulative_length, 5.0);
        assert_eq!(result.steps[1].cumulative_length, 8.0);
    }

    #[test]
    fn enforces_required_elements() {
        let required = element("M54F", "M54M", 5.0);
        let spacers = vec![
            required.clone(),
            element("M54F", "M54F", 3.0),
            element("M54M", "M54F", 2.0),
            element("M54F", "M54F", 8.0),
        ];
        assert!(find_minimal_chain(
            &spacers,
            std::slice::from_ref(&required),
            "M54M",
            "M54F",
            8.0
        )
        .is_some());
        assert!(find_minimal_chain(
            &spacers,
            &[element("M54F", "M54M", 7.0)],
            "M54M",
            "M54F",
            8.0
        )
        .is_none());
    }

    #[test]
    fn accepts_both_endpoint_conventions() {
        let spacers = vec![element("M42M", "M42F", 2.0)];
        // Opposite-gender endpoints (physical mating threads).
        assert!(find_minimal_chain(&spacers, &[], "M42F", "M42M", 2.0).is_some());
        // Same thread as the spacer's own ends.
        assert!(find_minimal_chain(&spacers, &[], "M42M", "M42F", 2.0).is_some());
        // Mixed conventions.
        assert!(find_minimal_chain(&spacers, &[], "M42M", "M42M", 2.0).is_some());
        assert!(find_minimal_chain(&spacers, &[], "M42F", "M42F", 2.0).is_some());
    }

    #[test]
    fn reordering_is_pure() {
        let elements = vec![element("M54F", "M54M", 1.0), element("M54F", "M54M", 2.0)];
        assert_eq!(move_up(&elements, 1)[0].length, 2.0);
        assert_eq!(move_down(&elements, 0)[1].length, 1.0);
        assert_eq!(delete_element(&elements, 0).len(), 1);
    }
}
