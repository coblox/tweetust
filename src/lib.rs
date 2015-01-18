//! Tweetust is the simple wrapper for Twitter API.
//! # Getting started
//! This is a Twitter API wrapper, so you must lean Twitter API.
//! [Visit the official document](https://dev.twitter.com/).
//!
//! After getting the API key, let's start using tweetust.
//! # How to get the access token
//! See [request_token function](auth/fn.request_token.html).
//! After getting the access token, you can use [to_authenticator function](auth/struct.AccessTokenResult.html#method.to_authenticator)
//! to make [OAuthAuthenticator](conn/oauth_authenticator/struct.OAuthAuthenticator.html).
//! # How to create OAuthAuthenticator with an access token string
//! See [OAuthAuthenticator::new](conn/oauth_authenticator/struct.OAuthAuthenticator.html#method.new).
//! # The first tweeting
//! When you created OAuthAuthenticator and set to `auth` variable, you can tweet in a minute.
//!
//! ```rust
//! // extern crate tweetust; or use tweetust;
//! let your_tweet =
//!   tweetust::TwitterClient::new(&auth)
//!     .statuses()
//!     .update("My First Tweet!")
//!     .execute();
//! ```
//! It's easy for people who have leaned about Twitter, isn't it?

#![allow(unstable)]
#![experimental]
#![feature(box_syntax, plugin)]

//TODO: `type` field

extern crate hyper;
extern crate oauthcli;
extern crate "rustc-serialize" as rustc_serialize;
extern crate url;

#[plugin]
#[no_link]
extern crate tweetust_macros;

use std::error::{Error, FromError};
use rustc_serialize::json;
use models::TwitterResponse;

pub use auth::{access_token, request_token};
pub use clients::TwitterClient;
pub use conn::oauth_authenticator::OAuthAuthenticator;

pub mod auth;
pub mod clients;
pub mod conn;
pub mod models;

#[derive(Show)]
pub enum TwitterError {
    ErrorResponse(models::error::ErrorResponse),
    HttpError(hyper::HttpError),
    JsonError(json::DecoderError, TwitterResponse<()>),
    ParseError(TwitterResponse<()>)
}

impl Error for TwitterError {
    fn description(&self) -> &str {
        "an error occured in your request"
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            TwitterError::ErrorResponse(ref e) => Some(e),
            TwitterError::HttpError(ref e) => Some(e),
            TwitterError::JsonError(ref e, _) => Some(e),
            TwitterError::ParseError(_) => None
        }
    }
}

impl FromError<hyper::HttpError> for TwitterError {
    fn from_error(err: hyper::HttpError) -> TwitterError {
        TwitterError::HttpError(err)
    }
}

pub type TwitterResult<T> = Result<TwitterResponse<T>, TwitterError>;
