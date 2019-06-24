use reqwest::{Client, Url};
use std::fs::File;
use std::path::Path;

#[allow(clippy::match_wild_err_arm)]
pub fn download_file(uri: &str, filename: &str) {
    if Path::new(filename).exists() {
        println!("\t\tFile {} already exists.  Skipping.", filename);
    } else {
        let client = Client::new();
        println!("\t\tDownloading {} from {}", filename, uri);
        match client.get(uri).send() {
            Ok(mut res) => {
                let mut file = File::create(filename).unwrap();
                res.copy_to(&mut file).unwrap();
            }
            Err(_) => panic!("Boom!"),
        }
    }
}

pub fn get_filename(uri: &str, filename: &str) -> String {
    let url = Url::parse(uri).unwrap();

    let mut target_filename = filename;
    if target_filename.is_empty() {
        if let Some(segments) = url.path_segments() {
            for segment in segments {
                target_filename = segment;
            }
        };
    };
    target_filename.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_filename_with_no_default() {
        let expected = "somefile.txt".to_string();
        let actual = get_filename("https://somewhere.com/somefile.txt", "");

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_filename_with_default() {
        let expected = "otherfile.txt".to_string();
        let actual = get_filename("https://somewhere.com/somefile.txt", "otherfile.txt");

        assert_eq!(expected, actual);
    }
}
