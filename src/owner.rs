use crate::prelude::*;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    #[default]
    Add,
    Remove,
}

/// Executes `cargo owner` on all workspace crates: <https://doc.rust-lang.org/cargo/reference/publishing.html#cargo-owner>
///
/// This handler executes the following commands:
/// ```bash
/// $ cargo owner --add github-handle
/// $ cargo owner --remove github-handle
/// $ cargo owner --add github:rust-lang:owners
/// $ cargo owner --remove github:rust-lang:owners
/// ```

pub struct Owner {
    ctx: Context,
}

impl Owner {
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
    }

    pub async fn change(&self, action: Action, username: String) -> Result<()> {
        match &self.ctx {
            Context::Workspace(ctx) => {
                for crt in ctx.crates.iter() {
                    let project = &crt.name().to_string();

                    let (action, descr) = match action {
                        Action::Add => ("--add", "adding"),
                        Action::Remove => ("--remove", "removing"),
                    };

                    let result = cmd!("cargo", "owner", action, &username)
                        .dir(&crt.folder)
                        .run();

                    match result {
                        Ok(_) => {
                            log_info!("Owner", "{project} -> {descr} {username}");
                        }
                        Err(err) => {
                            log_warn!("Owner", "{project} -> {err}");
                            // println!("\n{err}\n");
                            // println!("\t->  {project}\n");
                            // return Ok(());
                        }
                    }
                }
            }
            _ => {
                panic!("not currently supported in the context of a single crate");
            }
        }

        log_info!("Owner", "done");

        Ok(())
    }
}
