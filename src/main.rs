mod init;
mod utils;
mod packages;

fn main() -> Result<(), String> {
    let packages_list = init::init()?;
    println!("{:?}", packages_list);
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::utils::*;

    #[test]
    fn search_package() -> Result<(), String> {
        let packages = get_packages("packages.json")?;

        let mut toret: Option<Package> = None;

        for package in packages {
            if package.name == "wng" {
                toret = Some(package);
            }
        }

        assert!(toret.is_some());

        println!("{}", toret.unwrap());
        Ok(())
    }

    #[test]
    fn package_serialization() {
        let to_ser = Package::new(
            "wng",
            vec!["Wafelack <wafelack@protonmail.com>"],
            "3.5.0",
            "N/A",
            "Wanager is a package and projects manager for C and C++"
        );
        let serialized = serde_json::to_string(&to_ser).unwrap();
        assert_eq!(
            serialized,
            r#"{"name":"wng","description":"Wanager is a package and projects manager for C and C++","authors":["Wafelack <wafelack@protonmail.com>"],"version":"3.5.0","source":"N/A"}"#
        );
    }

    #[test]
    fn package_deserialization() {
        let to_deser = r#"{"name":"wng", "description":"Wanager is a package and projects manager for C and C++", "authors":["Wafelack <wafelack@protonmail.com>"],"version":"3.5.0","source":"N/A"}"#;
        let deserialized: Package = serde_json::from_str(&to_deser).unwrap();
        assert_eq!(
            deserialized,
            Package::new(
                "wng",
                vec!["Wafelack <wafelack@protonmail.com>"],
                "3.5.0",
                "N/A",
                "Wanager is a package and projects manager for C and C++"
            )
        );
    }
}
