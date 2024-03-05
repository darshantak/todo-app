use std::cell::RefCell;
use std::collections::HashMap;
mod todo;
use candid::CandidType;


#[derive(CandidType, Clone)]
struct Todo {
    id: String,
    name: String,
    completed: bool,
}

thread_local! {
    static TODO_CANISTER : RefCell<HashMap<String,Todo>> = RefCell::new(HashMap::new());
}

#[ic_cdk::update]
fn add_todo(name: String) {
    let id = todo::generate_todo_id();
    let new_todo = Todo {
        id: id.clone(),
        name: name.clone(),
        completed: false,
    };
    TODO_CANISTER.with(|todo_item| {
        todo_item
            .borrow_mut()
            .insert(name.clone(), new_todo.clone())
    });
}

#[ic_cdk::query]
fn get_todo_by_name(name: String) -> Result<Todo, String> {
    let todo_item = TODO_CANISTER.with(|todo_item| {
        let todos = todo_item.borrow();
        for todo in todos.values() {
            if todo.name == name {
                return Ok(todo.clone());
            }
        }
        Err("Not found".to_string())
    });
    todo_item
}

#[allow(unused)]
#[ic_cdk::query]
fn get_todos_pagination(page_size: usize) -> Vec<Todo> {
    let mut current_page: Vec<Todo> = Vec::new();
    let mut result: Vec<Vec<Todo>> = Vec::new();
    let mut processed_todos = 0;
    let todo_items = TODO_CANISTER.with(|todo_cell| {
        let todos = todo_cell.borrow();

        for item in todos.values() {
            current_page.push(item.clone()); // Cloning the todo item
            processed_todos += 1;

            if current_page.len() == page_size || processed_todos == todos.len() {
                result.push(current_page.clone());
                current_page.clear();
            }
        }
    });
    result.into_iter().flatten().collect()
}

#[ic_cdk::update]
fn update_todo(name: String, completed: bool) -> Result<(), String> {
    let mut updated = false;
    TODO_CANISTER.with(|todo_cell| {
        let mut todo_items = todo_cell.borrow_mut();
        for item in todo_items.values_mut() {
            if item.name == name {
                item.completed = completed;
                updated = true;
            }
        }
    });

    if updated {
        Ok(())
    } else {
        Err(format!("Error updating the task"))
    }
}

#[ic_cdk::update]
fn delete_todo(name: String) -> Result<(), String> {
    let mut deleted = false;

    TODO_CANISTER.with(|todo_cell| {
        let mut todo_items = todo_cell.borrow_mut();

        todo_items.retain(|_, item| {
            if item.name == name {
                deleted = true;
                false
            } else {
                true 
            }
        });
    });

    if deleted {
        Ok(())
    } else {
        Err(format!("Todo item with name '{}' not found", name))
    }
}
