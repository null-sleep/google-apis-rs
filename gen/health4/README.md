<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-health4` library allows access to all features of the *Google Google Health API* service.

This documentation was generated from *Google Health API* crate version *7.0.0+20260701*, where *20260701* is the exact revision of the *health:v4* schema built by the [mako](http://www.makotemplates.org/) code generator *v7.0.0*.

Everything else about the *Google Health API* *v4* API can be found at the
[official documentation site](https://developers.google.com/health).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.GoogleHealthAPI.html) ...

* projects
 * [*subscribers create*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.ProjectSubscriberCreateCall.html), [*subscribers delete*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.ProjectSubscriberDeleteCall.html), [*subscribers list*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.ProjectSubscriberListCall.html), [*subscribers patch*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.ProjectSubscriberPatchCall.html), [*subscribers subscriptions create*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.ProjectSubscriberSubscriptionCreateCall.html), [*subscribers subscriptions delete*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.ProjectSubscriberSubscriptionDeleteCall.html), [*subscribers subscriptions list*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.ProjectSubscriberSubscriptionListCall.html) and [*subscribers subscriptions patch*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.ProjectSubscriberSubscriptionPatchCall.html)
* users
 * [*data types data points batch delete*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.UserDataTypeDataPointBatchDeleteCall.html), [*data types data points create*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.UserDataTypeDataPointCreateCall.html), [*data types data points daily roll up*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.UserDataTypeDataPointDailyRollUpCall.html), [*data types data points export exercise tcx*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.UserDataTypeDataPointExportExerciseTcxCall.html), [*data types data points get*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.UserDataTypeDataPointGetCall.html), [*data types data points list*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.UserDataTypeDataPointListCall.html), [*data types data points patch*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.UserDataTypeDataPointPatchCall.html), [*data types data points reconcile*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.UserDataTypeDataPointReconcileCall.html), [*data types data points roll up*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.UserDataTypeDataPointRollUpCall.html), [*get identity*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.UserGetIdentityCall.html), [*get irn profile*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.UserGetIrnProfileCall.html), [*get profile*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.UserGetProfileCall.html), [*get settings*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.UserGetSettingCall.html), [*paired devices get*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.UserPairedDeviceGetCall.html), [*paired devices list*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.UserPairedDeviceListCall.html), [*update profile*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.UserUpdateProfileCall.html) and [*update settings*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.UserUpdateSettingCall.html)


Download supported by ...

* [*data types data points export exercise tcx users*](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.UserDataTypeDataPointExportExerciseTcxCall.html)



# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-health4/7.0.0+20260701/google_health4/api/struct.GoogleHealthAPI.html)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-apis-common/latest/google_apis_common/trait.MethodsBuilder.html) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-apis-common/latest/google_apis_common/trait.CallBuilder.html)
* **[Resources](https://docs.rs/google-apis-common/latest/google_apis_common/trait.Resource.html)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-apis-common/latest/google_apis_common/trait.Part.html)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-apis-common/latest/google_apis_common/trait.CallBuilder.html)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.projects().subscribers_create(...).doit().await
let r = hub.projects().subscribers_delete(...).doit().await
let r = hub.projects().subscribers_patch(...).doit().await
let r = hub.users().data_types_data_points_batch_delete(...).doit().await
let r = hub.users().data_types_data_points_create(...).doit().await
let r = hub.users().data_types_data_points_patch(...).doit().await
```

The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities`
supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be
specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
The `doit()` method performs the actual communication with the server and returns the respective result.

# Usage

## Setting up your Project

To use this library, you would put the following lines into your `Cargo.toml` file:

```toml
[dependencies]
google-health4 = "*"
serde = "1"
serde_json = "1"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_health4 as health4;
use health4::api::CreateSubscriberPayload;
use health4::{Result, Error};
use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};

// Get an ApplicationSecret instance by some means. It contains the `client_id` and
// `client_secret`, among other things.
let secret: yup_oauth2::ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you,
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let connector = hyper_rustls::HttpsConnectorBuilder::new()
    .with_native_roots()
    .unwrap()
    .https_only()
    .enable_http2()
    .build();

let executor = hyper_util::rt::TokioExecutor::new();
let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
    secret,
    yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    yup_oauth2::client::CustomHyperClientBuilder::from(
        hyper_util::client::legacy::Client::builder(executor).build(connector),
    ),
).build().await.unwrap();

let client = hyper_util::client::legacy::Client::builder(
    hyper_util::rt::TokioExecutor::new()
)
.build(
    hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .unwrap()
        .https_or_http()
        .enable_http2()
        .build()
);
let mut hub = GoogleHealthAPI::new(client, auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = CreateSubscriberPayload::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().subscribers_create(req, "parent")
             .subscriber_id("magna")
             .doit().await;

match result {
    Err(e) => match e {
        // The Error enum provides details about what exactly happened.
        // You can also just use its `Debug`, `Display` or `Error` traits
         Error::HttpError(_)
        |Error::Io(_)
        |Error::MissingAPIKey
        |Error::MissingToken(_)
        |Error::Cancelled
        |Error::UploadSizeLimitExceeded(_, _)
        |Error::Failure(_)
        |Error::BadRequest(_)
        |Error::FieldClash(_)
        |Error::JsonDecodeError(_, _) => println!("{}", e),
    },
    Ok(res) => println!("Success: {:?}", res),
}

```
## Handling Errors

All errors produced by the system are provided either as [Result](https://docs.rs/google-health4/7.0.0+20260701/google_health4/type.Result.html) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the
[Hub Delegate](https://docs.rs/google-health4/7.0.0+20260701/google_health4/trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-health4/7.0.0+20260701/google_health4/type.Result.html), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-apis-common/latest/google_apis_common/trait.ResponseResult.html), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols:
*simple* and *resumable*. The distinctiveness of each is represented by customized
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-health4/7.0.0+20260701/google_health4/trait.Delegate.html) to the
[Method Builder](https://docs.rs/google-apis-common/latest/google_apis_common/trait.CallBuilder.html) before making the final `doit()` call.
Respective methods will be called to provide progress information, as well as determine whether the system should
retry on failure.

The [delegate trait](https://docs.rs/google-health4/7.0.0+20260701/google_health4/trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-apis-common/latest/google_apis_common/trait.RequestValue.html) and
[decodable](https://docs.rs/google-apis-common/latest/google_apis_common/trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-apis-common/latest/google_apis_common/trait.Part.html) which are identifiable by name, which will be sent to
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-apis-common/latest/google_apis_common/trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-apis-common/latest/google_apis_common/trait.RequestValue.html) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **health4** library was generated by Sebastian Thiel, and is placed
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

