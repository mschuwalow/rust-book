use super::domain::*;
use std::collections::HashMap;
pub struct Runner {
    state: HashMap<Department, Vec<User>>,
}
impl Runner {
    pub fn new() -> Runner {
        Runner {
            state: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, department: &Department, user: &User) -> () {
        let entry = self.state.entry(department.clone()).or_insert(Vec::new());
        (*entry).push(user.clone());
    }

    pub fn list_all_users(&self) -> Vec<User> {
        let mut result = Vec::new();
        for (_, users) in self.state.iter() {
            for user in users {
                result.push(user.clone());
            }
        }
        result
    }

    pub fn list_all_users_in_dep(&self, department: &Department) -> Vec<User> {
        match self.state.get(department) {
            Some(users) => users.clone(),
            None => Vec::new(),
        }
    }
}
