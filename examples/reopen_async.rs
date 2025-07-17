fn main() {
    let ret = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(10)
        .max_blocking_threads(512)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { do_main().await });
    if let Err(e) = ret {
        println!("err:do_main => e:{:?}", e);
        log::error!("err:do_main => e:{:?}", e);
    }
}

async fn do_main() -> anyhow::Result<()> {
    let log4_handle = log4rs::init_file("./conf/log4rs.yaml", Default::default())
        .map_err(|e| anyhow::anyhow!("err:log4rs::init_file => e:{:?}", e))?;

    log::info!("multiline ********* reopen");
    log::info!(target:"main", "{}", "multiline ********* reopen");
    log::info!(target:"test", "{}", "multiline ********* reopen");

    let info_str = r#"multiline 1111
222222
3333333"#;
    log::info!("{}", info_str);
    log::info!(target:"main", "{}", info_str);
    log::info!(target:"test", "{}", info_str);

    let reopen_wait = log4_handle.reopen();
    let _ = reopen_wait.recv().await;
    log::info!("********* end");
    log::logger().flush();
    Ok(())
}
