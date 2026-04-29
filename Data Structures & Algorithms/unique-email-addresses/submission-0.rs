use std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        emails.into_iter()
            .map(|email| {

                let mut parts = email.split('@');
                let local = parts.next().unwrap();
                let domain = parts.next().unwrap();
                

                let clean_local = local.split('+').next().unwrap().replace('.', "");
                

                format!("{}@{}", clean_local, domain)
            })

            .collect::<HashSet<String>>()
            .len() as i32
    }
}