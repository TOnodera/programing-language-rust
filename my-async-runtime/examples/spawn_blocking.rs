use std::sync::{Arc, Mutex};
use std::task::Waker;

pub struct SpawnBlocking<T>(Arc<Mutex<Shared<T>>>);

pub struct Shared<T> {
    pub value: Option<T>,
    pub waker: Option<Waker>,
}

pub fn spawn_blocking<T, F>(clusure: F) -> SpawnBlocking<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    let inner = Arc::new(Mutex::new(Shared {
        value: None,
        waker: None,
    }));

    std::thread::spawn({
        // Arcなのでcloneしたときは参照カウントが+1される
        let inner = inner.clone();
        move || {
            let value = clusure();

            let maybe_waker = {
                let mut gurad = inner.lock().unwrap();
                gurad.value = Some(value);
                // waker.take()はSome(waker)を返しShared.wakerはNoneになる（消費する)
                gurad.waker.take()
            };

            if let Some(waker) = maybe_waker {
                // エグゼキューター(非同期スレッド実行環境)を起動する(ポーリングさせる)
                waker.wake();
            }
        }
    });

    SpawnBlocking(inner)
}

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

impl<T: Send> Future for SpawnBlocking<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let mut gurad = self.0.lock().unwrap();
        if let Some(value) = gurad.value.take() {
            return Poll::Ready(value);
        }

        // Readyになっていない場合はwakerをクローンしておいてwake()できるようにする。
        gurad.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

#[tokio::main]
async fn main() {
    let asy = spawn_blocking(|| {
        return 10;
    })
    .await;
    println!("{}", asy);
}
