use std::{collections::HashMap, env, fs::{read_dir, File}, io::{BufRead, BufReader}, path::PathBuf, thread::available_parallelism};
use output::Output;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use serde_json::{from_str, to_string_pretty};
mod site;
mod stat;
use site::Site;
use stat::Stat;
mod line_parser;
mod output;

// Open the file from the PathBuf as a BufReader and return it with the site name.
fn process_file(path: &PathBuf) -> (String, BufReader<File>) {
    let file_name = path.file_name().expect("ERROR: couldn't get the filename of the PathBuf.").to_str().expect("ERROR: couldn't convert to str.");
    let site_name = file_name.replace(".jsonl", "");
    let reader = BufReader::new(File::open(path).expect("ERROR: couldn't open the file."));
    
    (site_name, reader)
}

// Filter files that don't have the .jsonl extension.
fn filter_jsonl(path: PathBuf) -> Option<PathBuf>{
    if path.extension().map_or(false, |ext| ext == "jsonl") {
        Some(path)
    } else {
        None
    }
}

// Parse the files concurrently and get a HashMap<String, Site> with all the sites and their stats.
fn get_sites(folder_name: &str) -> HashMap<String, Site> {
    
    read_dir([env!("CARGO_MANIFEST_DIR"), folder_name].join("/")).expect("ERROR: couldn't read the folder containing the files.")
        .flatten()
        .map(|d| d.path())
        .filter_map(|path| {
            filter_jsonl(path)
        })
        .collect::<Vec<PathBuf>>()
        .par_iter()
        .flat_map(|path| {
            let (site_name, reader) = process_file(path);
            reader.lines().par_bridge().map(move |l| (site_name.clone(), l))
        })
        .map(|(site_name, l)| {
            let parsed_line = from_str(l.expect("ERROR: couldn't get the line as str.").as_str()).expect("ERROR: couldn't get the LineParser from the line.");
            let site = Site::from_line(parsed_line);

            HashMap::from([(site_name, site)])
        })
        .reduce(|| HashMap::new(), |mut sites, site| {
            site.iter().for_each(|(iter_site_name, iter_site)| sites.entry(iter_site_name.clone()).or_insert(Site::new()).sum(iter_site));
            sites
        })
}

// Get the keys of a Vec<(String, f64) ordering by the f64 values in descending order.
fn keys_sorted_by_value(mut vec_stats: Vec<(String, f64)>) -> Vec<String> {
    vec_stats.sort_by(|(tag1, ratio1), (tag2, ratio2)| {
        match ratio2.partial_cmp(ratio1) {
            Some(std::cmp::Ordering::Equal) => tag1.cmp(tag2),
            other => other.expect("ERROR: couldn't compare the two values."),
        }
    });
    let vec = vec_stats.into_iter().map(|(s, _)| s).collect();
    vec
}

// Get the requested Output by calculating the remaining stats.
fn get_output(mut sites: HashMap<String, Site>) -> Output {
    let mut total_tags = HashMap::new();
    let mut total_chatty_sites_stats = Vec::new();
    for (site_name, site) in &mut sites {
        let mut chatty_tags = Vec::new();
        for (tag_name, tag) in site.get_tags() {
            chatty_tags.push((tag_name.clone(), tag.get_ratio()));
            total_tags.entry(tag_name.clone()).or_insert(Stat::new(0, 0)).sum(&tag);
        }
        let mut chatty_tags = keys_sorted_by_value(chatty_tags);
        chatty_tags.truncate(10);
        site.set_chatty_tags(chatty_tags);
        
        total_chatty_sites_stats.push((site_name.clone(), site.get_site_ratio()));
    }
    let mut total_chatty_sites = keys_sorted_by_value(total_chatty_sites_stats);
    total_chatty_sites.truncate(10);
    
    let total_chatty_tags = total_tags.iter().map(|(tag, tag_stat)| (tag.clone(), tag_stat.get_ratio())).collect();
    let mut total_chatty_tags = keys_sorted_by_value(total_chatty_tags);
    total_chatty_tags.truncate(10);
    
    Output::new(103740, sites, total_tags, HashMap::from([("chatty_sites".to_string(), total_chatty_sites), ("chatty_tags".to_string(), total_chatty_tags)]))
}

fn get_threads() -> usize {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        return args[1].parse().expect("ERROR: couldn't process input parameter for the number of threads.");
    }
    available_parallelism().expect("ERROR: couldn't get the machine's number of threads.").get()
}

fn main() {
    let threads = get_threads();
    rayon::ThreadPoolBuilder::new().num_threads(threads).build_global().expect("ERROR: couldn't start Rayon with a custom thread number.");
    
    let sites = get_sites("data");
    let output = get_output(sites);
    let output_json = to_string_pretty(&output).expect("ERROR: couldn't transform the output to JSON-formatted string.");
    println!("{}", output_json);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threads_default() -> Result<(), String> {
        assert_eq!(get_threads(), available_parallelism().expect("ERROR: couldn't get the machine's number of threads.").get());
        Ok(())
    }   
    #[test]
    fn test_output_sample() -> Result<(), String> {
        let sites_sample = get_sites("data_tests");
        let output = get_output(sites_sample);
        let output_json = serde_json::to_string(&output).expect("ERROR: couldn't transform the output to JSON-formatted string.");
       
        let padron = r#"padron":103740"#;
        let anime_stats = r#""anime.stackexchange.com":{"questions":3,"words":606"#;
        let total_chatty_tags = r#""chatty_tags":["jojos-bizarre-adventure","re-creators","phd","graduate-admissions","professors","recommendation-letter","undergraduate","defense","masters","thesis-committee"]"#;
        let total_chatty_sites = r#"["anime.stackexchange.com","academia.stackexchange.com","android.stackexchange.com"]"#;
        assert!(output_json.contains(padron) && output_json.contains(anime_stats) && output_json.contains(total_chatty_tags) && output_json.contains(total_chatty_sites));
        Ok(())
    }
}