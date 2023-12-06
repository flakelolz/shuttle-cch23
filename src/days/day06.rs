use axum::Json;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct ElfCount {
    elf: i32,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: i32,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf_on_it: i32,
}

pub async fn elfs_on_shelf(body: String) -> Json<ElfCount> {
    let elfs = body.split(' ').filter(|word| word.contains("elf")).count();

    let elf_on_shelf = body
        .lines()
        .filter(|line| line.contains("elf on a shelf"))
        .count();

    let no_elf_on_shelf = body
        .lines()
        .filter(|line| !line.contains("elf on a"))
        .filter(|line| line.contains("shelf"))
        .count();

    Json(ElfCount {
        elf: elfs as i32,
        elf_on_a_shelf: elf_on_shelf as i32,
        shelf_with_no_elf_on_it: no_elf_on_shelf as i32,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_elfs() {
        let body = "there is an elf on a shelf on an elf.\nthere is also another shelf in Belfast."
            .to_string();

        let result = ElfCount {
            elf: 5,
            elf_on_a_shelf: 1,
            shelf_with_no_elf_on_it: 1,
        };

        assert_eq!(elfs_on_shelf(body).await.0, result);
    }
    #[tokio::test]
    async fn test_elfs_on_shelf() {
        let body = "elf on a shelf.\nthere's an elf alone with no shelf,\nbut shelf with no elf on it.\nin belfast\nthere is an elf on a shelf."
            .to_string();

        let result = ElfCount {
            elf: 9,
            elf_on_a_shelf: 2,
            shelf_with_no_elf_on_it: 2,
        };

        println!("{body}");

        assert_eq!(elfs_on_shelf(body).await.0, result);
    }
}

