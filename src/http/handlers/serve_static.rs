use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::PathBuf;
use std::str;

use http::request::Request;
use http::response::Response;

pub fn handler(req: Request, mut res: Response, stream: TcpStream) -> Response {
    // FIXME: use handler config instead of server config
    let server = res.server.clone();

    // Check HTTP method
    let mut methods = vec!["GET", "HEAD"];
    if server.allow_trace {
        methods.push("TRACE");
    }
    if let None = methods.iter().find(|&&method| method == req.method) {
        res.status_code = 501;
        res.status_message = "Not Implemented".into();
        res.send(&stream);
        return res;
    }

    if req.method == "TRACE" {
        // The TRACE method is used to invoke a remote, application-layer
        // loop-back of the request message. The final recipient of the
        // request SHOULD reflect the message received back to the client
        // as the entity-body of a 200 (OK) response.
        //
        // If the request is valid, the response SHOULD contain the entire
        // request message in the entity-body, with a Content-Type of
        // "message/http".
        //
        // (RFC 2616 9.8)
        res.headers.set("content-type", "message/http");
        res.body = req.to_string().as_bytes().to_vec();
        res.send(&stream);
        return res;
    }

    // Build local file path from URI
    let req_path = String::from(server.root_path) + &req.canonicalized_uri();
    let mut path = PathBuf::from(&req_path);

    if path.is_dir() {
        // Trailing slash redirect
        if !req.uri.ends_with("/") {
            let redirect_uri = req.uri.clone() + "/";
            res.status_code = 301;
            res.status_message = "Moved Permanently".into();
            res.headers.set("location", &redirect_uri);
            res.send(&stream);
            return res;
        }

        // Directory index file
        for index in &server.directory_indexes {
            if path.join(index).is_file() {
                path.push(index);
                break;
            }
        }
    }

    // Set content-type header based on file extension
    if let Some(extension) = path.extension() {
        let extension = extension.to_str().unwrap();
        if let Some(content_type) = server.content_types.get(extension) {
             res.headers.set("content-type", content_type);
        }
    }

    // Read file
    if let Err(_) = read_file(path.to_str().unwrap(), &mut res.body) {
        res.status_code = 404;
        res.status_message = "Not Found".into();
        res.send(&stream);
        return res;
    }

    if req.method == "HEAD" {
        // The HEAD method is identical to GET except that the server MUST NOT
        // return a message-body in the response.
        //
        // (RFC 2616 9.4)
        res.send_head(&stream);
    } else { // GET method
        res.send(&stream);
    }

    res
}

fn read_file(path: &str, buf: &mut Vec<u8>) -> Result<(), String> {
    match File::open(path) {
        Err(_) => return Err("Could not parse request line".into()),
        Ok(mut file) => {
            if let Err(_) = file.read_to_end(buf) {
                return Err("Could not parse request line".into())
            }
        }
    }
    Ok(())
}
