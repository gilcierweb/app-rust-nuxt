pub trait Model {}

pub trait BaseRepository<TEntity>
    where TEntity: Model {
    fn all(&self) -> Result<Vec<TEntity>, String>;
    fn find(&self, id: &str) -> Result<Option<TEntity>, String>;
    fn create(&mut self, entity: &TEntity) -> Result<(), String>;
    fn update(&mut self, entity: &TEntity) -> Result<(), String>;
    fn delete(&mut self, id: &str) -> Result<(), String>;
}
// model
// struct User;
// // repository current
// struct UserRepository;
//
// impl BaseRepository<User> for UserRepository {
//     fn all(&self) -> Result<Vec<User>, String> {
//         todo!()
//     }
//
//     fn find(&self, id: &str) -> Result<Option<User>, String> {
//         todo!()
//     }
//
//     fn create(&mut self, entity: &User) -> Result<(), String> {
//         todo!()
//     }
//
//     fn update(&mut self, entity: &User) -> Result<(), String> {
//         todo!()
//     }
//
//     fn delete(&mut self, id: &str) -> Result<(), String> {
//         todo!()
//     }
// }
