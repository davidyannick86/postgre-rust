pub mod domain {
    pub mod model;
}

pub mod application {
    pub mod user_service;
}

pub mod primary {
    pub mod cli_adapter;
    pub mod cli_port;
    pub mod http_adapter;
    pub mod http_port;
}

pub mod secondary {
    pub mod psql_repo;
    pub mod repo_port;
}
