fn open_or_senior(_data: Vec<(i32, i32)>) -> Vec<String> {
    if _data.len() > 0 {
        return _data
            .into_iter()
            .map(|(age, handicap)| {
                if age >= 55 && handicap > 7 {
                    return String::from("Senior");
                }
                String::from("Open")
            })
            .collect::<Vec<String>>();
    }
    return vec![];
}

#[cfg(test)]
mod tests {
    use crate::categorize_new_member::open_or_senior;

    #[test]
    fn when_members_is_empty_return_empty_vector() {
        let members: Vec<(i32, i32)> = vec![];
        let expected: Vec<String> = vec![];
        assert_eq!(expected, open_or_senior(members));
    }

    #[test]
    fn when_member_has_age_laess_than_55_and_handicap_less_than_7() {
        let members: Vec<(i32, i32)> = vec![(33, 4)];
        let expected: Vec<String> = vec![String::from("Open")];
        assert_eq!(expected, open_or_senior(members));
    }

    #[test]
    fn when_member_has_age_more_than_55_and_handicap_more_than_7() {
        let members: Vec<(i32, i32)> = vec![(58, 10)];
        let expected: Vec<String> = vec![String::from("Senior")];
        assert_eq!(expected, open_or_senior(members));
    }

    #[test]
    fn when_mutiple_members() {
        let members: Vec<(i32, i32)> = vec![(45, 12), (55, 21), (19, -2), (104, 20)];
        let expected: Vec<String> = vec![
            String::from("Open"),
            String::from("Senior"),
            String::from("Open"),
            String::from("Senior"),
        ];
        assert_eq!(expected, open_or_senior(members));
    }
}
