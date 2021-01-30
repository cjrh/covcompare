use std::path::PathBuf;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
struct Coverage {
    line: f64,
    branch: f64,
}

pub fn compare(base: PathBuf, comp: PathBuf) -> (usize, String) {
    // println!("File 1: {:?}", &base);
    // println!("File 2: {:?}", &comp);

    let base_result = extract(&base);
    if base_result.is_none() {
        return (9, "No base result for comparison".to_owned());
    }
    let base_result = base_result.unwrap();

    let comp_result = extract(&comp);
    if comp_result.is_none() {
        return (8, "Nothing found to compare".to_owned());
    }
    let comp_result = comp_result.unwrap();

    let mut msgs: Vec<String> = Vec::new();
    let mut code: usize = 1;
    code = 0;

    let nolinechange = (base_result.line - comp_result.line).abs() < 0.01;
    let nobranchchange = (base_result.branch - comp_result.branch).abs() < 0.01;

    if nolinechange & nobranchchange {
        msgs.push(format!(
            "Coverage remained about the same, at {} line and {} branch",
            comp_result.line, comp_result.branch,
        ));
    } else {
        if base_result.line > comp_result.line {
            msgs.push(format!(
                "Line coverage dropped from {:.3} to {:.3} ({:+.3})",
                base_result.line,
                comp_result.line,
                comp_result.line - base_result.line
            ));
            code = 1;
        }
        if base_result.branch > comp_result.branch {
            msgs.push(format!(
                "Branch coverage dropped from {:.3} to {:.3} ({:+.3})",
                base_result.branch,
                comp_result.branch,
                comp_result.branch - base_result.branch
            ));
            code = 1;
        }
        if comp_result.line > base_result.line {
            msgs.push(format!(
                "Line coverage improved from {:.3} to {:.3} ({:+.3})",
                base_result.line,
                comp_result.line,
                base_result.line - comp_result.line,
            ));
        }
        if comp_result.branch > base_result.line {
            msgs.push(format!(
                "Branch coverage improved from {:.3} to {:.3} ({:+.3})",
                base_result.branch,
                comp_result.branch,
                comp_result.line - base_result.line,
            ));
        }
    }

    (code, msgs.join("; "))
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
                        // println!("    {}", &a);
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
        let result = compare(PathBuf::from("a.xml"), PathBuf::from("b.xml"));
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
        let result = compare(f1, PathBuf::from("thisfiledoesntexist.xml"));
        assert_eq!(result.0, 8);
        Ok(())
    }

    #[test]
    fn test_compare_same() -> Result<(), String> {
        let (f1, f2) = same_files();
        let result = compare(f1, f2);
        assert_eq!(result.0, 0);
        // assert_eq!(result.1, "");
        Ok(())
    }

    #[test]
    fn test_compare_different() -> Result<(), String> {
        let (f1, f2) = different_files();
        let result = compare(f1, f2);
        println!("{:?}", &result);
        assert_eq!(result.0, 1);
        // assert_eq!(result.1, "");
        Ok(())
    }
}
