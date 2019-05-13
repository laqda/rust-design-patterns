#[allow(dead_code)]
pub fn test() {
    let api_factory = v2019::APIFactory2019::new();
//    let api_factory = v2020::APIFactory2020::new();
    let api= api_factory.create();
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

pub trait APIFactory<A> where A: API {
    fn new() -> Self;
    fn create(&self) -> A;
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
    pub struct APIFactory2019 {}

    impl super::APIFactory<API2019> for APIFactory2019 {
        fn new() -> Self {
            APIFactory2019 {}
        }

        fn create(&self) -> API2019 {
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
    pub struct APIFactory2020 {}

    impl super::APIFactory<API2020> for APIFactory2020 {
        fn new() -> Self {
            APIFactory2020 {}
        }

        fn create(&self) -> API2020 {
            API2020 {}
        }
    }
}
