#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum HeaderName {
    Host,
    ContentType,
    ContentLength,
    UserAgent,
    Accept,
}

impl HeaderName {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "Host" => Some(HeaderName::Host),
            "User-Agent" => Some(HeaderName::UserAgent),
            "Accept" => Some(HeaderName::Accept),
            "Content-Type" => Some(HeaderName::ContentType),
            "Content-Length" => Some(HeaderName::ContentLength),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            HeaderName::Host => "Host",
            HeaderName::UserAgent => "User-Agent",
            HeaderName::Accept => "Accept",
            HeaderName::ContentType => "Content-Type",
            HeaderName::ContentLength => "Content-Length",
        }
    }
}

pub enum ContentType {
    TextPlain,
    OctetStream,
}

impl ContentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContentType::TextPlain => "text/plain",
            ContentType::OctetStream => "application/octet-stream",
        }
    }
}
