use std::{
    env,
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::{Component, Path},
    thread,
};

mod page;

const INDEX_HTML: &str = page::INDEX_HTML;

fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&address).expect("Impossible d'ouvrir le serveur HTTP");

    println!("Portfolio Rust disponible sur http://{address}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(error) => eprintln!("Erreur de connexion: {error}"),
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 8192];
    let bytes_read = match stream.read(&mut buffer) {
        Ok(0) | Err(_) => return,
        Ok(bytes_read) => bytes_read,
    };

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let mut parts = request.split_whitespace();
    let method = parts.next().unwrap_or("");
    let target = parts.next().unwrap_or("/");

    if method != "GET" && method != "HEAD" {
        send_response(&mut stream, 405, "text/plain; charset=utf-8", b"Method Not Allowed", false);
        return;
    }

    let path = target.split(['?', '#']).next().unwrap_or("/");
    let path = match percent_decode(path) {
        Some(path) => path,
        None => {
            send_response(&mut stream, 400, "text/plain; charset=utf-8", b"Bad Request", false);
            return;
        }
    };

    if path == "/" || path == "/index.html" {
        send_response(
            &mut stream,
            200,
            "text/html; charset=utf-8",
            INDEX_HTML.as_bytes(),
            method == "HEAD",
        );
        return;
    }

    let relative_path = path.trim_start_matches('/');
    if relative_path.is_empty() {
        send_response(&mut stream, 404, "text/plain; charset=utf-8", b"Not Found", false);
        return;
    }

    let root = match env::current_dir() {
        Ok(root) => root,
        Err(_) => {
            send_response(&mut stream, 500, "text/plain; charset=utf-8", b"Server Error", false);
            return;
        }
    };
    let candidate = root.join(relative_path);

    if has_parent_component(&candidate) {
        send_response(&mut stream, 403, "text/plain; charset=utf-8", b"Forbidden", false);
        return;
    }

    let canonical = match candidate.canonicalize() {
        Ok(path) => path,
        Err(_) => {
            send_response(&mut stream, 404, "text/plain; charset=utf-8", b"Not Found", false);
            return;
        }
    };
    let canonical_root = match root.canonicalize() {
        Ok(path) => path,
        Err(_) => {
            send_response(&mut stream, 500, "text/plain; charset=utf-8", b"Server Error", false);
            return;
        }
    };

    if !canonical.starts_with(&canonical_root) || !canonical.is_file() {
        send_response(&mut stream, 404, "text/plain; charset=utf-8", b"Not Found", false);
        return;
    }

    match fs::read(&canonical) {
        Ok(contents) => send_response(
            &mut stream,
            200,
            content_type(&canonical),
            &contents,
            method == "HEAD",
        ),
        Err(_) => send_response(&mut stream, 500, "text/plain; charset=utf-8", b"Server Error", false),
    }
}

fn send_response(
    stream: &mut TcpStream,
    status: u16,
    content_type: &str,
    body: &[u8],
    head_only: bool,
) {
    let reason = match status {
        200 => "OK",
        400 => "Bad Request",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        500 => "Internal Server Error",
        _ => "Error",
    };

    let headers = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(headers.as_bytes());
    if !head_only {
        let _ = stream.write_all(body);
    }
    let _ = stream.flush();
}

fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("mp4") => "video/mp4",
        Some("pdf") => "application/pdf",
        Some("doc" | "docx") => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        _ => "application/octet-stream",
    }
}

fn percent_decode(input: &str) -> Option<String> {
    let bytes = input.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] == b'%' {
            if index + 2 >= bytes.len() {
                return None;
            }
            let high = hex_value(bytes[index + 1])?;
            let low = hex_value(bytes[index + 2])?;
            decoded.push((high << 4) | low);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }

    String::from_utf8(decoded).ok()
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn has_parent_component(path: &Path) -> bool {
    path.components()
        .any(|component| matches!(component, Component::ParentDir))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_decode_handles_utf8_paths() {
        assert_eq!(
            percent_decode("/Proc%C3%A9dure%201.docx").as_deref(),
            Some("/Procédure 1.docx")
        );
    }

    #[test]
    fn percent_decode_rejects_invalid_escape() {
        assert_eq!(percent_decode("/fichier%2"), None);
        assert_eq!(percent_decode("/fichier%GG"), None);
    }

    #[test]
    fn content_type_matches_portfolio_assets() {
        assert_eq!(content_type(Path::new("index.html")), "text/html; charset=utf-8");
        assert_eq!(content_type(Path::new("photo.png.png")), "image/png");
        assert_eq!(content_type(Path::new("video.mp4")), "video/mp4");
        assert_eq!(content_type(Path::new("CV.pdf")), "application/pdf");
    }

    #[test]
    fn rust_page_contains_main_sections() {
        assert!(INDEX_HTML.contains("<main>"));
        assert!(INDEX_HTML.contains("id=\"cv\""));
        assert!(INDEX_HTML.contains("id=\"projets\""));
        assert!(INDEX_HTML.contains("id=\"contact\""));
    }
}
