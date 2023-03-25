use std::io;

use crate::connection::{Backend, QuicConnection};

pub struct HttpConnection<B: Backend + Send> {
    connection: QuicConnection<B>,
    ready: bool,
}

impl<B: Backend + Send> HttpConnection<B> {
    pub(crate) async fn new(connection: QuicConnection<B>) -> Result<Self, io::Error> {
        // TODO: Do the handshake
        todo!()
    }
}