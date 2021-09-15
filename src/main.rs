use id3::Tag;
use serde::{Serialize, Deserialize};
use std::{collections::HashMap, env, ffi::OsStr, fs::File};
use ulid::ulid;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut index: HashMap<String, Vec<String>> = HashMap::new();

    let args = env::args().map(|e| e.to_lowercase()).collect::<Vec<String>>();
    let root_path = &args[1].to_string();
    let search_term = &args[2].clone().to_string();

    println!("Search Term: {}", &search_term);
    println!("Search Path: {}", &root_path);

    let db_file_path = "./music_index.json";
    let database_exists = std::path::Path::new(&db_file_path).exists();
    let tag_database: Vec<TagRecord>;

    if database_exists {
        println!("Found existing database");

        let db_file = File::open(&db_file_path)?;
        let reader = std::io::BufReader::new(db_file);
        tag_database = serde_json::from_reader(reader)?;

        println!("Read file contents");
    } else {
        println!("Building database");
        // TODO - better mp3 matching
        tag_database = WalkDir::new(&root_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().is_file() && e.path().extension().unwrap_or(OsStr::new("none")) == "mp3"
            })
            // TODO - combine the tag parsing, creating of internal tag struct and Ok filtering into one method to filter map over. This way I can store the file path
            .map(|e| Tag::read_from_path(e.path()))
            .filter_map(|e| e.ok())
            .map(|t| TagRecord::new(t))
            .collect::<Vec<TagRecord>>();
    
        let serialized = serde_json::to_string(&tag_database).expect("failed to serialize database");
        std::fs::write(&db_file_path, &serialized)?;

        println!("Created database file");
    }

    println!("Mp3 Count: {}", tag_database.len());
    println!("Building index...");

    // build the inverted index
    // [{ term_which_is_single_word_from_artist_name_and_the_hash_key: [ "ulid_of_tag_record", "ulid_of_tag_record", etc... ]}, ...]
    // This should be persisted so it isn't rebuilt every time the app runs.
    // The user can can command the app to rebuild the index or the app can store some the state of the lookup directory and check for changes to rebuild automatically.
    for record in &tag_database {
        let tag = &record.tag;
        let terms: Vec<String> = tag
            .artist
            .split_whitespace()
            .map(|i| i.to_lowercase())
            .map(|i| i.to_string())
            .collect();

        for term in terms {
            let id = record.id.clone();

            if let Some(ids) = index.get_mut(&term) {
                if !ids.contains(&id) {
                    ids.push(id.to_string())
                }
            } else {
                index.insert(term.to_string(), vec![id]);
            }
        }
    }

    println!("index size: {}", index.len());

    // Query the inverted index
    let mut results: Vec<TagRecord> = Vec::new();
    if let Some(ids) = index.get(search_term) {
        for record in tag_database {
            if ids.contains(&record.id) {
                results.push(record);
            }
        }

        for result in results {
            println!(
                "{} - {} - {}",
                result.tag.artist,
                result.tag.album,
                result.tag.title
            )
        }
    } else {
        println!("No records found");
    }

    Ok(())
}

// The lookup value needs to be the term, like "fugazi", "end", "minor", etc...
// That term should have a list of IDs attached to it. The IDs need to be at least a u64, but probably a ulid-lite.
// When looking up, find the term, then fetch all the Tags

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagRecord {
    pub id: String,
    pub tag: MyTag,
}

impl TagRecord {
    pub fn new(tag: Tag) -> Self {
        let id = ulid();
        let my_tag = MyTag {
            artist: tag.artist().unwrap_or("").to_string(),
            album: tag.album().unwrap_or("").to_string(),
            title: tag.title().unwrap_or("").to_string(),
        };

        Self { id, tag: my_tag }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MyTag {
    pub artist: String,
    pub album: String,
    pub title: String,
}
