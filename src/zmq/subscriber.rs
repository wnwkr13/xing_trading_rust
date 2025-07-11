use zmq::{Context, Socket};

pub struct ZmqSubscriber {
    socket: Socket,
}

impl ZmqSubscriber {
    pub fn connect_with_ctx(
        ctx: &Context,
        endpoint: &str,
        topic: &[u8],
    ) -> Result<Self, zmq::Error> {
        let socket = ctx.socket(zmq::SUB)?;
        socket.connect(endpoint)?;
        socket.set_subscribe(topic)?;
        Ok(Self { socket })
    }

    pub fn recv_string(&self) -> Result<String, zmq::Error> {
        match self.socket.recv_string(0) {
            Ok(Ok(string)) => Ok(string),
            Ok(Err(_utf8_error)) => {
                eprintln!("[ZMQ] UTF-8 디코딩 실패, 바이너리로 받으세요.");
                Err(zmq::Error::from_raw(0))
            }
            Err(e) => Err(e),
        }
    }

    pub fn recv_bytes(&self) -> Result<Vec<u8>, zmq::Error> {
        self.socket.recv_bytes(0)
    }

    pub fn recv_string_timeout(&self, timeout_ms: i32) -> Result<Option<String>, zmq::Error> {
        match self.socket.recv_string(timeout_ms) {
            Ok(Ok(string)) => Ok(Some(string)),
            Ok(Err(_)) => Err(zmq::Error::from_raw(0)),
            Err(zmq::Error::EAGAIN) => Ok(None), // 타임아웃
            Err(e) => Err(e),
        }
    }
}
