use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

#[derive(Debug, PartialEq, Eq)]
pub struct SearchMatch {
    path: PathBuf,
    line_number: usize,
    content: String,
}

impl SearchMatch {
    pub fn new(path: impl Into<PathBuf>, line_number: usize, content: &str) -> Self {
        Self {
            path: path.into(),
            line_number,
            content: content.to_string(),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn line_number(&self) -> usize {
        self.line_number
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

type SearchResult = io::Result<Vec<SearchMatch>>;

#[derive(Debug, Clone, Copy, Default)]
pub struct SearchOptions {
    case_insensitive: bool,
    recursive: bool,
}

impl SearchOptions {
    pub fn new(case_insensitive: bool, recursive: bool) -> Self {
        Self {
            case_insensitive,
            recursive,
        }
    }
}

pub fn search(needle: &str, path: impl AsRef<Path>, options: SearchOptions) -> SearchResult {
    let path = path.as_ref();
    let metadata = fs::metadata(path)?;

    if metadata.is_file() {
        search_in_file(needle, path, options)
    } else if metadata.is_dir() {
        search_in_dir(needle, path, options)
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Path is neither a file nor a directory",
        ))
    }
}

fn search_in_file(needle: &str, path: impl AsRef<Path>, options: SearchOptions) -> SearchResult {
    let path = path.as_ref();
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let mut results = Vec::new();

    let needle = if options.case_insensitive {
        needle.to_lowercase()
    } else {
        needle.to_string()
    };
    for (line_num, line) in buf.lines().enumerate() {
        let line = line?;
        let is_match = if options.case_insensitive {
            line.to_lowercase().contains(&needle)
        } else {
            line.contains(&needle)
        };
        if is_match {
            results.push(SearchMatch::new(path, line_num + 1, &line));
        }
    }
    Ok(results)
}

fn search_in_dir(needle: &str, path: impl AsRef<Path>, options: SearchOptions) -> SearchResult {
    let dir = path.as_ref();
    let mut results = Vec::new();

    let mut entries = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        entries.push((path, file_type));
    }

    entries.sort_by(|(left, _), (right, _)| left.cmp(right));

    for (path, file_type) in entries {
        if file_type.is_file() {
            results.extend(search_in_file(needle, &path, options)?);
        } else if file_type.is_dir() && options.recursive {
            results.extend(search_in_dir(needle, &path, options)?);
        }
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_find_in_one_file() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "needle in the haystack").unwrap();
        let res = search("needle", file.path(), SearchOptions::default());
        assert!(res.is_ok());
        let got = res.unwrap();
        let expected = SearchMatch::new(file.path(), 1, "needle in the haystack");
        assert_eq!(got, vec![expected]);
    }

