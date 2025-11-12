#![allow(dead_code)]
use regex::Regex;
use std::fs;
use std::mem;

// An extremely manual tree
#[derive(Debug, Default)]
struct GenreTree {
    first_level_genres: Vec<FirstLevelGenre>,
}

#[derive(Debug, Default)]
struct FirstLevelGenre {
    name: String,
    second_level_genres: Vec<SecondLevelGenre>,
}

#[derive(Debug, Default)]
struct SecondLevelGenre {
    name: String,
    third_level_genres: Vec<ThirdLevelGenre>,
}

#[derive(Debug, Default)]
struct ThirdLevelGenre {
    name: String,
    fourth_level_genres: Vec<FourthLevelGenre>,
}

#[derive(Debug, Default)]
struct FourthLevelGenre {
    name: String,
    fifth_level_genres: Vec<FifthLevelGenre>,
}

#[derive(Debug, Default)]
struct FifthLevelGenre {
    name: String,
    sixth_level_genres: Vec<SixthLevelGenre>,
}

#[derive(Debug, Default)]
struct SixthLevelGenre {
    name: String,
    seventh_level_genres: Vec<SeventhLevelGenre>,
}

// Only 7 levels of genres (currently)
#[derive(Debug, Default)]
struct SeventhLevelGenre {
    name: String,
}

// TODO - rewrite as an iterator probably
fn get_next_level(lines: &str, top_layer: bool) -> Vec<(String, String)> {
    let mut genre_blocks: Vec<(String, String)> = Vec::new();
    let top_level_genre_re = Regex::new(r"^    [a-zA-Z0-9]").unwrap();
    let mut sub_genre_block = String::new();
    let mut genre_name: Option<String> = None;
    for line in lines.lines() {
        if top_layer {
            if line.contains("Genre") {
                continue;
            }
            if line.contains("Scenes & Movements") {
                // TODO - handle this in a better way
                genre_blocks.push((genre_name.unwrap().clone(), mem::take(&mut sub_genre_block)));
                break;
            }
        }
        if top_level_genre_re.is_match(line) {
            match genre_name {
                None => {
                    genre_name = Some(line.trim().to_string());
                }
                Some(current_genre_name) => {
                    genre_blocks
                        .push((current_genre_name.clone(), mem::take(&mut sub_genre_block)));
                    genre_name = Some(line.trim().to_string());
                }
            }
        } else {
            sub_genre_block.push_str(&line[4..line.len()]);
            sub_genre_block.push('\n');
        }
    }
    genre_blocks
}

