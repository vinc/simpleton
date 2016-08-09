use http::request::Request;
use http::response::Response;

/// Run after every other handlers to print a log of the request
/// and its response to stdout.
pub fn handler(req: Request, res: Response) -> Response {
    println!(
        "{} - - [{}] \"{} {} {}\" {} -",
        req.ip,
        res.date,
        req.method,
        req.uri,
        req.version,
        res.status_code
    );

    res
}
