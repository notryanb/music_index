use id3::Tag;
use std::{collections::HashMap, ffi::OsStr, path::Path};
use ulid::ulid;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let root_path = Path::new("E:/Mp3s/Fugazi");
    let root_path = Path::new("E:/Mp3s");
    let mut index: HashMap<String, Vec<String>> = HashMap::new();

    let search_term = "China";

    // TODO - better mp3 matching
    let tag_database = WalkDir::new(&root_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().is_file() && e.path().extension().unwrap_or(OsStr::new("none")) == "mp3"
        })
        .map(|e| Tag::read_from_path(e.path()))
        .filter_map(|e| e.ok())
        .map(|t| TagRecord::new(t))
        .collect::<Vec<TagRecord>>();

    println!("Mp3 Count: {}", tag_database.len());
    println!("Building index...");

    // build the index by inserting the 
    for record in &tag_database {
        let tag = &record.tag;        
        let terms: Vec<String> = tag.artist().unwrap_or("").split_whitespace().map(|i| i.to_string()).collect();
        
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
    // dbg!(&index);

    let mut results: Vec<TagRecord> = Vec::new();
    if let Some(ids) = index.get(search_term) {
        for record in tag_database {
            if ids.contains(&record.id) {
                results.push(record);
            }
        }

        for result in results {
            println!("{} - {} - {}", result.tag.artist().unwrap_or(""), result.tag.album().unwrap_or(""), result.tag.title().unwrap_or(""))
        }
    } else {
        println!("No records found");
    }

    Ok(())
}

// The lookup value needs to be the term, like "fugazi", "end", "minor", etc...
// That term should have a list of IDs attached to it. The IDs need to be at least a u64, but probably a ulid-lite.
// When looking up, find the term, then fetch all the Tags

#[derive(Clone, Debug)]
struct TagRecord {
    pub id: String,
    pub tag: Tag,
}

impl TagRecord {
    pub fn new(tag: Tag) -> Self {
        let id = ulid();
        let tag = tag.clone();

        Self { id, tag }
    }
}
