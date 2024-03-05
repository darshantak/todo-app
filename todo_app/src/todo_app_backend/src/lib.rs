
use candid::CandidType;
use std::cell::RefCell;
use std::collections::HashMap;
use candid::types::number::Nat;
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
fn add_todo_item(item_name: String) {
    let item_id = Nat::from(0_u32);
    let new_todo = Todo {
        id: item_id.clone(),
        name: item_name.clone(),
        completed: false,
    };
    let name = String::from("value");
    TODO_CANISTER.with(|todo_item| {
        todo_item
            .borrow_mut()
            .insert(name, new_todo.clone())
    });

}

// #[ic_cdk::update]
// #[candid_method(update)]
// fn add_todo_item(item_name: String) -> Result<Todo, String> {
//     let id = generate_todo_id();
//     let new_todo = Todo {
//         id: id.clone(),
//         name: item_name.clone(),
//         completed: false,
//     };

//     TODO_CANISTER.with(|todo_item| {
//         let mut todo_map = todo_item.borrow_mut();
//         if let Some(existing_todo) = todo_map.get(&item_name) {
//             Err(format!(
//                 "Todo item with name '{}' already exists",
//                 item_name
//             ))
//         } else {
//             todo_map.insert(item_name.clone(), new_todo.clone());
//             Ok(new_todo)
//         }
//     })
// }

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
