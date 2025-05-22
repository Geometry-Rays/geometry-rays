// This is the function for parsing the level download response from the servers
pub fn parse_level_download_response(
    level_download_result: String,
    online_level_name: &mut String,
    online_level_desc: &mut String,
    online_level_diff: &mut u8,
    online_level_rated: &mut bool,
    online_level_creator: &mut String,
    online_level_data: &mut String
) {
    let level_download_result_parts: Vec<&str> = level_download_result.split(";;;;;").collect();
    let level_download_result_parts_empty_user: Vec<&str> = level_download_result.split(";;;;;;").collect();
    let name_desc: Vec<&str> = level_download_result_parts[0].split(";").collect();

    *online_level_name = name_desc[0].to_string();
    *online_level_desc = name_desc[1].to_string();
    *online_level_diff = name_desc[2].parse().unwrap();
    if name_desc.len() > 4 {
        *online_level_creator = name_desc[4].parse().unwrap();
    } else {
        *online_level_creator = "".to_string();
    }

    if name_desc.len() > 4 {
        *online_level_data = level_download_result_parts[1].to_string();
    } else {
        *online_level_data = level_download_result_parts_empty_user[1].to_string();
    }

    if name_desc[3] == "0" {
        *online_level_rated = false
    } else {
        *online_level_rated = true
    }
}