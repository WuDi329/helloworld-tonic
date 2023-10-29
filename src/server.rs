use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

// 这个宏的作用类似于 C 语言中的 #include 指令，但是它是一个 Rust 的宏，
// 可以在编译时将指定文件的内容插入到 Rust 代码中。
// 自动生成一个hello_world模块,包含了Greeter服务和相关的消息类型的定义
// proto中的服务是一个trait
pub mod hello_world {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> { // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}


// Tokio 的 #[tokio::main] 属性宏用于标记 main 函数，表示这是一个异步函数，需要在 Tokio 的异步运行时中运行。
// 当程序运行时，Tokio 的异步运行时会创建一个事件循环，然后将异步任务提交给事件循环进行处理。当一个异步任务需要等待 I/O 操作等需要等待的操作时，它会将自己挂起，然后将控制权交给事件循环，等待操作完成后再恢复执行。这种方式可以在不阻塞线程的情况下处理大量的并发连接，从而提高程序的性能和响应能力。
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}