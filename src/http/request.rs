use std::borrow::Cow;
use std::convert::TryFrom;

use reqwest::header::{
    HeaderMap as Headers,
    HeaderValue,
    AUTHORIZATION,
    CONTENT_LENGTH,
    CONTENT_TYPE,
    USER_AGENT,
};
use reqwest::{Client, RequestBuilder as ReqwestRequestBuilder, Url};
use tracing::instrument;

use super::multipart::Multipart;
use super::routing::RouteInfo;
use super::HttpError;
use crate::constants;
use crate::internal::prelude::*;

#[deprecated = "use Request directly now"]
pub type RequestBuilder<'a> = Request<'a>;

#[derive(Clone, Debug)]
pub struct Request<'a> {
    pub(super) body: Option<Vec<u8>>,
    pub(super) multipart: Option<Multipart<'a>>,
    pub(super) headers: Option<Headers>,
    pub(super) route: RouteInfo<'a>,
}

impl<'a> Request<'a> {
    #[must_use]
    pub const fn new(route_info: RouteInfo<'a>) -> Self {
        Self {
            body: None,
            multipart: None,
            headers: None,
            route: route_info,
        }
    }

    pub fn body(&mut self, body: Option<Vec<u8>>) -> &mut Self {
        self.body = body;

        self
    }

    pub fn multipart(&mut self, multipart: Option<Multipart<'a>>) -> &mut Self {
        self.multipart = multipart;

        self
    }

    pub fn headers(&mut self, headers: Option<Headers>) -> &mut Self {
        self.headers = headers;

        self
    }

    pub fn route(&mut self, route_info: RouteInfo<'a>) -> &mut Self {
        self.route = route_info;

        self
    }

    #[instrument(skip(token))]
    pub fn build(
        self,
        client: &Client,
        token: &str,
        proxy: Option<&Url>,
    ) -> Result<ReqwestRequestBuilder> {
        let Request {
            body,
            multipart,
            headers: ref request_headers,
            route: ref route_info,
        } = self;

        let (method, _, mut path) = route_info.deconstruct();

        if let Some(proxy) = proxy {
            path = Cow::Owned(path.to_mut().replace("https://discord.com/", proxy.as_str()));
        }

        let mut builder =
            client.request(method.reqwest_method(), Url::parse(&path).map_err(HttpError::Url)?);

        let mut headers = Headers::with_capacity(4);
        headers.insert(USER_AGENT, HeaderValue::from_static(constants::USER_AGENT));
        headers
            .insert(AUTHORIZATION, HeaderValue::from_str(token).map_err(HttpError::InvalidHeader)?);

        // Discord will return a 400: Bad Request response if we set the content type header,
        // but don't give a body.
        if body.is_some() {
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }

        if let Some(multipart) = multipart {
            // Setting multipart adds the content-length header
            builder = builder.multipart(multipart.build_form()?);
        } else {
            let length = body
                .as_ref()
                .map(|b| HeaderValue::try_from(b.len().to_string()))
                .transpose()
                .map_err(HttpError::InvalidHeader)?;

            headers.insert(CONTENT_LENGTH, length.unwrap_or_else(|| HeaderValue::from_static("0")));
        }

        if let Some(request_headers) = request_headers.clone() {
            headers.extend(request_headers);
        }

        if let Some(bytes) = body {
            builder = builder.body(bytes);
        }

        Ok(builder.headers(headers))
    }

    #[must_use]
    pub fn body_ref(&self) -> Option<&[u8]> {
        self.body.as_deref()
    }

    #[must_use]
    pub fn body_mut(&mut self) -> Option<&mut [u8]> {
        self.body.as_deref_mut()
    }

    #[must_use]
    pub fn headers_ref(&self) -> &Option<Headers> {
        &self.headers
    }

    #[must_use]
    pub fn headers_mut(&mut self) -> &mut Option<Headers> {
        &mut self.headers
    }

    #[must_use]
    pub fn route_ref(&self) -> &RouteInfo<'_> {
        &self.route
    }

    #[must_use]
    pub fn route_mut(&mut self) -> &mut RouteInfo<'a> {
        &mut self.route
    }
}
