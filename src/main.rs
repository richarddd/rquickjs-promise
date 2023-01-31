use std::{env, error::Error, fs};

use rquickjs::{
    BuiltinResolver, Context, FileResolver, Func, ModuleLoader, Object, Runtime, ScriptLoader,
    Tokio,
};

fn print(msg: String) {
    println!("{}", msg);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tokio::task::LocalSet::new()
        .run_until(async {
            let resolver = (
                BuiltinResolver::default(),
                FileResolver::default().with_path("."),
            );
            let loader = (ModuleLoader::default(), ScriptLoader::default());

            let rt = Runtime::new().unwrap();
            rt.set_max_stack_size(512 * 1024);
            rt.set_loader(resolver, loader);
            rt.spawn_executor(Tokio);
            let ctx = Context::full(&rt).unwrap();

            ctx.with(|ctx| {
                let console = Object::new(ctx).unwrap();
                console.set("log", Func::from(print)).unwrap();
                ctx.globals().set("console", console).unwrap();

                let args: Vec<String> = env::args().collect();
                let filename = args.get(1).expect("missing file argument");
                let filename = filename.as_str();

                let source = fs::read_to_string(filename).unwrap();
                ctx.compile(filename, source).unwrap();
            });

            rt.idle().await;
        })
        .await;

    Ok(())
}
