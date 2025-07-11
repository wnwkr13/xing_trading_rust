use std::sync::{Arc, Mutex};
use zmq::{Context, Socket};

pub struct ZmqPublisher {
    socket: Arc<Mutex<Socket>>,
}

impl ZmqPublisher {
    /// endpoint 예: "tcp://0.0.0.0:5555"
    pub fn bind(endpoint: &str) -> Result<Self, zmq::Error> {
        let ctx = Context::new();
        let socket = ctx.socket(zmq::PUB)?;
        socket.bind(endpoint)?;
        Ok(Self {
            socket: Arc::new(Mutex::new(socket)),
        })
    }

    /// endpoint 예: "tcp://127.0.0.1:5555"
    pub fn connect(endpoint: &str) -> Result<Self, zmq::Error> {
        let ctx = Context::new();
        let socket = ctx.socket(zmq::PUB)?;
        socket.connect(endpoint)?;
        Ok(Self {
            socket: Arc::new(Mutex::new(socket)),
        })
    }

    /// 메시지 publish (String, &str, &[u8] 모두 지원)
    pub fn send<S: AsRef<[u8]>>(&self, msg: S) -> Result<(), zmq::Error> {
        self.socket.lock().unwrap().send(msg.as_ref(), 0)?;
        Ok(())
    }

    /// Arc<Mutex<Socket>> clone 반환 (콜백 등에서 사용)
    pub fn arc(&self) -> Arc<Mutex<Socket>> {
        Arc::clone(&self.socket)
    }
}
