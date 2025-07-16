#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() {
    // 初始化配置
    config::init();
    let config = config::get();

    // 初始化日志
    let _guard = config.log.guard();

    // 初始化数据库
    db::init().await;

    // 初始化路由
    api::init().await;
}
