

use firebase_rs::*;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct User{
    name: String,
    age: u32,
    email: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct Response{
    name: String
}

#[tokio::main]
async fn main(){
    let user = User{
        name: "shivi_garg_new_2".to_string(),
        age: 30,
        email: "shivansh@gmail.com".to_string(),
    };

    let firebase = Firebase::new("https://crud-rust-4bec2-default-rtdb.firebaseio.com/").unwrap();

    // it will return a user as response
    let response = set_user(&firebase, &user).await;

    // // now we have to print that data stored in firebase
    let user = get_user(&firebase, &response.name).await;
    println!("{:?}", user);

    // // fetch all users
    let users = get_all_users2(&firebase).await;

    delete_user(&firebase, &response.name).await;

}


// 1st func: take a user object and store it in firebase
// it should return. a User.name in response

// Receive Firebase client
//         │
//         ▼
// Receive User
//         │
//         ▼
// Convert User → JSON
//         │
//         ▼
// Send HTTP request
//         │
//         ▼
// Wait for Firebase
//         │
//         ▼
// Receive JSON response
//         │
//         ▼
// Convert JSON → User
//         │
//         ▼
// Return User.name

async fn set_user(firebase_client: &Firebase, user: &User) -> Response{

    // our collection name will be users
    let firebase = firebase_client.at("users");

    // store some data at firebase location
    // firebase == location of collection
    let _users = firebase.set::<User>(&user).await;

    // so _users will have something like 
    // Ok(
    // FirebaseResponse {
    //     data: "{\"name\":\"shivansh_garg\",\"age\":30,\"email\":\"shivansh@gmail.com\"}"
    // }
    println!("firebase response: {:?}", _users);

   // now we have to extract data field and then w get json data 
    // we need to convert to Response type by giving json

    return fn_string_to_response(&_users.unwrap().data);
}


async fn get_user(firebase_client: &Firebase, id: &String) -> User{
    let firebase = firebase_client.at("users").at(&id);
    let user = firebase.get::<User>().await;
    return user.unwrap();
}

// id that is used to store, and its details 
async fn get_all_users(firebase_client: &Firebase) -> HashMap<String, User>{

    let firebase = firebase_client.at("users");
    
    let all_users = firebase.get::<HashMap<String, User>>().await;

    println!("{:?}", all_users);

    return all_users.unwrap();
}


// one more version for get_all_users
async fn get_all_users2(firebase_client: &Firebase){
    
     let firebase = firebase_client.at("users");
    
    let all_users = firebase.get::<HashMap<String, User>>().await;

    match all_users {
        Ok(users) =>{
            println!("All Users:");

            for(id, user) in users{
                println!(" ID: {}, Name: {}", id, user.name);
            }
        },
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

async fn delete_user(firebase_client: &Firebase, id: &String){

        let firebase = firebase_client.at("users").at(&id);

        let delete_query = firebase.delete().await;

        match(delete_query){
            Ok(_) =>{
                println!(" User deleted succesfully");
            },
            Err(e) =>{
                println!(" user deleted failed query")
            }
        }
}

// helper functions
fn fn_string_to_response(s: &str) -> Response{
    serde_json::from_str(s).unwrap()
}