impl GenreTree {
    fn find_all_matching_genres(self) -> Vec<String> {
        unimplemented!();
    }
    // TODO make the structs generic so I can just return the genre type and it's parents
    //  irrelevant of how nested it is. Means I won't have to do a stupid String return either
    // MAYBE find the deepest genre?
    pub fn find_matching_genre(self, search_string: &str) -> Option<String> {
        let matching_string = format!("{}::genre", search_string);
        for first_genre in self.first_level_genres {
            if first_genre.name == matching_string {
                return Some(first_genre.name);
            }
            for second_genre in first_genre.second_level_genres {
                if second_genre.name == matching_string {
                    return Some(format!("{}>>{}", first_genre.name, second_genre.name));
                }
                for third_genre in second_genre.third_level_genres {
                    if third_genre.name == matching_string {
                        return Some(format!(
                            "{}>>{}>>{}",
                            first_genre.name, second_genre.name, third_genre.name
                        ));
                    }
                }
            }
        }
        None
    }
    pub fn load_genres(file_path: &str) -> Self {
        let mut genre_tree = Self {
            first_level_genres: vec![],
        };
        let data =
            fs::read_to_string(file_path).expect("Need the Genre Hierarchy file - see README");
        assert_eq!(data.lines().next().unwrap(), "Genres");
        // TODO - consider how to make this more generic. Would be a lot easier with a more
        //        generic container type rather than explicit types per "layer" of hierarchy
        let first_genres = get_next_level(&data, true);
        for first_genre in first_genres {
            let second_genres = get_next_level(&first_genre.1, false);
            let mut tmp_second_level_genres: Vec<SecondLevelGenre> = Vec::new();
            for second_genre in second_genres {
                let third_genres = get_next_level(&second_genre.1, false);
                let mut tmp_third_level_genres: Vec<ThirdLevelGenre> = Vec::new();
                for third_genre in third_genres {
                    let fourth_genres = get_next_level(&third_genre.1, false);
                    let mut tmp_fourth_level_genres: Vec<FourthLevelGenre> = Vec::new();
                    for fourth_genre in fourth_genres {
                        let fifth_genres = get_next_level(&fourth_genre.1, false);
                        let mut tmp_fifth_level_genres: Vec<FifthLevelGenre> = Vec::new();
                        for fifth_genre in fifth_genres {
                            let sixth_genres = get_next_level(&fifth_genre.1, false);
                            let mut tmp_sixth_level_genres: Vec<SixthLevelGenre> = Vec::new();
                            for sixth_genre in sixth_genres {
                                let seventh_genres = get_next_level(&sixth_genre.1, false);
                                let mut tmp_seventh_level_genres: Vec<SeventhLevelGenre> =
                                    Vec::new();
                                for seventh_genres in seventh_genres {
                                    tmp_seventh_level_genres.push(SeventhLevelGenre {
                                        name: seventh_genres.0,
                                    });
                                }
                                tmp_sixth_level_genres.push(SixthLevelGenre {
                                    name: sixth_genre.0,
                                    seventh_level_genres: tmp_seventh_level_genres,
                                });
                            }
                            tmp_fifth_level_genres.push(FifthLevelGenre {
                                name: fifth_genre.0,
                                sixth_level_genres: tmp_sixth_level_genres,
                            });
                        }
                        tmp_fourth_level_genres.push(FourthLevelGenre {
                            name: fourth_genre.0,
                            fifth_level_genres: tmp_fifth_level_genres,
                        });
                    }
                    tmp_third_level_genres.push(ThirdLevelGenre {
                        name: third_genre.0,
                        fourth_level_genres: tmp_fourth_level_genres,
                    });
                }
                tmp_second_level_genres.push(SecondLevelGenre {
                    name: second_genre.0,
                    third_level_genres: tmp_third_level_genres,
                });
            }
            genre_tree.first_level_genres.push(FirstLevelGenre {
                name: first_genre.0,
                second_level_genres: tmp_second_level_genres,
            });
        }

        genre_tree
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_genres() -> GenreTree {
        let filename = "test/RateYourMusic Genre Hierarchy.txt";
        GenreTree::load_genres(filename)
    }

    #[test]
    fn test_loading_works() {
        let genres = load_genres();
        println!("{:#?}", genres);
        let mut found_uncategorised = false;
        for first_genre in genres.first_level_genres {
            if first_genre.name.contains("Uncategorised") {
                found_uncategorised = true;
            }
        }
        assert!(
            found_uncategorised,
            "Missing the Uncategorised first level genre"
        );
    }

    #[test]
    fn test_finding_second_level_genre() {
        let genres = load_genres();
        assert_eq!(
            genres.find_matching_genre("Avant-Garde Metal"),
            Some("Metal>>Avant-Garde Metal::genre".to_string())
        );
    }

    #[test]
    fn test_finding_deepest_second_level_genre() {
        let genres = load_genres();
        let alternative_metal = genres.find_matching_genre("Alternative Metal");
        assert_ne!(
            alternative_metal,
            Some("Metal>>Alternative Metal".to_string())
        );

        assert_eq!(
            alternative_metal,
            Some("Metal>>Alternative Metal>>Alternative Metal::genre".to_string())
        );
    }
}
