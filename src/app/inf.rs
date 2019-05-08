use failure::Error;
use serde::{Deserialize, Serialize};
use tokio::codec::{Framed, FramedRead, FramedWrite, LengthDelimitedCodec};
use tokio::net::{UnixListener, UnixStream};
use tokio::prelude::*;
use tokio_serde_json::{ReadJson, WriteJson};

type Result<T> = std::result::Result<T, Error>;

use crate::manager::Handle;

#[serde(default)]
#[derive(Debug, Default, Serialize, Deserialize)]
struct Request {
    pub cmd: String,
    pub args: Vec<String>,
}

#[serde(default)]
#[derive(Debug, Default, Serialize, Deserialize)]
struct Response {
    pub body: String,
}

fn handler(socket: UnixStream) -> impl Future<Item = (), Error = ()> {
    let (send, recv) = Framed::new(socket, LengthDelimitedCodec::new()).split();

    // Serialize frames with JSON
    let writer = WriteJson::new(send);
    let reader = ReadJson::<_, Request>::new(recv);

    reader
        .fold(writer, |writer, line: Request| {
            println!("received: {:?}", line);

            writer.send(Response {
                body: "response".to_string(),
            })
        })
        .map(|_| ())
        .map_err(|_| ())
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

fn request(request: Request) -> Result<()> {
    use tokio::net::UnixStream;

    let future = UnixStream::connect("/tmp/zinit.unix")
        // .map_err(|err| {
        //     println!("failed to connect: {}", err);
        //     //()
        // })
        .and_then(|socket| {
            let (send, recv) = Framed::new(socket, LengthDelimitedCodec::new()).split();

            // Serialize frames with JSON
            let writer = WriteJson::new(send);
            let reader = ReadJson::<_, Response>::new(recv);

            // Send the value
            writer.send(request).and_then(move |_| {
                //recv
                let r = Response {
                    body: "".to_string(),
                };
                reader.take(1).fold(r, |r: Response, response: Response| {
                    println!("got response: {:?}", response);
                    Ok(response)
                })
            })
        });

    //tokio::run(future);

    Ok(())
}

pub fn status(service: Option<&str>) -> Result<()> {
    Ok(())
}
