use serde::Deserialize;

#[derive(Deserialize)]
pub struct LineParser {
    pub texts: Vec<String>,
    pub tags: Vec<String>,
}

#[test]
    fn test_site_new() -> Result<(), String> {
        let line_test = r#"{"texts": ["Legality of collecting/distributing a dataset of screenshots of websites", "I'm doing a research project where I want to collect thousands of screenshots of different websites, for training/testing of image models. Ultimately, I am trying to create a dataset that I can present and distribute to the community at various conferences/symposia.\n\nMy primary concern is how legal this might be. My assumption would be that taking a screenshot of websites of private companies like newyorktimes.com would be some sort of copyright violation, but that may be a faulty assumption.\n\nI'm also unclear as to grey areas like:\n\n\nreddit:  user provided content\ntwitter: user provided content\nwikipedia:   open source knowledge base\ngoogle:  content scraped/sourced from other providers\n\n\nEssentially, what copyright considerations, if any, would I have to keep in mind when collecting, annotating and distributing screenshots? How might licensing/attribution work when making this data publically available to others for use?"], "tags": ["data", "license"]}"#;
        let parsed_line_test: LineParser = serde_json::from_str(line_test).expect("ERROR: couldn't get the LineParser from the line.");

        assert!(parsed_line_test.texts.len() == 2 && parsed_line_test.tags.len() == 2);
        Ok(())
    }