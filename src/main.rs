use std::{pin::Pin, time::Instant};

use mini_redis::client;
use tokio_stream::{Stream, StreamExt};

// lets say we have a radio station mini_redis is our mini radio station that responsible for boradcast (publish) message
async fn publish() -> mini_redis::Result<()> {
    let mut client = client::connect("127.0.0.8080").await?;

    // Publish some dat
    // publish that responsible for boradcast message with the channel  `mumbers` mesages are 1, 2, 3 so on.
    client.publish("numbers", "1".into()).await?;
    client.publish("numbers", "2".into()).await?;
    client.publish("numbers", "3".into()).await?;
    client.publish("numbers", "four".into()).await?;
    Ok(())
}

// let subscriber is our radion receiver like a home or car that recevie boradcast message.
async fn subscribe() -> mini_redis::Result<()> {
    // client that connect the mini radion station
    let client = client::connect("127.0.0.8080").await?;
    // client.subscriber the specific channel that is numbers
    let subscriber = client.subscribe(vec!["numbers".to_string()]).await?;
    // subscriber that tune the numbers channel to receive a messages
    // let messages = subscriber.into_stream();
    // filter message that has lenght  is 1
    let messages = subscriber
        .into_stream()
        .filter(|msg| match msg {
            Ok(msg) if msg.content.len() == 1 => true,
            _ => false,
        })
        .take(3);
    // or
    /* let messages = subscriber.into_stream().filter(|msg| match msg {
         Ok(msg) if msg.content.len() == 1 =>true,
        _=> false
     })
     .map(|msg| msg.unwrap().content)
     .take(3)
    */
    // tokio::pin responsible for channel so that frequency does not change
    tokio::pin!(messages);
    // while let receive message one by one untill finished the messages.
    while let Some(msg) = messages.next().await {
        println!("GOT MSG = {:?}", msg);
    }
    Ok(())
}

#[tokio::main]

async fn main() -> mini_redis::Result<()> {
    // tokio::spawn is a seperate worker responsible for boradcase  messages with out blocking main radio receiver.
    tokio::spawn(async { publish().await });

    subscribe().await?;
    println!("DONE");

    Ok(())
}

struct Delay {
    when: Instant,
}

use std::task::{Context, Poll};
use std::time::Duration;

// let's say oven to bake cookies with interval with batches
struct Interval {
    rem: usize,
    delay: Delay,
}

// new interval with 3 match to backing the cookie `Delay ` one of the mechanizom to set timer 
impl Interval {
    fn new() -> Self {
        Self {
            rem: 3,
            delay: Delay {
                when: Instant::now(),
            },
        }
    }
}
// Interval produce an unit for successfull backing the batches of cookie
impl Stream for Interval {
    type Item = ();

    // poll_next is abstract method of stream crate that take interval instace and return the Poll with options unit
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<()>> {
        // count down timer if counter is 0 not backing batches remain so retrun None
        if self.rem == 0 {
            // No more delays
            return Poll::Ready(None);
        }
        // if ready to bake batch of backing cookie
        match Pin::new(&mut self.delay).poll(cx) {
            Poll::Ready(_) => {
                // set the time wher to bake the cookie of a batch.
                let when = self.delay.when + Duration::from_millis(10);
                self.delay = Delay { when };
                self.rem -= 1; // set self. rem batch that remain
                Poll::Ready(Some(())) // if time interval is comple retun Poll for the batches
            }
            // if retrun Poll::Pending thats mean batches the still preheading.
            Poll::Pending => Poll::Pending,
        }
    }
}
