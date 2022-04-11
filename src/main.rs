use sea_orm::{sea_query::*,tests_cfg::*,EntityName,Schema, ConnectionTrait};
use sea_orm::{Database,DatabaseConnection};

fn main(){}

async fn create_table(){
    let db = Database::connect("mysql://root:123456@localhost:3306/rb").await.unwrap();
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
}
