use std::path::PathBuf;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
struct Coverage {
    line: f64,
    branch: f64,
}

impl Coverage {
    fn trunc(&mut self) {
        self.line = f64::trunc(self.line * 1000.0) / 1000.0;
        self.branch = f64::trunc(self.branch * 1000.0) / 1000.0;
    }
}

pub fn compare(base: PathBuf, comp: PathBuf, tolerance: f64) -> (usize, String) {
    // println!("File 1: {:?}", &base);
    // println!("File 2: {:?}", &comp);

    let base_result = extract(&base);
    if base_result.is_none() {
        return (9, "No base result for comparison".to_owned());
    }
    let mut base_result = base_result.unwrap();
    base_result.trunc();

    let comp_result = extract(&comp);
    if comp_result.is_none() {
        return (8, "Nothing found to compare".to_owned());
    }
    let mut comp_result = comp_result.unwrap();
    comp_result.trunc();

    let mut msgs: Vec<String> = Vec::new();

    let report_line = format!(
        "from {:.3} to {:.3} ({:+.3})",
        base_result.line,
        comp_result.line,
        comp_result.line - base_result.line
    );

    let report_branch = format!(
        "from {:.3} to {:.3} ({:+.3})",
        base_result.branch,
        comp_result.branch,
        comp_result.branch - base_result.branch,
    );

    let line_ok = (comp_result.line - base_result.line) >= -tolerance;
    let branch_ok = (comp_result.branch - base_result.branch) >= -tolerance;

    let line_symbol = if line_ok { "✅" } else { "❌" };

    let branch_symbol = if branch_ok { "✅" } else { "❌" };

    msgs.push(format!(
        "Line coverage changed {} {}",
        report_line, line_symbol
    ));
    msgs.push(format!(
        "Branch coverage changed {} {}",
        report_branch, branch_symbol
    ));

    let code = if line_ok & branch_ok { 0 } else { 1 };

    (code, msgs.join("\n"))
}

fn extract(file: &PathBuf) -> Option<Coverage> {
    if !file.exists() {
        return None;
    }
    // let ignore = vec!["line", "lines", "methods", "class"];
    // let ignore = ignore.iter().map(|o| o.to_owned()).collect::<String>();

    let f = std::fs::File::open(file).unwrap();
    let f = std::io::BufReader::new(f);
    let parser = EventReader::new(f);
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                let ln = name.local_name;
                if ln == "coverage" {
                    let mut result = Coverage {
                        line: 0.0,
                        branch: 0.0,
                    };
                    attributes.iter().for_each(|a| {
                        if a.name.local_name == "line-rate" {
                            result.line = a.value.parse().unwrap();
                        } else if a.name.local_name == "branch-rate" {
                            result.branch = a.value.parse().unwrap();
                        }
                    });
                    return Some(result);
                }
            }
            // Ok(XmlEvent::EndElement { name }) => {
            //     let ln = name.local_name;
            //     if ln != "line" {
            //         println!("name: {}", &ln);
            //     }
            // }
            Err(_) => return None,
            _ => {}
        }
    }

    Some(Coverage {
        line: 0.0,
        branch: 0.0,
    })
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn same_files() -> (PathBuf, PathBuf) {
        (
            PathBuf::from("secret/coverage.xml"),
            PathBuf::from("secret/coverage.xml"),
        )
    }
    fn different_files() -> (PathBuf, PathBuf) {
        (
            PathBuf::from("secret/coverage.xml"),
            PathBuf::from("secret/coverage (copy).xml"),
        )
    }

    fn make_expected(i: usize, msg: &str) -> (usize, String) {
        (0, String::from("msg"))
    }

    #[test]
    fn test_add() {
        let expected = make_expected(9, "msg");
        let result = compare(PathBuf::from("a.xml"), PathBuf::from("b.xml"), 0.002);
        assert_eq!(result.0, 9);
    }

    #[test]
    fn test_missing_file() -> Result<(), String> {
        let f = PathBuf::from("thisfiledoesntexist.xml");
        match extract(&f) {
            Some(_) => Err("Should be none".to_owned()),
            None => Ok(()),
        }
    }

    #[test]
    fn test_extract_nobase() -> Result<(), String> {
        let (f1, f2) = same_files();
        match extract(&f1) {
            Some(result) => {
                println!("{:?}", &result);
                Ok(())
            }
            None => Err("Should be none".to_owned()),
        }
    }

    #[test]
    fn test_compare_base_no_comp() -> Result<(), String> {
        let (f1, f2) = same_files();
        let result = compare(f1, PathBuf::from("thisfiledoesntexist.xml"), 0.002);
        assert_eq!(result.0, 8);
        Ok(())
    }

    #[test]
    fn test_compare_same() -> Result<(), String> {
        let (f1, f2) = same_files();
        let result = compare(f1, f2, 0.002);
        assert_eq!(result.0, 0);
        // assert_eq!(result.1, "");
        Ok(())
    }

    #[test]
    fn test_compare_different() -> Result<(), String> {
        let (f1, f2) = different_files();
        let result = compare(f1, f2, 0.002);
        println!("{:?}", &result);
        assert_eq!(result.0, 1);
        // assert_eq!(result.1, "");
        Ok(())
    }
}
