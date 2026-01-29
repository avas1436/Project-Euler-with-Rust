enum UserStatus {
    Active,
    Inactive,
    Banned(String),
}

fn main() {
    for i in [
        Some(UserStatus::Banned(String::from("Spamming"))),
        Some(UserStatus::Active),
        Some(UserStatus::Inactive),
    ] {
        let status: Option<UserStatus> = i;
        match status {
            Some(UserStatus::Active) => println!("User is active ✅"),
            Some(UserStatus::Inactive) => println!("User is inactive ⏸"),
            Some(UserStatus::Banned(reason)) => println!("User is banned ❌. Reason: {}", reason),
            None => println!("No status available ⚠️"),
        }
    }
}
