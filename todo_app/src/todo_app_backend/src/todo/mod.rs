#![allow(unused)]
use rand::random;
use std::{collections::HashMap, env::current_exe, fmt::format};
struct Todo {
    id: String,
    name: String,
    completed: bool,
}

struct TodoCanister {
    todos: HashMap<String, Todo>,
}

impl TodoCanister {
    pub fn new() -> Self {
        TodoCanister {
            todos: HashMap::new(),
        }
    }

    pub fn add_todo(&mut self, name: String) -> String {
        let id = generate_todo_id();

        let new_todo = Todo {
            id: id.clone(),
            name,
            completed: false,
        };
        self.todos.insert(id.clone(), new_todo);
        id
    }

    pub fn get_todo_by_id(&self, id: &String) -> Option<&Todo> {
        return self.todos.get(id);
    }

    pub fn get_todos_pagination(&self, page_size: usize) -> Vec<Vec<&Todo>> {
        let mut current_page: Vec<&Todo> = Vec::new();
        let mut result: Vec<Vec<&Todo>> = Vec::new();
        let mut processed_todos = 0;
        for item in self.todos.values() {
            current_page.push(item);
            // Here we will be clearing out the current_page vector based on 2 things
            // 1. If the current_page vector and has achieved the page_size
            // 2. If we have iterated all the todos present and then we clear out the vector.
            processed_todos += 1;
            if current_page.len() == page_size || processed_todos == self.todos.len() {
                result.push(current_page.clone());
                current_page.clear();
            }
        }
        result
    }

    pub fn update_todo(&mut self, id : &String, new_text: String) -> Result<(),String>{
     match self.todos.get_mut(id) {
         Some(todo) => {
            todo.name = new_text;
            Ok(())
         }
         None => {
            Err(format!("Error updating the todo {}",id))
         }
     }
    }

    pub fn delete_todo_by_list(&mut self, id: &String) -> Result<(), String> {

        if self.todos.remove(id).is_some() {
            Ok(())
        } else {
            Err("todo with id not found ".to_string())
        }
    }

}
pub fn generate_todo_id() -> String {
    format!("todo-{}", rand::random::<u64>())
}
