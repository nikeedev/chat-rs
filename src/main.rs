// use std::env;
// use std::io::{stdin, stdout, Write};
// use std::process::exit;
// use ws::{connect, CloseCode};

// fn main() {
//     // let mut addr = String::new();
//     // print!("Write WebSocket address: ");
//     // let _ = stdout().flush();

//     // stdin().read_line(&mut addr).unwrap();

//     let args: Vec<String> = env::args().collect();

//     if args.len() < 2 {
//         eprintln!("No WebSocket address were specified");
//         exit(2);
//     }

//     let addr = &args[1];

//     let mut text = String::new();

//     let _ = stdout().flush();

//     let mut leave = false;
//     // Connect to the url and call the closure
//     if let Err(error) = connect(format!("ws://{}", addr), |out| {
//         stdin().read_line(&mut text).unwrap();
//         text = text.trim_end().to_string();

//         if text.starts_with('/') {
//             let words = parse_command(text.clone());
//             leave = words[0].as_str() == "leave";
//         } else if out.send(text.clone()).is_err() {
//             println!("Websocket couldn't queue an initial message.")
//         } else {
//             println!("Client sent message 'Hello WebSocket'. ")
//         }

//         // Queue a message to be sent when the WebSocket is open

//         // The handler needs to take ownership of out, so we use move
//         move |msg| {
//             // Handle messages received on this connection
//             println!("Client got message '{}'. ", msg);

//             // Close the connection
//             if leave {
//                 out.close(CloseCode::Normal)
//             } else {
//                 Ok(())
//             }
//         }
//     }) {
//         // Inform the user of failure
//         eprintln!("Failed to connect to chat server. Error: {:?}", error);
//     }
// }

// fn parse_command(command: String) -> Vec<String> {
//     let words: Vec<String> = command[1..command.len() - 1]
//         .to_string()
//         .split(' ')
//         .map(|e| e.to_string())
//         .collect();

//     words
// }

/// An example of a chat web application server
extern crate ws;
use ws::{listen, Handler, Message, Request, Response, Result, Sender};

// This can be read from a file
static INDEX_HTML: &'static [u8] = br#"
<!DOCTYPE html>
<html>
	<head>
		<meta charset="utf-8">
	</head>
	<body>
      <pre id="messages"></pre>
			<form id="form">
				<input type="text" id="msg">
				<input type="submit" value="Send">
			</form>
      <script>
        var socket = new WebSocket("ws://" + window.location.host + "/ws");
        socket.onmessage = function (event) {
          var messages = document.getElementById("messages");
          messages.append(event.data + "\n");
        };
        var form = document.getElementById("form");
        form.addEventListener('submit', function (event) {
          event.preventDefault();
          var input = document.getElementById("msg");
          socket.send(input.value);
          input.value = "";
        });
		</script>
	</body>
</html>
    "#;

// Server web application handler
struct Server {
    out: Sender,
}

impl Handler for Server {
    //
    fn on_request(&mut self, req: &Request) -> Result<(Response)> {
        // Using multiple handlers is better (see router example)
        match req.resource() {
            // The default trait implementation
            "/ws" => Response::from_request(req),

            // Create a custom response
            "/" => Ok(Response::new(200, "OK", INDEX_HTML.to_vec())),

            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }

    // Handle messages received in the websocket (in this case, only on /ws)
    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Broadcast to all connections
        self.out.broadcast(msg)
    }
}

fn main() {
    // Listen on an address and call the closure for each connection
    listen("127.0.0.1:8000", |out| Server { out }).unwrap()
}