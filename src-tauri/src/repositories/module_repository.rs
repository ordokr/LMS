use async_trait::async_trait;
use uuid::Uuid;
use crate::error::Error;
use crate::db::DB;
use crate::models::course::module::Module;
use crate::repositories::Repository;

pub struct ModuleRepository {
    db: DB,
}

impl ModuleRepository {
    pub fn new(db: DB) -> Self {
        ModuleRepository { db }
    }
    
    pub async fn find_by_course_id(&self, course_id: Uuid) -> Result<Vec<Module>, Error> {
        Module::find_by_course_id(&self.db, course_id).await
    }
    
    pub async fn find_by_course_id_and_name(&self, course_id: Uuid, name: &str) -> Result<Module, Error> {
        let modules = self.find_by_course_id(course_id).await?;
        
        // Find the module with matching name
        for module in modules {
            if module.name == name {
                return Ok(module);
            }
        }
        
        Err(Error::NotFound("Module not found".to_string()))
    }
    
    pub async fn reorder_modules(&self, ordered_ids: Vec<Uuid>) -> Result<(), Error> {
        // Update position for each module based on the order of IDs
        for (index, id) in ordered_ids.iter().enumerate() {
            let mut module = Module::find(&self.db, *id).await?;
            module.position = index as i32;
            module.update(&self.db).await?;
        }
        
        Ok(())
    }
}

#[async_trait]
impl Repository<Module> for ModuleRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Module, Error> {
        Module::find(&self.db, id).await
    }
    
    async fn find_all(&self) -> Result<Vec<Module>, Error> {
        Module::find_all(&self.db).await
    }
    
    async fn create(&self, module: &Module) -> Result<Uuid, Error> {
        module.create(&self.db).await
    }
    
    async fn update(&self, module: &Module) -> Result<(), Error> {
        module.update(&self.db).await
    }
    
    async fn delete(&self, id: Uuid) -> Result<(), Error> {
        Module::delete(&self.db, id).await
    }
}