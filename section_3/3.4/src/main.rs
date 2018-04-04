extern crate some_crate;

#[test]
fn test_user_structure(){
	let new_user = some_crate::User{
    	name: "Dave".to_string(),
    	email: "dave@mail.com".to_string(),
    	age: 32,
    	user_type: some_crate::UserType::Guest
    };

    assert_eq!("Dave".to_string(), new_user.name);
}

fn main() {

    let new_user = some_crate::User{
    	name: "Dave".to_string(),
    	email: "dave@mail.com".to_string(),
    	age: 32,
    	user_type: some_crate::UserType::Guest
    };

    new_user.print_user();
}
