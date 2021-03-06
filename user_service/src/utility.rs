/// Outcomes is used to wrap Vec<User>
#[derive(Serialize, Debug, PartialEq, Deserialize)]
pub struct Outcomes<T> {
    pub outcomes: Vec<T>,
}

/// wrap_vec is used to map Vec<User> into Outcomes
pub fn wrap_vec<T>(v: Vec<T>) -> Outcomes<T> {
    Outcomes { outcomes: v }
}

#[cfg(test)]
mod tests {
    use crate::model::User;
    use crate::utility::Outcomes;
    use crate::utility::wrap_vec;

    #[test]
    fn test_wrap_vec() {
        let users: Vec<User> = vec![
            User {
                id: "101".to_string(),
                name: "sanjay".to_string(),
                email: "sanjay@gmail.com".to_string(),
            },
            User {
                id: "102".to_string(),
                name: "sunil".to_string(),
                email: "sunil@gmail.com".to_string(),
            },
        ];
        let outcomes: Outcomes<User> = Outcomes {
            outcomes: users.clone(),
        };

        assert_eq!(wrap_vec(users), outcomes);
    }
}