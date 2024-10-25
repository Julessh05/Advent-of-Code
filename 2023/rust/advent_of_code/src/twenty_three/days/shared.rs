use std::fs;

/// Get the content corresponding to the current day.
/// You only have to pass the name of the folder that lives under
/// the twenty_three/days folder.
///
/// IMPORTANT
/// For this to work, your input / puzzle data path has to be relative to the shared.rs file in the
/// days module
pub fn get_content(folder: &str) -> Vec<String> {
    let content: String = fs::read_to_string(format!("src/twenty_three/days/{folder}/content.txt"))
        .expect("Error opening content file corresponding to {filename}");
    let mut vec: Vec<String> = content.split("\n").map(str::to_string).collect();
    vec.retain(|line| !line.is_empty());
    return vec;
}
