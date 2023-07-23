use std::{time::SystemTime, sync::{Arc, RwLock}};

use interprocess::os::windows::named_pipe::ByteReaderPipeStream;
use log::{error, info};
use lazy_static::lazy_static;

mod nozomi;
use nozomi::lua::NozomiLua;
use poggers::internal::windows::module::InModule;

// static LOCAL_PLAYER_ADDRESS: usize = 0x17E0A8;
lazy_static! {
    static ref PROCESS: Arc<RwLock<Option<InModule>>> = Default::default();
}
static LUA_STR: &str = include_str!("../test.lua");

fn setup_logging() {
    fern::Dispatch::new()
        .chain(std::io::stdout())
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                // This will color the log level only, not the whole line. Just a touch.
                record.level(),
                record.target(),
                message
            ))
        })
        .apply()
        .unwrap();
}

#[poggers_derive::create_entry(no_console)]
fn entry() -> Result<(), Box<dyn std::error::Error>> {
    // We manually allocate a console because poggers is stupid.
    unsafe {
        poggers::exports::AllocConsole();
    }

    *PROCESS.write().unwrap() = Some(InModule::new("ac_client.exe")?);

    setup_logging();

    info!("DLL Injected");

    let lua = NozomiLua::new()?;

    match lua.load(LUA_STR) {
        Ok(_) => {
            info!("It executed.")
        }
        Err(err) => {
            error!("Got error: {err}");
        }
    };

    // TODO: Actually make namedpipes work :guh:
    // let cunt = ByteReaderPipeStream::connect("Nozomi")?;


    Ok(())
}
