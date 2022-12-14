

pub struct Processor {
    ctx : Arc<Context>,

}

impl Processor {
    pub fn new(ctx : Arc<Context>) -> Processor {
        Processor {
            ctx : ctx
        }
    }
}

impl Processor {
    pub fn sync(&mut self) {

        
        for repository in manifest.repositories.iter() {
            // Repository
            
            repository.sync().await?;
        }

    }
}