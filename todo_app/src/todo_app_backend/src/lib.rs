use candid::types::number::Nat;
use candid::CandidType;
use std::cell::RefCell;
use std::collections::HashMap;
#[derive(CandidType, Clone)]
struct Todo {
    id: Nat,
    name: String,
    completed: bool,
}

thread_local! {
    static TODO_CANISTER : RefCell<HashMap<String,Todo>> = RefCell::new(HashMap::new());
    // static COUNTER : RefCell<Nat> = RefCell::new(Nat::from(0_u32));
}

#[ic_cdk::update]
fn backfill_todo_items(){
    for i in 1..20{
        let _ = add_todo_item(format!("test-{}",i));
    }
}

#[ic_cdk::update]
fn add_todo_item(item_name: String) -> Result<Todo, String> {
    TODO_CANISTER.with(|todo_item| {
        let mut todo_map = todo_item.borrow_mut();
        let todo_size = todo_map.len();
        let id: Nat = Nat::from(todo_size + 1);
        let new_todo = Todo {
            id: id.clone(),
            name: item_name.clone(),
            completed: false,
        };
        if let Some(existing_todo) = todo_map.get(&item_name) {
            Err(format!(
                "Todo item with name '{}' already exists",
                item_name
            ))
        } else {
            todo_map.insert(item_name.clone(), new_todo.clone());
            Ok(new_todo)
        }
    })
}

#[ic_cdk::query]
fn get_todo_by_name(name: String) -> Result<Todo, String> {
    TODO_CANISTER.with(|todo_list| {
        let todos = todo_list.borrow();
        for todo in todos.values() {
            if todo.name == name {
                return Ok(todo.clone());
            }
        }
        Err("Not found".to_string())
    })
}

#[allow(unused)]
#[ic_cdk::query]
fn get_todos_pagination(page_size: Nat) -> Vec<Vec<Todo>> {
    let mut result: Vec<Vec<Todo>> = Vec::new();
    let mut processed_todos = 0;
    let mut current_page: Vec<Todo> = Vec::new();

    TODO_CANISTER.with(|todo_cell| {
        let todos = todo_cell.borrow();

        for item in todos.values() {
            current_page.push(item.clone()); 
            processed_todos += 1;
            if current_page.len() == page_size || processed_todos == todos.len() {
                result.push(current_page.clone());
                current_page.clear();
            }
        }
    });
    result
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_todo(){
        // let id = Nat::from(1);
        let temp = add_todo_item("test".to_string());
        let temp = add_todo_item("test-1".to_string());
        let temp = add_todo_item("test-2".to_string());
        let temp = add_todo_item("test-3".to_string());
        let temp = add_todo_item("test-4".to_string());
        let temp = add_todo_item("test-5".to_string());
        let temp = add_todo_item("test-6".to_string());
        let temp = add_todo_item("test-7".to_string());
        let temp = add_todo_item("test-8".to_string());
        let temp = add_todo_item("test-9".to_string());
    }

}