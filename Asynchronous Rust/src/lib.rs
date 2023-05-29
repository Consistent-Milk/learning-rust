use futures;

struct Song {
    song: String,
}

impl Song {
    fn new(song: String) -> Self {
        Song { song: (song) }
    }
}

async fn learn_song() -> Song {
    Song::new("Hello world!".to_string())
}

async fn sing_song(song: Song) {
    println!("Singing song: {}", song.song);
}

async fn dance() {
    println!("Dancing too!");
}

async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song: Song = learn_song().await;
    sing_song(song).await;
}

// Only make this public
// as async_main calls all other necessary async functions
// on this module
pub async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(f1, f2);
}