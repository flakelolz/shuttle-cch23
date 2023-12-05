use axum::extract::Path;

pub async fn power_nums(Path(nums): Path<String>) -> String {
    nums.split('/')
        .map(|s| s.parse::<i32>().unwrap())
        .fold(0_i32, |a, b| (a ^ b))
        .pow(3)
        .to_string()
}

#[allow(dead_code)]
async fn power_nums_i32(Path((num1, num2)): Path<(i32, i32)>) -> String {
    (num1 ^ num2).pow(3).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_power_nums_1() {
        assert_eq!(power_nums(Path("4/8".to_string())).await, "1728")
    }

    #[tokio::test]
    async fn test_power_nums_2() {
        assert_eq!(power_nums(Path("10".to_string())).await, "1000")
    }

    #[tokio::test]
    async fn test_power_nums_3() {
        assert_eq!(power_nums(Path("4/5/8/10".to_string())).await, "27")
    }
}
