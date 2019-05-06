use failure::Error;
use tokio::codec::{Framed, LinesCodec};
use tokio::net::{UnixListener, UnixStream};
use tokio::prelude::*;

type Result<T> = std::result::Result<T, Error>;

use crate::manager::Handle;

fn handler(socket: UnixStream) -> impl Future<Item = (), Error = ()> {
    let (sink, stream) = Framed::new(socket, LinesCodec::new()).split();
    stream
        .fold(sink, |sink, line: String| {
            println!("received: {}", line);
            sink.send("received".to_string())
            //Ok(())
        })
        .map(|_| ())
        .map_err(|e| ())

}

pub fn listener(handle: Handle) -> Result<impl Future<Item = (), Error = ()>> {
    let listener = UnixListener::bind("/tmp/zinit.unix")?;
    let listener = listener
        .incoming()
        .for_each(|socket| {
            tokio::spawn(handler(socket));

            Ok(())
        })
        .map(|_| ())
        .map_err(|_| ());

    Ok(listener)
}
