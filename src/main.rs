use rbit::Metainfo;
use teloxide::net::Download;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting...");

    let bot = Bot::from_env();

    let handler = Update::filter_message().endpoint(handle_message);

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn handle_message(bot: Bot, msg: Message) -> ResponseResult<()> {
    if let Some(text) = msg.text()
        && text.starts_with("/start")
    {
        bot.send_message(msg.chat.id, "Send torrent file").await?;
        return Ok(());
    }

    let document = match msg.document() {
        Some(doc) => doc,
        None => return Ok(()),
    };

    let file_name = document
        .file_name
        .as_deref()
        .unwrap_or("not a torrent file");

    if !file_name.ends_with(".torrent") {
        bot.send_message(msg.chat.id, "Please send a .torrent file")
            .await?;
        return Ok(());
    }

    bot.send_message(msg.chat.id, "Parsing...").await?;

    let mut buffer = Vec::new();
    let file = bot.get_file(document.file.id.clone()).await?;
    bot.download_file(&file.path, &mut buffer).await?;

    match Metainfo::from_bytes(&buffer) {
        Ok(metainfo) => {
            let trackers = metainfo.trackers();

            if trackers.is_empty() {
                bot.send_message(msg.chat.id, "No trackers found").await?;
                return Ok(());
            }

            bot.send_message(msg.chat.id, trackers.join("\n")).await?;
        }
        Err(e) => {
            bot.send_message(msg.chat.id, format!("Failed to parse trackers: {}", e))
                .await?;
        }
    }

    Ok(())
}
