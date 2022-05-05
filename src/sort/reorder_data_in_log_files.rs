//! Reorder Data in Log Files
//!
//! # Link
//!
//! [937. Reorder Data in Log Files](https://leetcode.com/problems/reorder-data-in-log-files/)

pub struct ReorderDataInLogFiles;

impl ReorderDataInLogFiles {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut indexed_logs: Vec<(usize, String)> = logs.into_iter().enumerate().collect();
        indexed_logs.sort_by(|a, b| {
            let lhs = &a.1;
            let rhs = &b.1;

            let (lhs_ident, lhs_log) = Self::split_log(lhs.as_str());
            let (rhs_ident, rhs_log) = Self::split_log(rhs.as_str());

            let lhs_is_alphabet_log = Self::is_alphabet_log(lhs_log);
            let rhs_is_alphabet_log = Self::is_alphabet_log(rhs_log);

            if !lhs_is_alphabet_log && !rhs_is_alphabet_log {
                return a.0.cmp(&b.0);
            }

            if lhs_is_alphabet_log && rhs_is_alphabet_log {
                let cmp_res = lhs_log.cmp(rhs_log);
                if cmp_res == std::cmp::Ordering::Equal {
                    return lhs_ident.cmp(rhs_ident);
                }
                return cmp_res;
            }

            if lhs_is_alphabet_log {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        indexed_logs.into_iter().map(|pair| pair.1).collect()
    }

    fn is_alphabet_log(log: &str) -> bool {
        !log.chars().nth(0).unwrap().is_numeric()
    }

    fn split_log<'a>(log: &'a str) -> (&'a str, &'a str) {
        let delim_pos = log.find(" ").unwrap();
        let (ident, log_content) = log.split_at(delim_pos);
        return (ident, &log_content[1..]);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let res = super::ReorderDataInLogFiles::reorder_log_files(vec![
            "dig1 8 1 5 1".to_owned(),
            "let1 art can".to_owned(),
            "dig2 3 6".to_owned(),
            "let2 own kit dig".to_owned(),
            "let3 art zero".to_owned(),
        ]);
        assert_eq!(
            res,
            vec![
                "let1 art can".to_owned(),
                "let3 art zero".to_owned(),
                "let2 own kit dig".to_owned(),
                "dig1 8 1 5 1".to_owned(),
                "dig2 3 6".to_owned()
            ]
        );
    }

    #[test]
    fn test2() {
        let res = super::ReorderDataInLogFiles::reorder_log_files(vec![
            "a1 9 2 3 1".to_owned(),
            "g1 act car".to_owned(),
            "zo4 4 7".to_owned(),
            "ab1 off key dog".to_owned(),
            "a8 act zoo".to_owned(),
        ]);
        assert_eq!(
            res,
            vec![
                "g1 act car".to_owned(),
                "a8 act zoo".to_owned(),
                "ab1 off key dog".to_owned(),
                "a1 9 2 3 1".to_owned(),
                "zo4 4 7".to_owned()
            ]
        );
    }
}
