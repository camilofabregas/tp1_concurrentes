use std::collections::HashMap;
use crate::stat::Stat;
use crate::line_parser::LineParser;

use serde::Serialize;

#[derive(Serialize)]
#[derive(Debug)]
pub struct Site {
    questions: i32,
    words: i32,
    tags: HashMap<String, Stat>,
    chatty_tags: Vec<String>
}

impl Site {
    // Create new empty Site.
    pub fn new() -> Self {
        Self { questions: 0, words: 0, tags: HashMap::new(), chatty_tags: Vec::new() }
    }
    // Create new Site from a parsed line.
    pub fn from_line(line: LineParser) -> Self {
        let line_questions = 1;
        let line_words = line.texts.join(" ").split_whitespace().count() as i32;
        let mut line_tags: HashMap<String, Stat> = HashMap::new();
        for tag in line.tags {
            line_tags.insert(tag.clone(), Stat::new(line_questions, line_words));
        }
        Self { questions: line_questions,  words: line_words, tags: line_tags, chatty_tags: Vec::new() }
    }
    // Combine two Site instances by summing it's site and tag stats.
    pub fn sum(&mut self, site: &Site) {
        self.questions += site.questions;
        self.words += site.words;
        for (key, value) in &site.tags {
            if self.tags.contains_key(key) {
                let mut tags_sum = self.tags.get(key).expect("ERROR: couldn't fetch the stats for the current tag.").clone();
                tags_sum.sum(value);
                self.tags.insert(key.clone(), tags_sum.clone());
            } else {
                self.tags.insert(key.clone(), value.clone());
            }
        }
    }
    // Get the ratio of the current Site instance.
    pub fn get_site_ratio(&self) -> f64 {
        Stat::calculate_ratio(&self.questions, &self.words)
    }
    // Get the tag stats for the current Site instance.
    pub fn get_tags(&self) -> &HashMap<String, Stat> {
        &self.tags
    }
    // Set the chatty tags for the current Site instance.
    pub fn set_chatty_tags(&mut self, _chatty_tags: Vec<String>) {
        self.chatty_tags = _chatty_tags;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_site_new() -> Result<(), String> {
        let site = Site::new();

        assert!(site.questions == 0 && site.words == 0 && site.tags.len() == 0 && site.chatty_tags.len() == 0);
        Ok(())
    }
    #[test]
    fn test_site_from_line() -> Result<(), String> {
        
        let line_test = r#"{"texts": ["Reasonable to exclude a specific professor from a thesis defense for personal reasons?", "When I started my Master's program, my supervisor was Professor X. However, I had to leave their group after a few months, partly because we didn't get along, but also because the research topic wasn't a good fit.\nPresently, my supervisor is Professor Y (who is awesome, and with whom I performed research work that really interested me). Now, I am getting ready to defend my thesis and need to select professors for the defense committee.\nIt so happens that my current thesis research deals tangentially with Professor X's research focus from about 5 years ago (and by the way, there aren't many professors whose research directly relates to my thesis). Under normal circumstances, Professor X would have been a decent choice for my defense committee.\nHere is where I need a gut-check: I personally do not want Professor X on my defense committee. It is unlikely that they would try to sabotage my defense, but I have always found it distressing to be in Professor X's presence. Yes, I could grin and bear it, but I would simply rather not do that. Is it reasonable to request that Professor X be excluded?"], "tags": ["masters", "thesis-committee", "defense"]}"#;
        let parsed_line_test: LineParser = serde_json::from_str(line_test).expect("ERROR: couldn't get the LineParser from the line.");
        let site = Site::from_line(parsed_line_test);
        
        assert!(site.questions == 1 && site.words == 206 && site.tags.len() == 3 && site.chatty_tags.len() == 0);
        Ok(())
    }
    #[test]
    fn test_site_sum() -> Result<(), String> {
        
        let line_test1 = r#"{"texts": ["Reasonable to exclude a specific professor from a thesis defense for personal reasons?", "When I started my Master's program, my supervisor was Professor X. However, I had to leave their group after a few months, partly because we didn't get along, but also because the research topic wasn't a good fit.\nPresently, my supervisor is Professor Y (who is awesome, and with whom I performed research work that really interested me). Now, I am getting ready to defend my thesis and need to select professors for the defense committee.\nIt so happens that my current thesis research deals tangentially with Professor X's research focus from about 5 years ago (and by the way, there aren't many professors whose research directly relates to my thesis). Under normal circumstances, Professor X would have been a decent choice for my defense committee.\nHere is where I need a gut-check: I personally do not want Professor X on my defense committee. It is unlikely that they would try to sabotage my defense, but I have always found it distressing to be in Professor X's presence. Yes, I could grin and bear it, but I would simply rather not do that. Is it reasonable to request that Professor X be excluded?"], "tags": ["masters", "thesis-committee", "defense"]}"#;
        let parsed_line_test1: LineParser = serde_json::from_str(line_test1).expect("ERROR: couldn't get the LineParser from the line.");
        let mut site1 = Site::from_line(parsed_line_test1);
        
        let line_test2 = r#"{"texts": ["Legality of collecting/distributing a dataset of screenshots of websites", "I'm doing a research project where I want to collect thousands of screenshots of different websites, for training/testing of image models. Ultimately, I am trying to create a dataset that I can present and distribute to the community at various conferences/symposia.\n\nMy primary concern is how legal this might be. My assumption would be that taking a screenshot of websites of private companies like newyorktimes.com would be some sort of copyright violation, but that may be a faulty assumption.\n\nI'm also unclear as to grey areas like:\n\n\nreddit:  user provided content\ntwitter: user provided content\nwikipedia:   open source knowledge base\ngoogle:  content scraped/sourced from other providers\n\n\nEssentially, what copyright considerations, if any, would I have to keep in mind when collecting, annotating and distributing screenshots? How might licensing/attribution work when making this data publically available to others for use?"], "tags": ["data", "license"]}"#;
        let parsed_line_test2: LineParser = serde_json::from_str(line_test2).expect("ERROR: couldn't get the LineParser from the line.");
        let site2 = Site::from_line(parsed_line_test2);
        
        site1.sum(&site2);

        assert!(site1.questions == 2 && site1.words == 354 && site1.tags.len() == 5 && site1.chatty_tags.len() == 0);
        Ok(())
    }
}