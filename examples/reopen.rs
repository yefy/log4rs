fn main() {
    let ret = do_main();
    if let Err(e) = ret {
        println!("err:do_main => e:{:?}", e);
        log::error!("err:do_main => e:{:?}", e);
    }
}

fn do_main() -> anyhow::Result<()> {
    let log4_handle = log4rs::init_file("./conf/log4rs.yaml", Default::default())
        .map_err(|e| anyhow::anyhow!("err:log4rs::init_file => e:{:?}", e))?;

    log::info!("********* reopen");
    let reopen_wait = log4_handle.reopen();
    loop {
        if reopen_wait.try_recv().is_ok() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    log::info!("********* end");
    log::logger().flush();
    Ok(())
}