    #[test]
    fn test_find_emoji_in_one_file() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "needle in the haystack").unwrap();
        writeln!(file, "emoji: 😀").unwrap();

        let res = search("😀", file.path(), SearchOptions::default());
        assert!(res.is_ok());
        let matches = res.unwrap();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].path(), file.path());
        assert_eq!(matches[0].line_number(), 2);
        assert_eq!(matches[0].content(), "emoji: 😀");
    }

    #[test]
    fn test_find_chinese_in_file() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "needle in the haystack").unwrap();
        writeln!(file, "emoji: 😀").unwrap();
        writeln!(file, "这是一个测试").unwrap(); // Chinese

        let res = search("测试", file.path(), SearchOptions::default());
        assert!(res.is_ok());
        let matches = res.unwrap();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].path(), file.path());
        assert_eq!(matches[0].line_number(), 3);
        assert_eq!(matches[0].content(), "这是一个测试");
    }

    #[test]
    fn test_find_japanese_in_file() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "needle in the haystack").unwrap();
        writeln!(file, "emoji: 😀").unwrap();
        writeln!(file, "これはテストです").unwrap(); // Japanese

        let res = search("テスト", file.path(), SearchOptions::default());
        assert!(res.is_ok());
        let matches = res.unwrap();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].path(), file.path());
        assert_eq!(matches[0].line_number(), 3);
        assert_eq!(matches[0].content(), "これはテストです");
    }

    #[test]
    fn test_find_korean_in_file() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "needle in the haystack").unwrap();
        writeln!(file, "emoji: 😀").unwrap();
        writeln!(file, "이것은 테스트입니다").unwrap(); // Korean

        let res = search("테스트", file.path(), SearchOptions::default());
        assert!(res.is_ok());
        let matches = res.unwrap();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].path(), file.path());
        assert_eq!(matches[0].line_number(), 3);
        assert_eq!(matches[0].content(), "이것은 테스트입니다");
    }

    #[test]
    fn test_find_not_in_one_file() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "needle in the haystack").unwrap();
        let res = search("missing", file.path(), SearchOptions::default());
        assert!(res.is_ok());
        assert!(res.unwrap().is_empty());
    }

    #[test]
    fn test_find_lines_in_one_file() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "needle in the haystack").unwrap();
        writeln!(file, "emoji: 😀").unwrap();
        writeln!(file, "这是一个测试").unwrap(); // Chinese
        writeln!(file, "😀 これはテストです").unwrap();
        writeln!(file, "这是另一个测试").unwrap(); // Chinese
        writeln!(file, "最后一个测试 😀").unwrap(); // Chinese

        let res = search("😀", file.path(), SearchOptions::default());
        assert!(res.is_ok());
        let matches = res.unwrap();
        assert_eq!(matches.len(), 3);
        assert_eq!(matches[0].line_number(), 2);
        assert_eq!(matches[0].content(), "emoji: 😀");
        assert_eq!(matches[1].line_number(), 4);
        assert_eq!(matches[1].content(), "😀 これはテストです");
        assert_eq!(matches[2].line_number(), 6);
        assert_eq!(matches[2].content(), "最后一个测试 😀");
    }

    #[test]
    fn test_open_non_existent_file() {
        let res = search("needle", "non_existent_file.txt", SearchOptions::default());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn test_search_match_creation() {
        let search_match = SearchMatch::new("path/to/file.txt", 42, "This is a matching line.");
        assert_eq!(search_match.path(), Path::new("path/to/file.txt"));
        assert_eq!(search_match.line_number(), 42);
        assert_eq!(search_match.content(), "This is a matching line.");
    }

    #[test]
    fn test_find_in_a_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file1_path = temp_dir.path().join("file1.txt");
        let file2_path = temp_dir.path().join("file2.txt");

        let mut file1 = File::create(&file1_path).unwrap();
        writeln!(file1, "needle in the haystack").unwrap();

        let mut file2 = File::create(&file2_path).unwrap();
        writeln!(file2, "no match here").unwrap();

        let res = search("needle", temp_dir.path(), SearchOptions::default());
        assert!(res.is_ok());
        let matches = res.unwrap();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].path(), file1_path.as_path());
        assert_eq!(matches[0].line_number(), 1);
        assert_eq!(matches[0].content(), "needle in the haystack");
    }

    #[test]
    fn test_find_in_nested_directories() {
        let temp_dir = tempfile::tempdir().unwrap();
        let nested_dir = temp_dir.path().join("nested");
        fs::create_dir(&nested_dir).unwrap();

        let file1_path = temp_dir.path().join("file1.txt");
        let file2_path = nested_dir.join("file2.txt");

        let mut file1 = File::create(&file1_path).unwrap();
        writeln!(file1, "needle in the haystack").unwrap();

        let mut file2 = File::create(&file2_path).unwrap();
        writeln!(file2, "needle in the nested haystack").unwrap();

        let res = search("needle", temp_dir.path(), SearchOptions::default());
        assert!(res.is_ok());
        let matches = res.unwrap();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].path(), file1_path.as_path());
        assert_eq!(matches[0].line_number(), 1);
        assert_eq!(matches[0].content(), "needle in the haystack");

        let res = search(
            "needle",
            temp_dir.path(),
            SearchOptions {
                case_insensitive: false,
                recursive: true,
            },
        );
        assert!(res.is_ok());
        let matches = res.unwrap();
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].path(), file1_path.as_path());
        assert_eq!(matches[0].line_number(), 1);
        assert_eq!(matches[0].content(), "needle in the haystack");
        assert_eq!(matches[1].path(), file2_path.as_path());
        assert_eq!(matches[1].line_number(), 1);
        assert_eq!(matches[1].content(), "needle in the nested haystack");
    }

    #[test]
    fn test_find_in_directory_returns_matches_sorted_by_path() {
        let temp_dir = tempfile::tempdir().unwrap();
        let nested_dir = temp_dir.path().join("a_dir");
        fs::create_dir(&nested_dir).unwrap();

        let later_path = temp_dir.path().join("z_file.txt");
        let earlier_path = nested_dir.join("a_file.txt");

        let mut later_file = File::create(&later_path).unwrap();
        writeln!(later_file, "needle in root file").unwrap();

        let mut earlier_file = File::create(&earlier_path).unwrap();
        writeln!(earlier_file, "needle in nested file").unwrap();

        let res = search("needle", temp_dir.path(), SearchOptions::new(false, true));
        assert!(res.is_ok());
        let matches = res.unwrap();
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].path(), earlier_path.as_path());
        assert_eq!(matches[0].content(), "needle in nested file");
        assert_eq!(matches[1].path(), later_path.as_path());
        assert_eq!(matches[1].content(), "needle in root file");
    }

    #[test]
    fn test_find_in_one_file_with_case_insensitive() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "needle in the haystack").unwrap();
        writeln!(file, "NEEDLE IN THE HAYSTACK").unwrap();
        let res = search("needle", file.path(), SearchOptions::default());
        assert!(res.is_ok());
        let got = res.unwrap();
        let expected = SearchMatch::new(file.path(), 1, "needle in the haystack");
        assert_eq!(got, vec![expected]);

        let res = search(
            "needle",
            file.path(),
            SearchOptions {
                case_insensitive: true,
                recursive: false,
            },
        );
        assert!(res.is_ok());
        let got = res.unwrap();
        assert_eq!(got.len(), 2);
        assert_eq!(got[0].path(), file.path());
        assert_eq!(got[0].line_number(), 1);
        assert_eq!(got[0].content(), "needle in the haystack");
        assert_eq!(got[1].path(), file.path());
        assert_eq!(got[1].line_number(), 2);
        assert_eq!(got[1].content(), "NEEDLE IN THE HAYSTACK");
    }
}
