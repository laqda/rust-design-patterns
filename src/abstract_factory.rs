#[allow(dead_code)]
pub fn test() {
    let api= v2019::APIFactory::new();
//    let api = v2020::APIFactory::new();
    println!("{:#?}", api.get_all_users());
}

#[derive(Debug)]
pub enum UserRoles {
    ADMIN,
    MANAGER,
    EMPLOYEE,
}

#[derive(Debug)]
pub struct User {
    role: UserRoles,
}

pub trait API {
    fn get_all_users(&self) -> Vec<User>;
}

pub trait APIFactory<A: API> {
    fn new() -> A;
}

mod v2019 {
    pub struct API2019 {}

    impl super::API for API2019 {
        fn get_all_users(&self) -> Vec<super::User> {
            println!("Use 2019 version - I use mysql");
            vec![
                super::User { role: super::UserRoles::ADMIN },
                super::User { role: super::UserRoles::MANAGER },
                super::User { role: super::UserRoles::EMPLOYEE },
            ]
        }
    }

    #[allow(dead_code)]
    pub struct APIFactory {}

    impl super::APIFactory<API2019> for APIFactory {
        fn new() -> API2019 {
            API2019 {}
        }
    }
}

mod v2020 {
    pub struct API2020 {}

    impl super::API for API2020 {
        fn get_all_users(&self) -> Vec<super::User> {
            println!("Use 2020 version - I use postgresql");
            vec![
                super::User { role: super::UserRoles::ADMIN },
                super::User { role: super::UserRoles::EMPLOYEE },
            ]
        }
    }

    #[allow(dead_code)]
    pub struct APIFactory {}

    impl super::APIFactory<API2020> for APIFactory {
        fn new() -> API2020 {
            API2020 {}
        }
    }
}
