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
use reqwest::multipart::Form;
use reqwest::{Client, RequestBuilder as ReqwestRequestBuilder, Url};
use tracing::instrument;

use super::routing::RouteInfo;
use super::HttpError;
use crate::constants;
use crate::internal::prelude::*;

pub struct RequestBuilder<'a>(Request<'a>);

impl<'a> RequestBuilder<'a> {
    #[must_use]
    pub const fn new(route_info: RouteInfo<'a>) -> Self {
        Self(Request {
            body: None,
            form: None,
            headers: None,
            route: route_info,
        })
    }

    #[must_use]
    pub fn build(self) -> Request<'a> {
        Request::new(self)
    }

    pub fn body(&mut self, body: Option<Vec<u8>>) -> &mut Self {
        self.0.body = body;

        self
    }

    pub fn form(&mut self, form: Option<&'a (dyn Fn() -> Form + Send + Sync + 'a)>) -> &mut Self {
        self.0.form = form;

        self
    }

    pub fn headers(&mut self, headers: Option<Headers>) -> &mut Self {
        self.0.headers = headers;

        self
    }

    pub fn route(&mut self, route_info: RouteInfo<'a>) -> &mut Self {
        self.0.route = route_info;

        self
    }
}

#[derive(Clone)]
pub struct Request<'a> {
    pub(super) body: Option<Vec<u8>>,
    // Callback, because we may need to build the form multiple times (on rate limits) and Form
    // doesn't implement Clone
    pub(super) form: Option<&'a (dyn Fn() -> Form + Send + Sync + 'a)>,
    pub(super) headers: Option<Headers>,
    pub(super) route: RouteInfo<'a>,
}

impl<'a> std::fmt::Debug for Request<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            body,
            form: _, // can't debug print closure
            headers,
            route,
        } = self;
        f.debug_struct("Request")
            .field("body", body)
            .field("headers", headers)
            .field("route", route)
            .finish()
    }
}

impl<'a> Request<'a> {
    #[must_use]
    pub fn new(builder: RequestBuilder<'a>) -> Self {
        builder.0
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
            form,
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

        if let Some(form) = form {
            // Setting multipart adds the content-length header
            builder = builder.multipart(form());
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
