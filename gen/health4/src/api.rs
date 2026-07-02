#![allow(clippy::ptr_arg)]

use std::collections::{BTreeSet, HashMap};

use tokio::time::sleep;

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Clone, Copy)]
pub enum Scope {
    /// See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.
    CloudPlatform,

    /// See your Google Health activity and fitness data
    GooglehealthActivityAndFitnesReadonly,

    /// Add activity and fitness data to Google Health, and edit or delete the data it adds.
    GooglehealthActivityAndFitnesWriteonly,

    /// See your Google Health ECG data
    GooglehealthEcgReadonly,

    /// See your Google Health health metrics and measurement data
    GooglehealthHealthMetricAndMeasurementReadonly,

    /// Add health metric and measurements data to Google Health, and edit or delete the data it adds.
    GooglehealthHealthMetricAndMeasurementWriteonly,

    /// See your Google Health Irregular Rhythm Notifications data
    GooglehealthIrnReadonly,

    /// See exercise GPS location data in Google Health
    GooglehealthLocationReadonly,

    /// Add exercise GPS location data to Google Health, and edit or delete the data it adds.
    GooglehealthLocationWriteonly,

    /// Add nutrition data to Google Health, and edit or delete the data it adds.
    GooglehealthNutritionWriteonly,

    /// See your Google Health profile data
    GooglehealthProfileReadonly,

    /// Add profile data to Google Health, and edit or delete the data it adds.
    GooglehealthProfileWriteonly,

    /// See your Google Health settings
    GooglehealthSettingReadonly,

    /// Add settings data to Google Health, and edit or delete the data it adds.
    GooglehealthSettingWriteonly,

    /// See your Google Health sleep data
    GooglehealthSleepReadonly,

    /// Add sleep data to Google Health, and edit or delete the data it adds.
    GooglehealthSleepWriteonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::GooglehealthActivityAndFitnesReadonly => "https://www.googleapis.com/auth/googlehealth.activity_and_fitness.readonly",
            Scope::GooglehealthActivityAndFitnesWriteonly => "https://www.googleapis.com/auth/googlehealth.activity_and_fitness.writeonly",
            Scope::GooglehealthEcgReadonly => "https://www.googleapis.com/auth/googlehealth.ecg.readonly",
            Scope::GooglehealthHealthMetricAndMeasurementReadonly => "https://www.googleapis.com/auth/googlehealth.health_metrics_and_measurements.readonly",
            Scope::GooglehealthHealthMetricAndMeasurementWriteonly => "https://www.googleapis.com/auth/googlehealth.health_metrics_and_measurements.writeonly",
            Scope::GooglehealthIrnReadonly => "https://www.googleapis.com/auth/googlehealth.irn.readonly",
            Scope::GooglehealthLocationReadonly => "https://www.googleapis.com/auth/googlehealth.location.readonly",
            Scope::GooglehealthLocationWriteonly => "https://www.googleapis.com/auth/googlehealth.location.writeonly",
            Scope::GooglehealthNutritionWriteonly => "https://www.googleapis.com/auth/googlehealth.nutrition.writeonly",
            Scope::GooglehealthProfileReadonly => "https://www.googleapis.com/auth/googlehealth.profile.readonly",
            Scope::GooglehealthProfileWriteonly => "https://www.googleapis.com/auth/googlehealth.profile.writeonly",
            Scope::GooglehealthSettingReadonly => "https://www.googleapis.com/auth/googlehealth.settings.readonly",
            Scope::GooglehealthSettingWriteonly => "https://www.googleapis.com/auth/googlehealth.settings.writeonly",
            Scope::GooglehealthSleepReadonly => "https://www.googleapis.com/auth/googlehealth.sleep.readonly",
            Scope::GooglehealthSleepWriteonly => "https://www.googleapis.com/auth/googlehealth.sleep.writeonly",
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for Scope {
    fn default() -> Scope {
        Scope::GooglehealthActivityAndFitnesReadonly
    }
}

// ########
// HUB ###
// ######

/// Central instance to access all GoogleHealthAPI related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_health4 as health4;
/// use health4::api::CreateSubscriberPayload;
/// use health4::{Result, Error};
/// # async fn dox() {
/// use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and
/// // `client_secret`, among other things.
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you,
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let connector = hyper_rustls::HttpsConnectorBuilder::new()
///     .with_native_roots()
///     .unwrap()
///     .https_only()
///     .enable_http2()
///     .build();
///
/// let executor = hyper_util::rt::TokioExecutor::new();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     yup_oauth2::client::CustomHyperClientBuilder::from(
///         hyper_util::client::legacy::Client::builder(executor).build(connector),
///     ),
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http2()
///         .build()
/// );
/// let mut hub = GoogleHealthAPI::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CreateSubscriberPayload::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().subscribers_create(req, "parent")
///              .subscriber_id("At")
///              .doit().await;
///
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::Io(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
#[derive(Clone)]
pub struct GoogleHealthAPI<C> {
    pub client: common::Client<C>,
    pub auth: Box<dyn common::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<C> common::Hub for GoogleHealthAPI<C> {}

impl<'a, C> GoogleHealthAPI<C> {
    pub fn new<A: 'static + common::GetToken>(
        client: common::Client<C>,
        auth: A,
    ) -> GoogleHealthAPI<C> {
        GoogleHealthAPI {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/7.0.0".to_string(),
            _base_url: "https://health.googleapis.com/".to_string(),
            _root_url: "https://health.googleapis.com/".to_string(),
        }
    }

    pub fn projects(&'a self) -> ProjectMethods<'a, C> {
        ProjectMethods { hub: self }
    }
    pub fn users(&'a self) -> UserMethods<'a, C> {
        UserMethods { hub: self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/7.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: impl Into<String>) -> String {
        std::mem::replace(&mut self._user_agent, agent_name.into())
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://health.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: impl Into<String>) -> String {
        std::mem::replace(&mut self._base_url, new_base_url.into())
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://health.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: impl Into<String>) -> String {
        std::mem::replace(&mut self._root_url, new_root_url.into())
    }
}

// ############
// SCHEMAS ###
// ##########
/// Energy burned as part of an activity, excluding the basal energy burn.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActiveEnergyBurned {
    /// Required. Observed interval
    pub interval: Option<ObservationTimeInterval>,
    /// Required. Energy burned during an activity, measured in kilocalories.
    pub kcal: Option<f64>,
}

impl common::Part for ActiveEnergyBurned {}

/// Represents the result of the rollup of active energy burned.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActiveEnergyBurnedRollupValue {
    /// Output only. Sum of the active energy burned in kilocalories.
    #[serde(rename = "kcalSum")]
    pub kcal_sum: Option<f64>,
}

impl common::Part for ActiveEnergyBurnedRollupValue {}

/// Record of active minutes in a given time interval.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActiveMinutes {
    /// Required. Active minutes by activity level. At most one record per activity level is allowed.
    #[serde(rename = "activeMinutesByActivityLevel")]
    pub active_minutes_by_activity_level: Option<Vec<ActiveMinutesByActivityLevel>>,
    /// Required. Observed interval.
    pub interval: Option<ObservationTimeInterval>,
}

impl common::Part for ActiveMinutes {}

/// Active minutes at a given activity level.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActiveMinutesByActivityLevel {
    /// Required. Number of whole minutes spent in activity.
    #[serde(rename = "activeMinutes")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub active_minutes: Option<i64>,
    /// Required. The level of activity.
    #[serde(rename = "activityLevel")]
    pub activity_level: Option<String>,
}

impl common::Part for ActiveMinutesByActivityLevel {}

/// Active minutes by activity level.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActiveMinutesRollupByActivityLevel {
    /// Number of whole minutes spent in activity.
    #[serde(rename = "activeMinutesSum")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub active_minutes_sum: Option<i64>,
    /// The level of activity.
    #[serde(rename = "activityLevel")]
    pub activity_level: Option<String>,
}

impl common::Part for ActiveMinutesRollupByActivityLevel {}

/// Represents the result of the rollup of the active minutes data type.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActiveMinutesRollupValue {
    /// Active minutes by activity level. At most one record per activity level is allowed.
    #[serde(rename = "activeMinutesRollupByActivityLevel")]
    pub active_minutes_rollup_by_activity_level: Option<Vec<ActiveMinutesRollupByActivityLevel>>,
}

impl common::Part for ActiveMinutesRollupValue {}

/// Record of active zone minutes in a given time interval.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActiveZoneMinutes {
    /// Required. Number of Active Zone Minutes earned in the given time interval. Note: active_zone_minutes equals to 1 for low intensity (fat burn) zones or 2 for high intensity zones (cardio, peak).
    #[serde(rename = "activeZoneMinutes")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub active_zone_minutes: Option<i64>,
    /// Required. Heart rate zone in which the active zone minutes have been earned, in the given time interval.
    #[serde(rename = "heartRateZone")]
    pub heart_rate_zone: Option<String>,
    /// Required. Observed interval.
    pub interval: Option<ObservationTimeInterval>,
}

impl common::Part for ActiveZoneMinutes {}

/// Represents the result of the rollup of the active zone minutes data type.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActiveZoneMinutesRollupValue {
    /// Active zone minutes in `HeartRateZone.CARDIO`.
    #[serde(rename = "sumInCardioHeartZone")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub sum_in_cardio_heart_zone: Option<i64>,
    /// Active zone minutes in `HeartRateZone.FAT_BURN`.
    #[serde(rename = "sumInFatBurnHeartZone")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub sum_in_fat_burn_heart_zone: Option<i64>,
    /// Active zone minutes in `HeartRateZone.PEAK`.
    #[serde(rename = "sumInPeakHeartZone")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub sum_in_peak_heart_zone: Option<i64>,
}

impl common::Part for ActiveZoneMinutesRollupValue {}

/// Internal type to capture activity level during a certain time interval.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityLevel {
    /// Required. Activity level type in the given time interval.
    #[serde(rename = "activityLevelType")]
    pub activity_level_type: Option<String>,
    /// Required. Observed interval.
    pub interval: Option<ObservationTimeInterval>,
}

impl common::Part for ActivityLevel {}

/// Represents the total duration in a specific activity level type.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityLevelRollupByActivityLevelType {
    /// Activity level type.
    #[serde(rename = "activityLevelType")]
    pub activity_level_type: Option<String>,
    /// Total duration in the activity level type.
    #[serde(rename = "totalDuration")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub total_duration: Option<chrono::Duration>,
}

impl common::Part for ActivityLevelRollupByActivityLevelType {}

/// Represents the result of the rollup of the activity level data type.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityLevelRollupValue {
    /// List of total durations in each activity level type.
    #[serde(rename = "activityLevelRollupsByActivityLevelType")]
    pub activity_level_rollups_by_activity_level_type:
        Option<Vec<ActivityLevelRollupByActivityLevelType>>,
}

impl common::Part for ActivityLevelRollupValue {}

/// An analysis window evaluated for AFib. Note: The current version of the algorithm will only produce alerts if all windows are positive. So anything returned from the API will always have the positive bit set to true. Internally, windows can be negative, however. We never save "inconclusive" windows (they aren't produced by the algorithm).
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AlertWindow {
    /// Output only. Observed interval end time in civil time in the timezone the subject is in at the end of the observed interval
    #[serde(rename = "civilEndTime")]
    pub civil_end_time: Option<CivilDateTime>,
    /// Output only. Observed interval start time in civil time in the timezone the subject is in at the start of the observed interval
    #[serde(rename = "civilStartTime")]
    pub civil_start_time: Option<CivilDateTime>,
    /// Required. The end time of the analysis window.
    #[serde(rename = "endTime")]
    pub end_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Required. The UTC offset of the user's timezone when the analysis window ended.
    #[serde(rename = "endUtcOffset")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub end_utc_offset: Option<chrono::Duration>,
    /// Optional. All heart beats in the interval contained in this analysis window.
    #[serde(rename = "heartBeats")]
    pub heart_beats: Option<Vec<HeartBeat>>,
    /// Optional. Flag indicating whether the window was positive for AFib or not. A `true` value indicates that AFib was detected in this window. A `false` value means AFib was not detected, but does not guarantee the absence of AFib.
    pub positive: Option<bool>,
    /// Required. Observed interval. The start time of the analysis window.
    #[serde(rename = "startTime")]
    pub start_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Required. The UTC offset of the user's timezone when the analysis window started.
    #[serde(rename = "startUtcOffset")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub start_utc_offset: Option<chrono::Duration>,
}

impl common::Part for AlertWindow {}

/// Captures the altitude gain (i.e. deltas), and not level above sea, for a user in millimeters.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Altitude {
    /// Required. Altitude gain in millimeters over the observed interval.
    #[serde(rename = "gainMillimeters")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub gain_millimeters: Option<i64>,
    /// Required. Observed interval.
    pub interval: Option<ObservationTimeInterval>,
}

impl common::Part for Altitude {}

/// Represents the result of the rollup of the user's altitude.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AltitudeRollupValue {
    /// Sum of the altitude gain in millimeters.
    #[serde(rename = "gainMillimetersSum")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub gain_millimeters_sum: Option<i64>,
}

impl common::Part for AltitudeRollupValue {}

/// Optional metadata for the application that provided this data.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Application {
    /// Output only. The Google OAuth 2.0 client ID of the web application or service that recorded the data. This is the client ID used during the Google OAuth flow to obtain user credentials. This field is system-populated when the data is uploaded from Google Web API.
    #[serde(rename = "googleWebClientId")]
    pub google_web_client_id: Option<String>,
    /// Output only. A unique identifier for the mobile application that was the source of the data. This is typically the application's package name on Android (e.g., `com.google.fitbit`) or the bundle ID on iOS. This field is informational and helps trace data origin. This field is system-populated when the data is uploaded from the Fitbit mobile application, Health Connect or Health Kit.
    #[serde(rename = "packageName")]
    pub package_name: Option<String>,
    /// Output only. The client ID of the application that recorded the data. This ID is a legacy Fitbit API client ID, which is different from a Google OAuth client ID. Example format: `ABC123`. This field is system-populated and used for tracing data from legacy Fitbit API integrations. This field is system-populated when the data is uploaded from a legacy Fitbit API integration.
    #[serde(rename = "webClientId")]
    pub web_client_id: Option<String>,
}

impl common::Part for Application {}

/// Number of calories burned due to basal metabolic rate (BMR) over a period of time.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BasalEnergyBurned {
    /// Required. Observed interval.
    pub interval: Option<ObservationTimeInterval>,
    /// Required. Number of calories burned due to basal metabolic rate in kilocalories over the observed interval.
    pub kcal: Option<f64>,
}

impl common::Part for BasalEnergyBurned {}

/// Request to delete a batch of identifiable data points.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [data types data points batch delete users](UserDataTypeDataPointBatchDeleteCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BatchDeleteDataPointsRequest {
    /// Required. The names of the DataPoints to delete. A maximum of 10000 data points can be deleted in a single request.
    pub names: Option<Vec<String>>,
}

impl common::RequestValue for BatchDeleteDataPointsRequest {}

/// Represents a blood glucose level measurement. LINT: LEGACY_NAMES
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BloodGlucose {
    /// Required. Blood glucose level concentration in mg/dL.
    #[serde(rename = "bloodGlucoseMilligramsPerDeciliter")]
    pub blood_glucose_milligrams_per_deciliter: Option<f64>,
    /// Optional. Meal type of the measurement.
    #[serde(rename = "mealType")]
    pub meal_type: Option<String>,
    /// Optional. Source of the measurement.
    #[serde(rename = "measurementSource")]
    pub measurement_source: Option<String>,
    /// Optional. Timing of the measurement.
    #[serde(rename = "measurementTiming")]
    pub measurement_timing: Option<String>,
    /// Optional. Standard free-form notes captured at manual logging.
    pub notes: Option<String>,
    /// Required. The time at which blood glucose was measured.
    #[serde(rename = "sampleTime")]
    pub sample_time: Option<ObservationSampleTime>,
    /// Optional. Type of body fluid used to measure the blood glucose.
    pub specimen: Option<String>,
}

impl common::Part for BloodGlucose {}

/// Represents the result of the rollup of the blood glucose data type. LINT: LEGACY_NAMES
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BloodGlucoseRollupValue {
    /// Average blood glucose level in mg/dL.
    #[serde(rename = "bloodGlucoseMilligramsPerDeciliterAvg")]
    pub blood_glucose_milligrams_per_deciliter_avg: Option<f64>,
}

impl common::Part for BloodGlucoseRollupValue {}

/// Body fat measurement.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BodyFat {
    /// Required. Body fat percentage, in range [0, 100].
    pub percentage: Option<f64>,
    /// Required. The time at which body fat was measured.
    #[serde(rename = "sampleTime")]
    pub sample_time: Option<ObservationSampleTime>,
}

impl common::Part for BodyFat {}

/// Represents the result of the rollup of the body fat data type.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BodyFatRollupValue {
    /// Average body fat percentage.
    #[serde(rename = "bodyFatPercentageAvg")]
    pub body_fat_percentage_avg: Option<f64>,
}

impl common::Part for BodyFatRollupValue {}

/// Represents the result of the rollup of the calories in heart rate zone data type.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CaloriesInHeartRateZoneRollupValue {
    /// List of calories burned in each heart rate zone.
    #[serde(rename = "caloriesInHeartRateZones")]
    pub calories_in_heart_rate_zones: Option<Vec<CaloriesInHeartRateZoneValue>>,
}

impl common::Part for CaloriesInHeartRateZoneRollupValue {}

/// Represents the amount of kilocalories burned in a specific heart rate zone.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CaloriesInHeartRateZoneValue {
    /// The heart rate zone.
    #[serde(rename = "heartRateZone")]
    pub heart_rate_zone: Option<String>,
    /// The amount of kilocalories burned in the specified heart rate zone.
    pub kcal: Option<f64>,
}

impl common::Part for CaloriesInHeartRateZoneValue {}

/// Civil time representation similar to google.type.DateTime, but ensures that neither the timezone nor the UTC offset can be set to avoid confusion between civil and physical time queries.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CivilDateTime {
    /// Required. Calendar date.
    pub date: Option<Date>,
    /// Optional. Time of day. Defaults to the start of the day, at midnight if omitted.
    pub time: Option<TimeOfDay>,
}

impl common::Part for CivilDateTime {}

/// Counterpart of google.type.Interval, but using CivilDateTime.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CivilTimeInterval {
    /// Required. The exclusive end of the range.
    pub end: Option<CivilDateTime>,
    /// Required. The inclusive start of the range.
    pub start: Option<CivilDateTime>,
}

impl common::Part for CivilTimeInterval {}

/// Core body temperature measurement, distinct from peripheral body temperature, reflects the temperature of the body's internal organs.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CoreBodyTemperature {
    /// Optional. The unique identifier of the core body temperature measurement.
    pub id: Option<String>,
    /// Optional. The location of the core body temperature measurement.
    #[serde(rename = "measurementLocation")]
    pub measurement_location: Option<String>,
    /// Required. The time at which core body temperature was measured.
    #[serde(rename = "sampleTime")]
    pub sample_time: Option<ObservationSampleTime>,
    /// Required. The core body temperature in Celsius.
    #[serde(rename = "temperatureCelsius")]
    pub temperature_celsius: Option<f64>,
}

impl common::Part for CoreBodyTemperature {}

/// Represents the result of the rollup of the core body temperature data type.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CoreBodyTemperatureRollupValue {
    /// Average core body temperature in Celsius.
    #[serde(rename = "temperatureCelsiusAvg")]
    pub temperature_celsius_avg: Option<f64>,
    /// Maximum core body temperature in Celsius.
    #[serde(rename = "temperatureCelsiusMax")]
    pub temperature_celsius_max: Option<f64>,
    /// Minimum core body temperature in Celsius.
    #[serde(rename = "temperatureCelsiusMin")]
    pub temperature_celsius_min: Option<f64>,
}

impl common::Part for CoreBodyTemperatureRollupValue {}

/// Payload for creating a subscriber.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [subscribers create projects](ProjectSubscriberCreateCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateSubscriberPayload {
    /// Required. Authorization mechanism for the subscriber endpoint. The `secret` within this message is crucial for endpoint verification and for securing webhook notifications.
    #[serde(rename = "endpointAuthorization")]
    pub endpoint_authorization: Option<EndpointAuthorization>,
    /// Required. The full HTTPS URI where update notifications will be sent. The URI must be a valid URL and use HTTPS as the scheme. This endpoint will be verified during the `CreateSubscriber` call. See CreateSubscriber RPC documentation for verification details.
    #[serde(rename = "endpointUri")]
    pub endpoint_uri: Option<String>,
    /// Optional. Configuration for the subscriber.
    #[serde(rename = "subscriberConfigs")]
    pub subscriber_configs: Option<Vec<SubscriberConfig>>,
}

impl common::RequestValue for CreateSubscriberPayload {}

/// Payload for creating a subscription.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [subscribers subscriptions create projects](ProjectSubscriberSubscriptionCreateCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateSubscriptionPayload {
    /// Optional. Data types subscribed to.
    #[serde(rename = "dataTypes")]
    pub data_types: Option<Vec<String>>,
    /// Required. Immutable. The resource name of the user for whom this subscription is active. Format: `users/{user}` where `{user}` is the public `healthUserId` as returned by the `GetIdentity` action in the profile PAPI (see `google.devicesandservices.health.v4main.HealthProfileService.GetIdentity`).
    pub user: Option<String>,
}

impl common::RequestValue for CreateSubscriptionPayload {}

/// Represents the daily heart rate variability data type. At least one of the following fields must be set: - `average_heart_rate_variability_milliseconds` - `non_rem_heart_rate_beats_per_minute` - `entropy` - `deep_sleep_root_mean_square_of_successive_differences_milliseconds`
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DailyHeartRateVariability {
    /// Optional. A user's average heart rate variability calculated using the root mean square of successive differences (RMSSD) in times between heartbeats.
    #[serde(rename = "averageHeartRateVariabilityMilliseconds")]
    pub average_heart_rate_variability_milliseconds: Option<f64>,
    /// Required. Date (in the user's timezone) of heart rate variability measurement.
    pub date: Option<Date>,
    /// Optional. The root mean square of successive differences (RMSSD) value during deep sleep.
    #[serde(rename = "deepSleepRootMeanSquareOfSuccessiveDifferencesMilliseconds")]
    pub deep_sleep_root_mean_square_of_successive_differences_milliseconds: Option<f64>,
    /// Optional. The Shanon entropy of heartbeat intervals. Entropy quantifies randomness or disorder in a system. High entropy indicates high HRV. Entropy is measured from the histogram of time interval between successive heart beats values measured during sleep.
    pub entropy: Option<f64>,
    /// Optional. Non-REM heart rate
    #[serde(rename = "nonRemHeartRateBeatsPerMinute")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub non_rem_heart_rate_beats_per_minute: Option<i64>,
}

impl common::Part for DailyHeartRateVariability {}

/// User's heart rate zone thresholds based on the Karvonen algorithm for a specific day.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DailyHeartRateZones {
    /// Required. Date (in user's timezone) of the heart rate zones record.
    pub date: Option<Date>,
    /// Required. The heart rate zones.
    #[serde(rename = "heartRateZones")]
    pub heart_rate_zones: Option<Vec<HeartRateZone>>,
}

impl common::Part for DailyHeartRateZones {}

/// A daily oxygen saturation (SpO2) record. Represents the user's daily oxygen saturation summary, typically calculated during sleep.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DailyOxygenSaturation {
    /// Required. The average value of the oxygen saturation samples during the sleep.
    #[serde(rename = "averagePercentage")]
    pub average_percentage: Option<f64>,
    /// Required. Date (in user's timezone) of the daily oxygen saturation record.
    pub date: Option<Date>,
    /// Required. The lower bound of the confidence interval of oxygen saturation samples during sleep.
    #[serde(rename = "lowerBoundPercentage")]
    pub lower_bound_percentage: Option<f64>,
    /// Optional. Standard deviation of the daily oxygen saturation averages from the past 7-30 days.
    #[serde(rename = "standardDeviationPercentage")]
    pub standard_deviation_percentage: Option<f64>,
    /// Required. The upper bound of the confidence interval of oxygen saturation samples during sleep.
    #[serde(rename = "upperBoundPercentage")]
    pub upper_bound_percentage: Option<f64>,
}

impl common::Part for DailyOxygenSaturation {}

/// A daily average respiratory rate (breaths per minute) for a day of the year. One data point per day calculated for the main sleep.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DailyRespiratoryRate {
    /// Required. The average number of breaths taken per minute.
    #[serde(rename = "breathsPerMinute")]
    pub breaths_per_minute: Option<f64>,
    /// Required. The date on which the respiratory rate was measured.
    pub date: Option<Date>,
}

impl common::Part for DailyRespiratoryRate {}

/// Measures the daily resting heart rate for a user, calculated using the all day heart rate measurements.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DailyRestingHeartRate {
    /// Required. The resting heart rate value in beats per minute.
    #[serde(rename = "beatsPerMinute")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub beats_per_minute: Option<i64>,
    /// Optional. Metadata for the daily resting heart rate.
    #[serde(rename = "dailyRestingHeartRateMetadata")]
    pub daily_resting_heart_rate_metadata: Option<DailyRestingHeartRateMetadata>,
    /// Required. Date (in the user's timezone) of the resting heart rate measurement.
    pub date: Option<Date>,
}

impl common::Part for DailyRestingHeartRate {}

/// Metadata for the daily resting heart rate.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DailyRestingHeartRateMetadata {
    /// Required. The method used to calculate the resting heart rate.
    #[serde(rename = "calculationMethod")]
    pub calculation_method: Option<String>,
}

impl common::Part for DailyRestingHeartRateMetadata {}

/// Request to roll up data points by civil time intervals.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [data types data points daily roll up users](UserDataTypeDataPointDailyRollUpCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DailyRollUpDataPointsRequest {
    /// Optional. The data source family name to roll up. If empty, data points from all available data sources will be rolled up. Format: `users/me/dataSourceFamilies/{data_source_family}` The supported values are: - `users/me/dataSourceFamilies/all-sources` - default value - `users/me/dataSourceFamilies/google-wearables` - tracker devices - `users/me/dataSourceFamilies/google-sources` - Google first party sources
    #[serde(rename = "dataSourceFamily")]
    pub data_source_family: Option<String>,
    /// Optional. The maximum number of data points to return. If unspecified, at most 1440 data points will be returned. The maximum page size is 10000; values above that will be truncated accordingly.
    #[serde(rename = "pageSize")]
    pub page_size: Option<i32>,
    /// Optional. The `next_page_token` from a previous request, if any. All other request fields need to be the same as in the initial request when the page token is specified.
    #[serde(rename = "pageToken")]
    pub page_token: Option<String>,
    /// Required. Closed-open range of data points that will be rolled up. The start time must be aligned with the aggregation window. The maximum range for `calories-in-heart-rate-zone`, `heart-rate`, `active-minutes` and `total-calories` is 14 days. The maximum range for all other data types is 90 days.
    pub range: Option<CivilTimeInterval>,
    /// Optional. Aggregation window size, in number of days. Defaults to 1 if not specified.
    #[serde(rename = "windowSizeDays")]
    pub window_size_days: Option<i32>,
}

impl common::RequestValue for DailyRollUpDataPointsRequest {}

/// Response containing the list of rolled up data points.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [data types data points daily roll up users](UserDataTypeDataPointDailyRollUpCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DailyRollUpDataPointsResponse {
    /// Values for each aggregation time window.
    #[serde(rename = "rollupDataPoints")]
    pub rollup_data_points: Option<Vec<DailyRollupDataPoint>>,
}

impl common::ResponseResult for DailyRollUpDataPointsResponse {}

/// Value of a daily rollup for a single civil time interval (aggregation window) of reconciled data points from all data sources, excluding those data points that are identified as recorded by wearables in intervals when they were not actually worn.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DailyRollupDataPoint {
    /// Returned by default when rolling up data points from the `active-energy-burned` data type.
    #[serde(rename = "activeEnergyBurned")]
    pub active_energy_burned: Option<ActiveEnergyBurnedRollupValue>,
    /// Returned by default when rolling up data points from the `active-minutes` data type, or when requested explicitly using the `active-minutes` rollup type identifier.
    #[serde(rename = "activeMinutes")]
    pub active_minutes: Option<ActiveMinutesRollupValue>,
    /// Returned by default when rolling up data points from the `active-zone-minutes` data type, or when requested explicitly using the `active-zone-minutes` rollup type identifier.
    #[serde(rename = "activeZoneMinutes")]
    pub active_zone_minutes: Option<ActiveZoneMinutesRollupValue>,
    /// Returned by default when rolling up data points from the `activity-level` data type, or when requested explicitly using the `activity-level` rollup type identifier.
    #[serde(rename = "activityLevel")]
    pub activity_level: Option<ActivityLevelRollupValue>,
    /// Returned by default when rolling up data points from the `altitude` data type, or when requested explicitly using the `altitude` rollup type identifier.
    pub altitude: Option<AltitudeRollupValue>,
    /// Returned by default when rolling up data points from the `blood-glucose` data type.
    #[serde(rename = "bloodGlucose")]
    pub blood_glucose: Option<BloodGlucoseRollupValue>,
    /// Returned by default when rolling up data points from the `body-fat` data type, or when requested explicitly using the `body-fat` rollup type identifier.
    #[serde(rename = "bodyFat")]
    pub body_fat: Option<BodyFatRollupValue>,
    /// Returned by default when rolling up data points from the `calories-in-heart-rate-zone` data type, or when requested explicitly using the `calories-in-heart-rate-zone` rollup type identifier.
    #[serde(rename = "caloriesInHeartRateZone")]
    pub calories_in_heart_rate_zone: Option<CaloriesInHeartRateZoneRollupValue>,
    /// End time of the window this value aggregates over
    #[serde(rename = "civilEndTime")]
    pub civil_end_time: Option<CivilDateTime>,
    /// Start time of the window this value aggregates over
    #[serde(rename = "civilStartTime")]
    pub civil_start_time: Option<CivilDateTime>,
    /// Returned by default when rolling up data points from the `core-body-temperature` data type, or when requested explicitly using the `core-body-temperature` rollup type identifier.
    #[serde(rename = "coreBodyTemperature")]
    pub core_body_temperature: Option<CoreBodyTemperatureRollupValue>,
    /// Returned by default when rolling up data points from the `distance` data type, or when requested explicitly using the `distance` rollup type identifier.
    pub distance: Option<DistanceRollupValue>,
    /// Returned by default when rolling up data points from the `floors` data type, or when requested explicitly using the `floors` rollup type identifier.
    pub floors: Option<FloorsRollupValue>,
    /// Returned by default when rolling up data points from the `heart-rate` data type, or when requested explicitly using the `heart-rate` rollup type identifier.
    #[serde(rename = "heartRate")]
    pub heart_rate: Option<HeartRateRollupValue>,
    /// Returned by default when rolling up data points from the `daily-heart-rate-variability` data type, or when requested explicitly using the `heart-rate-variability-personal-range` rollup type identifier.
    #[serde(rename = "heartRateVariabilityPersonalRange")]
    pub heart_rate_variability_personal_range: Option<HeartRateVariabilityPersonalRangeRollupValue>,
    /// Returned by default when rolling up data points from the `hydration-log` data type, or when requested explicitly using the `hydration-log` rollup type identifier.
    #[serde(rename = "hydrationLog")]
    pub hydration_log: Option<HydrationLogRollupValue>,
    /// Returned by default when rolling up data points from the `nutrition-log` data type, or when requested explicitly using the `nutrition-log` rollup type identifier.
    #[serde(rename = "nutritionLog")]
    pub nutrition_log: Option<NutritionLogRollupValue>,
    /// Returned by default when rolling up data points from the `daily-resting-heart-rate` data type, or when requested explicitly using the `resting-heart-rate-personal-range` rollup type identifier.
    #[serde(rename = "restingHeartRatePersonalRange")]
    pub resting_heart_rate_personal_range: Option<RestingHeartRatePersonalRangeRollupValue>,
    /// Returned by default when rolling up data points from the `run-vo2-max` data type, or when requested explicitly using the `run-vo2-max` rollup type identifier.
    #[serde(rename = "runVo2Max")]
    pub run_vo2_max: Option<RunVO2MaxRollupValue>,
    /// Returned by default when rolling up data points from the `sedentary-period` data type, or when requested explicitly using the `sedentary-period` rollup type identifier.
    #[serde(rename = "sedentaryPeriod")]
    pub sedentary_period: Option<SedentaryPeriodRollupValue>,
    /// Returned by default when rolling up data points from the `steps` data type, or when requested explicitly using the `steps` rollup type identifier.
    pub steps: Option<StepsRollupValue>,
    /// Returned by default when rolling up data points from the `swim-lengths-data` data type, or when requested explicitly using the `swim-lengths-data` rollup type identifier.
    #[serde(rename = "swimLengthsData")]
    pub swim_lengths_data: Option<SwimLengthsDataRollupValue>,
    /// Returned by default when rolling up data points from the `time-in-heart-rate-zone` data type, or when requested explicitly using the `time-in-heart-rate-zone` rollup type identifier.
    #[serde(rename = "timeInHeartRateZone")]
    pub time_in_heart_rate_zone: Option<TimeInHeartRateZoneRollupValue>,
    /// Returned by default when rolling up data points from the `total-calories` data type, or when requested explicitly using the `total-calories` rollup type identifier.
    #[serde(rename = "totalCalories")]
    pub total_calories: Option<TotalCaloriesRollupValue>,
    /// Returned by default when rolling up data points from the `weight` data type, or when requested explicitly using the `weight` rollup type identifier.
    pub weight: Option<WeightRollupValue>,
}

impl common::Part for DailyRollupDataPoint {}

/// Provides derived sleep temperature values, calculated from skin or internal device temperature readings during sleep.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DailySleepTemperatureDerivations {
    /// Optional. The user's baseline skin temperature. It is the median of the user's nightly skin temperature over the past 30 days.
    #[serde(rename = "baselineTemperatureCelsius")]
    pub baseline_temperature_celsius: Option<f64>,
    /// Required. Date for which the sleep temperature derivations are calculated.
    pub date: Option<Date>,
    /// Required. The user's nightly skin temperature. It is the mean of skin temperature samples taken from the user’s sleep.
    #[serde(rename = "nightlyTemperatureCelsius")]
    pub nightly_temperature_celsius: Option<f64>,
    /// Optional. The standard deviation of the user’s relative nightly skin temperature (temperature - baseline) over the past 30 days.
    #[serde(rename = "relativeNightlyStddev30dCelsius")]
    pub relative_nightly_stddev30d_celsius: Option<f64>,
}

impl common::Part for DailySleepTemperatureDerivations {}

/// Contains a daily summary of the user's VO2 max (cardio fitness score), which is the maximum rate of oxygen the body can use during exercise.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DailyVO2Max {
    /// Optional. Represents the user's cardio fitness level based on their VO2 max.
    #[serde(rename = "cardioFitnessLevel")]
    pub cardio_fitness_level: Option<String>,
    /// Required. The date for which the Daily VO2 max was measured.
    pub date: Option<Date>,
    /// Optional. An estimated field is added to indicate when the confidence has decreased sufficiently to consider the value an estimation.
    pub estimated: Option<bool>,
    /// Required. Daily VO2 max value measured as in ml consumed oxygen / kg of body weight / min.
    #[serde(rename = "vo2Max")]
    pub vo2_max: Option<f64>,
    /// Optional. The covariance of the VO2 max value.
    #[serde(rename = "vo2MaxCovariance")]
    pub vo2_max_covariance: Option<f64>,
}

impl common::Part for DailyVO2Max {}

/// A computed or recorded metric.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [data types data points create users](UserDataTypeDataPointCreateCall) (request)
/// * [data types data points get users](UserDataTypeDataPointGetCall) (response)
/// * [data types data points patch users](UserDataTypeDataPointPatchCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DataPoint {
    /// Optional. Data for points in the `active-energy-burned` interval data type collection.
    #[serde(rename = "activeEnergyBurned")]
    pub active_energy_burned: Option<ActiveEnergyBurned>,
    /// Optional. Data for points in the `active-minutes` interval data type collection.
    #[serde(rename = "activeMinutes")]
    pub active_minutes: Option<ActiveMinutes>,
    /// Optional. Data for points in the `active-zone-minutes` interval data type collection, measured in minutes.
    #[serde(rename = "activeZoneMinutes")]
    pub active_zone_minutes: Option<ActiveZoneMinutes>,
    /// Optional. Data for points in the `activity-level` daily data type collection.
    #[serde(rename = "activityLevel")]
    pub activity_level: Option<ActivityLevel>,
    /// Optional. Data for points in the `altitude` interval data type collection.
    pub altitude: Option<Altitude>,
    /// Optional. Data for points in the `basal-energy-burned` interval data type collection.
    #[serde(rename = "basalEnergyBurned")]
    pub basal_energy_burned: Option<BasalEnergyBurned>,
    /// Optional. Data for points in the `blood-glucose` sample data type collection.
    #[serde(rename = "bloodGlucose")]
    pub blood_glucose: Option<BloodGlucose>,
    /// Optional. Data for points in the `body-fat` sample data type collection.
    #[serde(rename = "bodyFat")]
    pub body_fat: Option<BodyFat>,
    /// Optional. Data for points in the `core-body-temperature` sample data type collection.
    #[serde(rename = "coreBodyTemperature")]
    pub core_body_temperature: Option<CoreBodyTemperature>,
    /// Optional. Data for points in the `daily-heart-rate-variability` daily data type collection.
    #[serde(rename = "dailyHeartRateVariability")]
    pub daily_heart_rate_variability: Option<DailyHeartRateVariability>,
    /// Optional. Data for points in the `daily-heart-rate-zones` daily data type collection.
    #[serde(rename = "dailyHeartRateZones")]
    pub daily_heart_rate_zones: Option<DailyHeartRateZones>,
    /// Optional. Data for points in the `daily-oxygen-saturation` daily data type collection.
    #[serde(rename = "dailyOxygenSaturation")]
    pub daily_oxygen_saturation: Option<DailyOxygenSaturation>,
    /// Optional. Data for points in the `daily-respiratory-rate` daily data type collection.
    #[serde(rename = "dailyRespiratoryRate")]
    pub daily_respiratory_rate: Option<DailyRespiratoryRate>,
    /// Optional. Data for points in the `daily-resting-heart-rate` daily data type collection.
    #[serde(rename = "dailyRestingHeartRate")]
    pub daily_resting_heart_rate: Option<DailyRestingHeartRate>,
    /// Optional. Data for points in the `daily-sleep-temperature-derivations` daily data type collection.
    #[serde(rename = "dailySleepTemperatureDerivations")]
    pub daily_sleep_temperature_derivations: Option<DailySleepTemperatureDerivations>,
    /// Optional. Data for points in the `daily-vo2-max` daily data type collection.
    #[serde(rename = "dailyVo2Max")]
    pub daily_vo2_max: Option<DailyVO2Max>,
    /// Optional. Data source information for the metric
    #[serde(rename = "dataSource")]
    pub data_source: Option<DataSource>,
    /// Optional. Data for points in the `distance` interval data type collection.
    pub distance: Option<Distance>,
    /// Optional. Data for points in the `electrocardiogram` session data type collection.
    pub electrocardiogram: Option<Electrocardiogram>,
    /// Optional. Data for points in the `exercise` session data type collection.
    pub exercise: Option<Exercise>,
    /// Optional. Data for points in the `floors` interval data type collection.
    pub floors: Option<Floors>,
    /// Optional. The food details.
    pub food: Option<Food>,
    /// Optional. The food measurement unit details.
    #[serde(rename = "foodMeasurementUnit")]
    pub food_measurement_unit: Option<FoodMeasurementUnit>,
    /// Optional. Data for points in the `heart-rate` sample data type collection.
    #[serde(rename = "heartRate")]
    pub heart_rate: Option<HeartRate>,
    /// Optional. Data for points in the `heart-rate-variability` sample data type collection.
    #[serde(rename = "heartRateVariability")]
    pub heart_rate_variability: Option<HeartRateVariability>,
    /// Optional. Data for points in the `height` sample data type collection.
    pub height: Option<Height>,
    /// Optional. Data for points in the `hydration-log` session data type collection.
    #[serde(rename = "hydrationLog")]
    pub hydration_log: Option<HydrationLog>,
    /// Optional. Data for points in the `irregular-rhythm-notification` session data type collection.
    #[serde(rename = "irregularRhythmNotification")]
    pub irregular_rhythm_notification: Option<IrregularRhythmNotification>,
    /// Identifier. Data point name, only supported for the subset of identifiable data types. For the majority of the data types, individual data points do not need to be identified and this field would be empty. Format: `users/{user}/dataTypes/{data_type}/dataPoints/{data_point}` Example: `users/abcd1234/dataTypes/sleep/dataPoints/a1b2c3d4-e5f6-7890-1234-567890abcdef` The `{user}` ID is a system-generated identifier, as described in Identity.health_user_id. The `{data_type}` ID corresponds to the kebab-case version of the field names in the DataPoint data union field, e.g. `heart-rate` for the `heart_rate` field. The `{data_point}` ID can be client-provided or system-generated. If client-provided, it must be a string of 4-63 characters, containing only lowercase letters, numbers, and hyphens.
    pub name: Option<String>,
    /// Optional. Data for points in the `nutrition-log` session data type collection.
    #[serde(rename = "nutritionLog")]
    pub nutrition_log: Option<NutritionLog>,
    /// Optional. Data for points in the `oxygen-saturation` sample data type collection.
    #[serde(rename = "oxygenSaturation")]
    pub oxygen_saturation: Option<OxygenSaturation>,
    /// Optional. Data for points in the `respiratory-rate-sleep-summary` sample data type collection.
    #[serde(rename = "respiratoryRateSleepSummary")]
    pub respiratory_rate_sleep_summary: Option<RespiratoryRateSleepSummary>,
    /// Optional. Data for points in the `run-vo2-max` sample data type collection.
    #[serde(rename = "runVo2Max")]
    pub run_vo2_max: Option<RunVO2Max>,
    /// Optional. Data for points in the `sedentary-period` interval data type collection.
    #[serde(rename = "sedentaryPeriod")]
    pub sedentary_period: Option<SedentaryPeriod>,
    /// Optional. Data for points in the `sleep` session data type collection.
    pub sleep: Option<Sleep>,
    /// Optional. Data for points in the `steps` interval data type collection.
    pub steps: Option<Steps>,
    /// Optional. Data for points in the `swim-lengths-data` interval data type collection.
    #[serde(rename = "swimLengthsData")]
    pub swim_lengths_data: Option<SwimLengthsData>,
    /// Optional. Data for points in the `time-in-heart-rate-zone` interval data type collection.
    #[serde(rename = "timeInHeartRateZone")]
    pub time_in_heart_rate_zone: Option<TimeInHeartRateZone>,
    /// Optional. Data for points in the `vo2-max` sample data type collection.
    #[serde(rename = "vo2Max")]
    pub vo2_max: Option<VO2Max>,
    /// Optional. Data for points in the `weight` sample data type collection.
    pub weight: Option<Weight>,
}

impl common::RequestValue for DataPoint {}
impl common::ResponseResult for DataPoint {}

/// Data Source definition to track the origin of data. Each health data point, regardless of the complexity or data model (whether a simple step count or a detailed sleep session) must retain information about its source of origin (e.g. the device or app that collected it).
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DataSource {
    /// Output only. Captures metadata for the application that provided this data.
    pub application: Option<Application>,
    /// Optional. Captures metadata for raw data points originating from devices. We expect this data source to be used for data points written on device sync.
    pub device: Option<Device>,
    /// Output only. Captures the platform that uploaded the data.
    pub platform: Option<String>,
    /// Optional. Captures how the data was recorded.
    #[serde(rename = "recordingMethod")]
    pub recording_method: Option<String>,
}

impl common::Part for DataSource {}

/// Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values. * A month and day, with a zero year (for example, an anniversary). * A year on its own, with a zero month and a zero day. * A year and month, with a zero day (for example, a credit card expiration date). Related types: * google.type.TimeOfDay * google.type.DateTime * google.protobuf.Timestamp
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Date {
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    pub day: Option<i32>,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    pub month: Option<i32>,
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    pub year: Option<i32>,
}

impl common::Part for Date {}

/// Captures metadata about the device that recorded the measurement.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Device {
    /// Optional. An optional name for the device.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// Optional. Captures the form factor of the device.
    #[serde(rename = "formFactor")]
    pub form_factor: Option<String>,
    /// Optional. An optional manufacturer of the device.
    pub manufacturer: Option<String>,
}

impl common::Part for Device {}

/// Distance traveled over an interval of time.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Distance {
    /// Required. Observed interval.
    pub interval: Option<ObservationTimeInterval>,
    /// Required. Distance in millimeters over the observed interval.
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub millimeters: Option<i64>,
}

impl common::Part for Distance {}

/// Result of the rollup of the user's distance.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DistanceRollupValue {
    /// Sum of the distance in millimeters.
    #[serde(rename = "millimetersSum")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub millimeters_sum: Option<i64>,
}

impl common::Part for DistanceRollupValue {}

/// Represents an Electrocardiogram (ECG) measurement session. This data type is based on SaMD feature and any changes to it may require additional review.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Electrocardiogram {
    /// Optional. Average heart rate recorded during ECG reading in beats per minute.
    #[serde(rename = "beatsPerMinuteAvg")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub beats_per_minute_avg: Option<i64>,
    /// Required. Observed interval. NOTE: Historical ECG data lacks timezone offsets, so `start_utc_offset` and `end_utc_offset` will be missing or default to zero. As a result, the civil time fields within this interval will default to UTC. It is recommended to use physical time fields instead for accurate time referencing. NOTE: The `start_time` and `end_time` of the interval are equal, representing the reading time.
    pub interval: Option<SessionTimeInterval>,
    /// Optional. The number of leads used for ECG reading.
    #[serde(rename = "leadNumber")]
    pub lead_number: Option<i32>,
    /// Output only. The meta information for the compatible device used to conduct the measurement. ECG measurements typically populate `firmware_version`, `feature_version`, and `device_model`.
    #[serde(rename = "medicalDeviceInfo")]
    pub medical_device_info: Option<MedicalDeviceInfo>,
    /// Optional. The factor by which to divide waveform samples to get voltage in millivolts: millivolts = waveform_sample / millivolts_scaling_factor.
    #[serde(rename = "millivoltsScalingFactor")]
    pub millivolts_scaling_factor: Option<i32>,
    /// Optional. The result classification of the ECG reading.
    #[serde(rename = "resultClassification")]
    pub result_classification: Option<String>,
    /// Optional. The sampling frequency of waveform samples in hertz.
    #[serde(rename = "samplingFrequencyHertz")]
    pub sampling_frequency_hertz: Option<i32>,
    /// Optional. An array of voltage values representing lead I ECG values. Each sample represents voltage difference in ECG graph. The first value in array corresponds to the start of the reading.
    #[serde(rename = "waveformSamples")]
    pub waveform_samples: Option<Vec<i32>>,
}

impl common::Part for Electrocardiogram {}

/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [subscribers subscriptions delete projects](ProjectSubscriberSubscriptionDeleteCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Empty {
    _never_set: Option<bool>,
}

impl common::ResponseResult for Empty {}

/// Authorization mechanism for a subscriber endpoint. For all requests sent by the Webhooks service, the JSON payload is cryptographically signed. The signature is delivered in the `X-HEALTHAPI-SIGNATURE` HTTP header. This is an ECDSA (NIST P256) signature of the JSON payload. Clients must verify this signature using Google Health API's public key to confirm the payload was sent by the Health API.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EndpointAuthorization {
    /// Required. Input only. Provides a client-provided secret that will be sent with each notification to the subscriber endpoint using the "Authorization" header. The value must include the authorization scheme, e.g., "Bearer " or "Basic ", as it will be used as the full Authorization header value. This secret is used by the API to test the endpoint during `CreateSubscriber` and `UpdateSubscriber` calls, and will be sent in the `Authorization` header for all subsequent webhook notifications to this endpoint.
    pub secret: Option<String>,
    /// Output only. Whether the secret is set.
    #[serde(rename = "secretSet")]
    pub secret_set: Option<bool>,
}

impl common::Part for EndpointAuthorization {}

/// Represents the energy quantity.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EnergyQuantity {
    /// Required. Value representing the energy in kilocalories.
    pub kcal: Option<f64>,
    /// Optional. Value representing the user provided unit.
    #[serde(rename = "userProvidedUnit")]
    pub user_provided_unit: Option<String>,
}

impl common::Part for EnergyQuantity {}

/// Rollup for the energy quantity.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EnergyQuantityRollup {
    /// Required. The sum of the energy in kilocalories.
    #[serde(rename = "kcalSum")]
    pub kcal_sum: Option<f64>,
    /// Optional. The user provided unit on the last element.
    #[serde(rename = "userProvidedUnitLast")]
    pub user_provided_unit_last: Option<String>,
}

impl common::Part for EnergyQuantityRollup {}

/// An exercise that stores information about a physical activity.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Exercise {
    /// Optional. Duration excluding pauses.
    #[serde(rename = "activeDuration")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub active_duration: Option<chrono::Duration>,
    /// Output only. Represents the timestamp of the creation of the exercise.
    #[serde(rename = "createTime")]
    pub create_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Required. Exercise display name.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// Optional. Exercise events that happen during an exercise, such as pause & restarts.
    #[serde(rename = "exerciseEvents")]
    pub exercise_events: Option<Vec<ExerciseEvent>>,
    /// Optional. Additional exercise metadata.
    #[serde(rename = "exerciseMetadata")]
    pub exercise_metadata: Option<ExerciseMetadata>,
    /// Required. The type of activity performed during an exercise.
    #[serde(rename = "exerciseType")]
    pub exercise_type: Option<String>,
    /// Required. Observed exercise interval
    pub interval: Option<SessionTimeInterval>,
    /// Required. Summary metrics for this exercise ( )
    #[serde(rename = "metricsSummary")]
    pub metrics_summary: Option<MetricsSummary>,
    /// Optional. Standard free-form notes captured at manual logging.
    pub notes: Option<String>,
    /// Optional. Laps or splits recorded within an exercise. Laps could be split based on distance or other criteria (duration, etc.) Laps should not be overlapping with each other.
    #[serde(rename = "splitSummaries")]
    pub split_summaries: Option<Vec<SplitSummary>>,
    /// Optional. The default split is 1 km or 1 mile. - if the movement distance is less than the default, then there are no splits - if the movement distance is greater than or equal to the default, then we have splits
    pub splits: Option<Vec<SplitSummary>>,
    /// Output only. This is the timestamp of the last update to the exercise.
    #[serde(rename = "updateTime")]
    pub update_time: Option<chrono::DateTime<chrono::offset::Utc>>,
}

impl common::Part for Exercise {}

/// Represents instantaneous events that happen during an exercise, such as start, stop, pause, split.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ExerciseEvent {
    /// Required. Exercise event time
    #[serde(rename = "eventTime")]
    pub event_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Required. Exercise event time offset from UTC
    #[serde(rename = "eventUtcOffset")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub event_utc_offset: Option<chrono::Duration>,
    /// Required. The type of the event, such as start, stop, pause, resume.
    #[serde(rename = "exerciseEventType")]
    pub exercise_event_type: Option<String>,
}

impl common::Part for ExerciseEvent {}

/// Additional exercise metadata.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ExerciseMetadata {
    /// Optional. Whether the exercise had GPS tracking.
    #[serde(rename = "hasGps")]
    pub has_gps: Option<bool>,
    /// Optional. Pool length in millimeters. Only present in the swimming exercises.
    #[serde(rename = "poolLengthMillimeters")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub pool_length_millimeters: Option<i64>,
}

impl common::Part for ExerciseMetadata {}

/// Represents a Response for exporting exercise data in TCX format.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [data types data points export exercise tcx users](UserDataTypeDataPointExportExerciseTcxCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ExportExerciseTcxResponse {
    /// Contains the exported TCX data. This field is intended for gRPC clients, as media download integration is not supported for gRPC. HTTP clients should instead use the `alt=media` query parameter to download the raw binary TCX file.
    #[serde(rename = "tcxData")]
    pub tcx_data: Option<String>,
}

impl common::ResponseResult for ExportExerciseTcxResponse {}

/// Gained elevation measured in floors over the time interval
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Floors {
    /// Required. Number of floors in the recorded interval
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// Required. Observed interval
    pub interval: Option<ObservationTimeInterval>,
}

impl common::Part for Floors {}

/// Represents the result of the rollup of the user's floors.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FloorsRollupValue {
    /// Sum of the floors count.
    #[serde(rename = "countSum")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub count_sum: Option<i64>,
}

impl common::Part for FloorsRollupValue {}

/// Represents a food item.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Food {
    /// Required. The access level of the food.
    #[serde(rename = "accessLevel")]
    pub access_level: Option<String>,
    /// Optional. The brand of the food.
    pub brand: Option<String>,
    /// Required. Value representing the default serving of the food.
    #[serde(rename = "defaultServing")]
    pub default_serving: Option<FoodServing>,
    /// Optional. The description of the food.
    pub description: Option<String>,
    /// Required. The display name of the food.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// Optional. Value representing the average energy of the food for the default serving.
    #[serde(rename = "energyAvg")]
    pub energy_avg: Option<EnergyQuantity>,
    /// Optional. Value representing the energy from fat of the food for the default serving.
    #[serde(rename = "energyFromFat")]
    pub energy_from_fat: Option<EnergyQuantity>,
    /// Optional. Value representing the maximum energy of the food for the default serving.
    #[serde(rename = "energyMax")]
    pub energy_max: Option<EnergyQuantity>,
    /// Optional. Value representing the minimum energy of the food for the default serving.
    #[serde(rename = "energyMin")]
    pub energy_min: Option<EnergyQuantity>,
    /// Optional. The language code where the food is available in format xx-XX. Supported values are defined in Settings.food_language_code.
    #[serde(rename = "languageCode")]
    pub language_code: Option<String>,
    /// Optional. The meal type associated with this food.
    #[serde(rename = "mealType")]
    pub meal_type: Option<String>,
    /// Optional. Value representing the nutrients of the food for the default serving.
    pub nutrients: Option<Vec<NutrientQuantity>>,
    /// Optional. The serving of the food.
    pub servings: Option<Vec<FoodServing>>,
    /// Optional. Value representing the total carbohydrate of the food for the default serving.
    #[serde(rename = "totalCarbohydrate")]
    pub total_carbohydrate: Option<WeightQuantity>,
    /// Optional. Value representing the total fat of the food for the default serving.
    #[serde(rename = "totalFat")]
    pub total_fat: Option<WeightQuantity>,
}

impl common::Part for Food {}

/// Represents a food measurement unit.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FoodMeasurementUnit {
    /// Required. The display name of the food measurement unit (e.g., "gram", "piece").
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// Optional. The plural display name of the food measurement unit (e.g., "grams", "pieces").
    #[serde(rename = "pluralDisplayName")]
    pub plural_display_name: Option<String>,
}

impl common::Part for FoodMeasurementUnit {}

/// Represents different properties and information about the serving of a specific food.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FoodServing {
    /// Optional. Amount of food consumed, fractional values are supported.
    pub amount: Option<f64>,
    /// Required. Food measurement unit
    #[serde(rename = "foodMeasurementUnit")]
    pub food_measurement_unit: Option<String>,
    /// Output only. Legacy measurement unit for serving size in singular form (e.g. "piece", "gram").
    #[serde(rename = "foodMeasurementUnitDisplayName")]
    pub food_measurement_unit_display_name: Option<String>,
    /// Output only. Legacy measurement unit for serving size in plural form (e.g. "pieces", "grams").
    #[serde(rename = "foodMeasurementUnitDisplayNamePlural")]
    pub food_measurement_unit_display_name_plural: Option<String>,
    /// Optional. Value representing the multiplier used to compute the energy when using this serving instead of the default serving.
    pub multiplier: Option<f64>,
}

impl common::Part for FoodServing {}

/// A single heart beat measurement.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct HeartBeat {
    /// Required. The beats-per-minute value extrapolated from the time before the following heart beat. This is calculated as 60000 / rr, where rr is the gap between heart beats in milliseconds (IBI - Interbeat Interval).
    #[serde(rename = "beatsPerMinute")]
    pub beats_per_minute: Option<i32>,
    /// Output only. The civil time in the timezone the subject is in at the time of the observation.
    #[serde(rename = "civilTime")]
    pub civil_time: Option<CivilDateTime>,
    /// Required. The time of the heart beat measurement.
    #[serde(rename = "physicalTime")]
    pub physical_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Required. The UTC offset of the user's timezone when the heart beat measurement occurred.
    #[serde(rename = "utcOffset")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub utc_offset: Option<chrono::Duration>,
}

impl common::Part for HeartBeat {}

/// A heart rate measurement.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct HeartRate {
    /// Required. The heart rate value in beats per minute.
    #[serde(rename = "beatsPerMinute")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub beats_per_minute: Option<i64>,
    /// Optional. Metadata about the heart rate sample.
    pub metadata: Option<HeartRateMetadata>,
    /// Required. Observation time
    #[serde(rename = "sampleTime")]
    pub sample_time: Option<ObservationSampleTime>,
}

impl common::Part for HeartRate {}

/// Heart rate metadata.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct HeartRateMetadata {
    /// Optional. Indicates the user’s level of activity when the heart rate sample was measured
    #[serde(rename = "motionContext")]
    pub motion_context: Option<String>,
    /// Optional. Indicates the location of the sensor that measured the heart rate.
    #[serde(rename = "sensorLocation")]
    pub sensor_location: Option<String>,
}

impl common::Part for HeartRateMetadata {}

/// Represents the result of the rollup of the heart rate data type.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct HeartRateRollupValue {
    /// The average heart rate value in the interval.
    #[serde(rename = "beatsPerMinuteAvg")]
    pub beats_per_minute_avg: Option<f64>,
    /// The maximum heart rate value in the interval.
    #[serde(rename = "beatsPerMinuteMax")]
    pub beats_per_minute_max: Option<f64>,
    /// The minimum heart rate value in the interval.
    #[serde(rename = "beatsPerMinuteMin")]
    pub beats_per_minute_min: Option<f64>,
}

impl common::Part for HeartRateRollupValue {}

/// Captures user's heart rate variability (HRV) as measured by the root mean square of successive differences (RMSSD) between normal heartbeats or by standard deviation of the inter-beat intervals (SDNN).
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct HeartRateVariability {
    /// Optional. The root mean square of successive differences between normal heartbeats. This is a measure of heart rate variability used by Google Health.
    #[serde(rename = "rootMeanSquareOfSuccessiveDifferencesMilliseconds")]
    pub root_mean_square_of_successive_differences_milliseconds: Option<f64>,
    /// Required. The time of the heart rate variability measurement.
    #[serde(rename = "sampleTime")]
    pub sample_time: Option<ObservationSampleTime>,
    /// Optional. The standard deviation of the heart rate variability measurement.
    #[serde(rename = "standardDeviationMilliseconds")]
    pub standard_deviation_milliseconds: Option<f64>,
}

impl common::Part for HeartRateVariability {}

/// Represents the result of the rollup of the user's daily heart rate variability personal range.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct HeartRateVariabilityPersonalRangeRollupValue {
    /// The upper bound of the user's average heart rate variability personal range.
    #[serde(rename = "averageHeartRateVariabilityMillisecondsMax")]
    pub average_heart_rate_variability_milliseconds_max: Option<f64>,
    /// The lower bound of the user's average heart rate variability personal range.
    #[serde(rename = "averageHeartRateVariabilityMillisecondsMin")]
    pub average_heart_rate_variability_milliseconds_min: Option<f64>,
}

impl common::Part for HeartRateVariabilityPersonalRangeRollupValue {}

/// The heart rate zone.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct HeartRateZone {
    /// Required. The heart rate zone type.
    #[serde(rename = "heartRateZoneType")]
    pub heart_rate_zone_type: Option<String>,
    /// Required. Maximum heart rate for this zone in beats per minute.
    #[serde(rename = "maxBeatsPerMinute")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub max_beats_per_minute: Option<i64>,
    /// Required. Minimum heart rate for this zone in beats per minute.
    #[serde(rename = "minBeatsPerMinute")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub min_beats_per_minute: Option<i64>,
}

impl common::Part for HeartRateZone {}

/// Body height measurement.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Height {
    /// Required. Height of the user in millimeters.
    #[serde(rename = "heightMillimeters")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub height_millimeters: Option<i64>,
    /// Required. The time at which the height was recorded.
    #[serde(rename = "sampleTime")]
    pub sample_time: Option<ObservationSampleTime>,
}

impl common::Part for Height {}

/// Holds information about a user logged hydration.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct HydrationLog {
    /// Required. Amount of liquid (ex. water) consumed.
    #[serde(rename = "amountConsumed")]
    pub amount_consumed: Option<VolumeQuantity>,
    /// Required. Observed interval.
    pub interval: Option<SessionTimeInterval>,
}

impl common::Part for HydrationLog {}

/// Represents the result of the rollup of the hydration log data type.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct HydrationLogRollupValue {
    /// Rollup for amount consumed.
    #[serde(rename = "amountConsumed")]
    pub amount_consumed: Option<VolumeQuantityRollup>,
}

impl common::Part for HydrationLogRollupValue {}

/// Represents details about the Google user’s identity.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get identity users](UserGetIdentityCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Identity {
    /// Output only. The Google User Identifier in the Google Health APIs. It matches the `{user}` resource ID segment in the resource name paths, e.g. `users/{user}/dataTypes/steps`. Valid values are strings of 1-63 characters, and valid characters are lowercase and uppercase letters, numbers, and hyphens.
    #[serde(rename = "healthUserId")]
    pub health_user_id: Option<String>,
    /// Output only. The legacy Fitbit User identifier. This is the Fitbit ID used in the legacy Fitbit APIs (v1-v3). It can be referenced by clients migrating from the legacy Fitbit APIs to map their existing identifiers to the new Google user ID. It **must not** be used for any other purpose. It is not of any use for new clients using only the Google Health APIs. Valid values are strings of 1-63 characters, and valid characters are lowercase and uppercase letters, numbers, and hyphens.
    #[serde(rename = "legacyUserId")]
    pub legacy_user_id: Option<String>,
    /// Identifier. The resource name of this Identity resource. Format: `users/me/identity`
    pub name: Option<String>,
}

impl common::ResponseResult for Identity {}

/// Represents a time interval, encoded as a Timestamp start (inclusive) and a Timestamp end (exclusive). The start must be less than or equal to the end. When the start equals the end, the interval is empty (matches no time). When both start and end are unspecified, the interval matches any time.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Interval {
    /// Optional. Exclusive end of the interval. If specified, a Timestamp matching this interval will have to be before the end.
    #[serde(rename = "endTime")]
    pub end_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Optional. Inclusive start of the interval. If specified, a Timestamp matching this interval will have to be the same or after the start.
    #[serde(rename = "startTime")]
    pub start_time: Option<chrono::DateTime<chrono::offset::Utc>>,
}

impl common::Part for Interval {}

/// Irregular Rhythm Notifications (IRN) Profile details. The Irregular Rhythm Notifications (IRN) feature checks for signs of atrial fibrillation (AFib). The IrnProfile details include information about the user’s onboarding status, enrollment status, and the last update time of analyzable data for this feature.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get irn profile users](UserGetIrnProfileCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IrnProfile {
    /// Required. Whether or not the user is currently enrolled in having their data processed for IRN alerts.
    #[serde(rename = "enrollmentStatus")]
    pub enrollment_status: Option<bool>,
    /// Identifier. The resource name of this IrnProfile resource. Format: `users/{user}/irnProfile` Example: `users/1234567890/irnProfile` or `users/me/irnProfile` The {user} ID is a system-generated Google Health API user ID, a string of 1-63 characters consisting of lowercase and uppercase letters, numbers, and hyphens. The literal `me` can also be used to refer to the authenticated user.
    pub name: Option<String>,
    /// Required. Whether or not the user has onboarded onto the IRN feature.
    #[serde(rename = "onboardingStatus")]
    pub onboarding_status: Option<bool>,
    /// Output only. The timestamp of the last piece of analyzable data synced by the user.
    #[serde(rename = "updateTime")]
    pub update_time: Option<chrono::DateTime<chrono::offset::Utc>>,
}

impl common::ResponseResult for IrnProfile {}

/// Represents an Irregular Rhythm Notification alert, indicating a potential sign of atrial fibrillation (AFib). This data type is based on SaMD feature and any changes to it may require additional review.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IrregularRhythmNotification {
    /// Optional. The overlapping analysis windows that were used to evaluate rhythm for potential AFib, containing specific information about the user's heart rhythm.
    #[serde(rename = "alertWindows")]
    pub alert_windows: Option<Vec<AlertWindow>>,
    /// Required. Observed interval.
    pub interval: Option<SessionTimeInterval>,
    /// Output only. The meta information for the compatible device used to conduct the measurement. Irregular Rhythm Notification measurements typically populate `algorithm_version`, `service_version`, and `device_model`.
    #[serde(rename = "medicalDeviceInfo")]
    pub medical_device_info: Option<MedicalDeviceInfo>,
}

impl common::Part for IrregularRhythmNotification {}

/// Response containing raw data points matching the query
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [data types data points list users](UserDataTypeDataPointListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListDataPointsResponse {
    /// Data points matching the query
    #[serde(rename = "dataPoints")]
    pub data_points: Option<Vec<DataPoint>>,
    /// Next page token, empty if the response is complete
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

impl common::ResponseResult for ListDataPointsResponse {}

/// Response message for ListPairedDevices.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [paired devices list users](UserPairedDeviceListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListPairedDevicesResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    /// The paired devices of the user.
    #[serde(rename = "pairedDevices")]
    pub paired_devices: Option<Vec<PairedDevice>>,
}

impl common::ResponseResult for ListPairedDevicesResponse {}

/// Response message for ListSubscribers.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [subscribers list projects](ProjectSubscriberListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListSubscribersResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    /// Subscribers from the specified project.
    pub subscribers: Option<Vec<Subscriber>>,
    /// The total number of subscribers matching the request.
    #[serde(rename = "totalSize")]
    pub total_size: Option<i32>,
}

impl common::ResponseResult for ListSubscribersResponse {}

/// Response message for ListSubscriptions.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [subscribers subscriptions list projects](ProjectSubscriberSubscriptionListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListSubscriptionsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    /// The subscriptions from the specified subscriber.
    pub subscriptions: Option<Vec<Subscription>>,
}

impl common::ResponseResult for ListSubscriptionsResponse {}

/// Software as Medical Device (SaMD) metadata. Used to construct the Unique Device Identifier (UDI).
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MedicalDeviceInfo {
    /// Output only. The algorithm version used by the feature.
    #[serde(rename = "algorithmVersion")]
    pub algorithm_version: Option<String>,
    /// Output only. The model name or device type of the compatible device used to collect the data.
    #[serde(rename = "deviceModel")]
    pub device_model: Option<String>,
    /// Output only. The version of the feature/app running on the device.
    #[serde(rename = "featureVersion")]
    pub feature_version: Option<String>,
    /// Output only. The firmware version running on the compatible device used to collect the data.
    #[serde(rename = "firmwareVersion")]
    pub firmware_version: Option<String>,
    /// Output only. The service version used by the feature.
    #[serde(rename = "serviceVersion")]
    pub service_version: Option<String>,
}

impl common::Part for MedicalDeviceInfo {}

/// Summary metrics for an exercise.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MetricsSummary {
    /// Optional. Total active zone minutes for the exercise.
    #[serde(rename = "activeZoneMinutes")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub active_zone_minutes: Option<i64>,
    /// Optional. Average heart rate during the exercise.
    #[serde(rename = "averageHeartRateBeatsPerMinute")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub average_heart_rate_beats_per_minute: Option<i64>,
    /// Optional. Average pace in seconds per meter.
    #[serde(rename = "averagePaceSecondsPerMeter")]
    pub average_pace_seconds_per_meter: Option<f64>,
    /// Optional. Average speed in millimeters per second.
    #[serde(rename = "averageSpeedMillimetersPerSecond")]
    pub average_speed_millimeters_per_second: Option<f64>,
    /// Optional. Total calories burned by the user during the exercise.
    #[serde(rename = "caloriesKcal")]
    pub calories_kcal: Option<f64>,
    /// Optional. Total distance covered by the user during the exercise.
    #[serde(rename = "distanceMillimeters")]
    pub distance_millimeters: Option<f64>,
    /// Optional. Total elevation gain during the exercise.
    #[serde(rename = "elevationGainMillimeters")]
    pub elevation_gain_millimeters: Option<f64>,
    /// Optional. Time spent in each heart rate zone.
    #[serde(rename = "heartRateZoneDurations")]
    pub heart_rate_zone_durations: Option<TimeInHeartRateZones>,
    /// Optional. Mobility workouts specific metrics. Only present in the advanced running exercises.
    #[serde(rename = "mobilityMetrics")]
    pub mobility_metrics: Option<MobilityMetrics>,
    /// Optional. Run VO2 max value for the exercise. Only present in the running exercises at the top level as in the summary of the whole exercise.
    #[serde(rename = "runVo2Max")]
    pub run_vo2_max: Option<f64>,
    /// Optional. Total steps taken during the exercise.
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub steps: Option<i64>,
    /// Optional. Number of full pool lengths completed during the exercise. Only present in the swimming exercises at the top level as in the summary of the whole exercise.
    #[serde(rename = "totalSwimLengths")]
    pub total_swim_lengths: Option<f64>,
}

impl common::Part for MetricsSummary {}

/// Mobility workouts specific metrics
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MobilityMetrics {
    /// Optional. Cadence is a measure of the frequency of your foot strikes. Steps / min in real time during workout.
    #[serde(rename = "avgCadenceStepsPerMinute")]
    pub avg_cadence_steps_per_minute: Option<f64>,
    /// Optional. The ground contact time for a particular stride is the amount of time for which the foot was in contact with the ground on that stride
    #[serde(rename = "avgGroundContactTimeDuration")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub avg_ground_contact_time_duration: Option<chrono::Duration>,
    /// Optional. Stride length is a measure of the distance covered by a single stride
    #[serde(rename = "avgStrideLengthMillimeters")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub avg_stride_length_millimeters: Option<i64>,
    /// Optional. Distance off the ground your center of mass moves with each stride while running
    #[serde(rename = "avgVerticalOscillationMillimeters")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub avg_vertical_oscillation_millimeters: Option<i64>,
    /// Optional. Vertical oscillation/stride length between [5.0, 11.0].
    #[serde(rename = "avgVerticalRatio")]
    pub avg_vertical_ratio: Option<f64>,
}

impl common::Part for MobilityMetrics {}

/// Represents the quantity of a nutrient.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NutrientQuantity {
    /// Required. Value representing the nutrient.
    pub nutrient: Option<String>,
    /// Required. Value representing the quantity of the nutrient.
    pub quantity: Option<WeightQuantity>,
}

impl common::Part for NutrientQuantity {}

/// Nutrient quantity rollup.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NutrientQuantityRollup {
    /// Required. Aggregated nutrient.
    pub nutrient: Option<String>,
    /// Required. Aggregated nutrient weight.
    pub quantity: Option<WeightQuantityRollup>,
}

impl common::Part for NutrientQuantityRollup {}

/// Holds information about a user logged food. There are two ways of creating a nutrition log based on the food type: 1. Identified food: Using the food field, which is a reference to a Food resource. In this case fields `nutrients`, `energy`, `energy_from_fat`, `total_carbohydrate`, `total_fat`, `food_display_name` will be populated based on the referenced food. 2. Anonymous food: Using the `food_display_name` field and setting the `nutrients`, `energy`, `energy_from_fat`, `total_carbohydrate`, `total_fat` fields manually. The identified food is preferred over the anonymous food. Nutrition logs created from anonymous food are not be editable.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NutritionLog {
    /// Optional. Value representing the energy of the nutrition log. For nutrition logs created from an identified food, this field will be populated based on the referenced food. For anonymous food, this field will be populated manually.
    pub energy: Option<EnergyQuantity>,
    /// Optional. Value representing the energy from fat of the nutrition log. For nutrition logs created from an identified food, this field will be populated based on the referenced food. For anonymous food, this field will be populated manually.
    #[serde(rename = "energyFromFat")]
    pub energy_from_fat: Option<EnergyQuantity>,
    /// Required. Represents the food ID.
    pub food: Option<String>,
    /// Value representing the display name of the food. For nutrition logs created from an identified food, this field will be populated based on the referenced food. For anonymous food, this field will be populated manually.
    #[serde(rename = "foodDisplayName")]
    pub food_display_name: Option<String>,
    /// Required. Observed interval.
    pub interval: Option<SessionTimeInterval>,
    /// Optional. Value representing the meal type of the nutrition log.
    #[serde(rename = "mealType")]
    pub meal_type: Option<String>,
    /// Optional. Value representing the nutrients of the nutrition log.
    pub nutrients: Option<Vec<NutrientQuantity>>,
    /// Optional. Value representing the nutrition log serving.
    pub serving: Option<Serving>,
    /// Optional. Value representing the total carbohydrate of the nutrition log. For nutrition logs created from an identified food, this field will be populated based on the referenced food. For anonymous food, this field will be populated manually.
    #[serde(rename = "totalCarbohydrate")]
    pub total_carbohydrate: Option<WeightQuantity>,
    /// Optional. Value representing the total fat of the nutrition log. For nutrition logs created from an identified food, this field will be populated based on the referenced food. For anonymous food, this field will be populated manually.
    #[serde(rename = "totalFat")]
    pub total_fat: Option<WeightQuantity>,
}

impl common::Part for NutritionLog {}

/// Represents the result of the rollup of the nutrition log data type.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NutritionLogRollupValue {
    /// Energy rollup.
    pub energy: Option<EnergyQuantityRollup>,
    /// Value Energy from fat rollup.
    #[serde(rename = "energyFromFat")]
    pub energy_from_fat: Option<EnergyQuantityRollup>,
    /// List of the nutrient roll-ups by the nutrient type.
    pub nutrients: Option<Vec<NutrientQuantityRollup>>,
    /// Total carbohydrate rollup.
    #[serde(rename = "totalCarbohydrate")]
    pub total_carbohydrate: Option<WeightQuantityRollup>,
    /// Total fat rollup.
    #[serde(rename = "totalFat")]
    pub total_fat: Option<WeightQuantityRollup>,
}

impl common::Part for NutritionLogRollupValue {}

/// Represents a sample time of an observed data point.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ObservationSampleTime {
    /// Output only. The civil time in the timezone the subject is in at the time of the observation.
    #[serde(rename = "civilTime")]
    pub civil_time: Option<CivilDateTime>,
    /// Required. The time of the observation.
    #[serde(rename = "physicalTime")]
    pub physical_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Required. The offset of the user's local time during the observation relative to the Coordinated Universal Time (UTC).
    #[serde(rename = "utcOffset")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub utc_offset: Option<chrono::Duration>,
}

impl common::Part for ObservationSampleTime {}

/// Represents a time interval of an observed data point.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ObservationTimeInterval {
    /// Output only. Observed interval end time in civil time in the timezone the subject is in at the end of the observed interval
    #[serde(rename = "civilEndTime")]
    pub civil_end_time: Option<CivilDateTime>,
    /// Output only. Observed interval start time in civil time in the timezone the subject is in at the start of the observed interval
    #[serde(rename = "civilStartTime")]
    pub civil_start_time: Option<CivilDateTime>,
    /// Required. Observed interval end time.
    #[serde(rename = "endTime")]
    pub end_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Required. The offset of the user's local time at the end of the observation relative to the Coordinated Universal Time (UTC).
    #[serde(rename = "endUtcOffset")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub end_utc_offset: Option<chrono::Duration>,
    /// Required. Observed interval start time.
    #[serde(rename = "startTime")]
    pub start_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Required. The offset of the user's local time at the start of the observation relative to the Coordinated Universal Time (UTC).
    #[serde(rename = "startUtcOffset")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub start_utc_offset: Option<chrono::Duration>,
}

impl common::Part for ObservationTimeInterval {}

/// This resource represents a long-running operation that is the result of a network API call.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [subscribers create projects](ProjectSubscriberCreateCall) (response)
/// * [subscribers delete projects](ProjectSubscriberDeleteCall) (response)
/// * [subscribers patch projects](ProjectSubscriberPatchCall) (response)
/// * [data types data points batch delete users](UserDataTypeDataPointBatchDeleteCall) (response)
/// * [data types data points create users](UserDataTypeDataPointCreateCall) (response)
/// * [data types data points patch users](UserDataTypeDataPointPatchCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Operation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    pub error: Option<Status>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    pub name: Option<String>,
    /// The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    pub response: Option<HashMap<String, serde_json::Value>>,
}

impl common::ResponseResult for Operation {}

/// A time interval to represent an out-of-bed segment.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct OutOfBedSegment {
    /// Required. Segment end time.
    #[serde(rename = "endTime")]
    pub end_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Required. The offset of the user's local time at the end of the segment relative to the Coordinated Universal Time (UTC).
    #[serde(rename = "endUtcOffset")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub end_utc_offset: Option<chrono::Duration>,
    /// Required. Segment tart time.
    #[serde(rename = "startTime")]
    pub start_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Required. The offset of the user's local time at the start of the segment relative to the Coordinated Universal Time (UTC).
    #[serde(rename = "startUtcOffset")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub start_utc_offset: Option<chrono::Duration>,
}

impl common::Part for OutOfBedSegment {}

/// Captures the user's instantaneous oxygen saturation percentage (SpO2).
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct OxygenSaturation {
    /// Required. The oxygen saturation percentage. Valid values are from 0 to 100.
    pub percentage: Option<f64>,
    /// Required. The time at which oxygen saturation was measured.
    #[serde(rename = "sampleTime")]
    pub sample_time: Option<ObservationSampleTime>,
}

impl common::Part for OxygenSaturation {}

/// User’s Paired 1P Device The PairedDevice details include information about the device type, battery status, battery level, last sync time, device version, mac address, and features.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [paired devices get users](UserPairedDeviceGetCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PairedDevice {
    /// Output only. The battery level of the device.
    #[serde(rename = "batteryLevel")]
    pub battery_level: Option<i32>,
    /// Output only. The battery status of the device. Supported: High | Medium | Low | Empty
    #[serde(rename = "batteryStatus")]
    pub battery_status: Option<String>,
    /// Output only. The device type. Supported: TRACKER | SCALE
    #[serde(rename = "deviceType")]
    pub device_type: Option<String>,
    /// Output only. The product name of the device
    #[serde(rename = "deviceVersion")]
    pub device_version: Option<String>,
    /// Output only. Lists of unique features supported by the device. Comprehensive list of supported features: **Fitness Tracking** - `ACTIVE_MINUTES`: Legacy active minutes. - `AUTOSTRIDE`: Automatic stride length calculation. - `BIKE_ONBOARDING`: Cycling UI support. - `CALORIES`: Daily burned calories. - `DISTANCE`: Daily distance tracking. - `ELEVATION`: Floors climbed. - `INACTIVITY_ALERTS`: Reminders to move. - `SEDENTARY_TIME`: Tracks inactive time. - `STEPS`: Daily steps. - `SWIM`: Swim tracking (laps/strokes). - `AUTORUN`: Automatic run detection. - `ACTIVE_ZONE_MINUTES`: Active Zone Minutes (AZM). **Heart Rate & Health** - `HEART_RATE`: Continuous heart rate (PPG). - `BAT_SIGNAL`: High/Low Heart Rate Alerts. **Advanced Sensors** - `SPO2`: Blood oxygen saturation. - `NIGHTTIME_OXYGEN_SATURATION`: Sleep SpO2. - `ESTIMATED_OXYGEN_VARIATION`: Estimated Oxygen Variation. - `EDA`: Electrodermal Activity (stress). - `SKIN_TEMPERATURE`: Skin temperature variation. - `INTERNAL_DEVICE_TEMPERATURE`: Internal device temperature. **Sleep & Wellness** - `SLEEP`: Basic sleep tracking. - `SMART_SLEEP`: Advanced sleep tracking (stages/score). - `BEDTIME_REMINDER`: Bedtime reminders. - `SOUNDSCAPE`: Snore and noise detection. **Advanced Workouts** - `WB`: Custom Workout Builder. - `AUTOCUES`: Auto Cues / Auto Lap. - `DWR_RUN`: Daily Run Recommendations. - `ADVANCED_RUNNING`: Advanced Running Dynamics (e.g., GCT, VO). **GPS & Location** - `GPS`: Built-in GPS. - `CONNECTED_GPS`: Connected GPS (uses phone). - `LOCATION_HINT`: Location helper. **Payments & NFC** - `PAYMENTS`: NFC payments (Fitbit Pay/Google Wallet). - `FELICA`: FeliCa support (Japan payments/transit). **Activity Detection** - `GROK`: SmartTrack automatic activity detection. - `RETRO_AR`: Retroactive Activity Recognition prompts. **Smart Features & UI** - `ALARMS`: Silent alarms. - `BLE_MUSIC_CONTROL`: BLE music control. - `MUSIC`: Direct music storage/control. - `YOUTUBE_MUSIC_SUPPORTED`: YouTube Music support. - `GALLERY`: App Gallery. - `TUTORIAL_SUPPORTED`: On-screen tutorials. - `SMILEY_EMOTE`: Legacy Zip face. - `MOBILE_TO_DEVICE_DEEPLINK`: Mobile to device settings deep link. - `HIDE_GALLERY`: Option to hide Gallery. - `HIDE_GOAL_SELECTION`: Option to hide goal selection. - `DIGITAL_WARRANTY_SUPPORTED`: Digital warranty display. - `DIRECT_DEVICE_SETTINGS_SUPPORTED`: Direct device settings management. **Gym HR Broadcasting** - `ASPEN_SUPPORTED`: Broadcast HR to gym equipment. - `ASPEN_REMOTE_UI_SUPPORTED`: Remote UI for HR sharing. **Privacy & Security** - `FINITE_IMPROBABILITY`: BLE Resolvable Private Address (RPA) privacy. - `DOMAIN_KEY_SYNC`: Domain key synchronization. **BLE Protocol** - `BONDING`: Secure BLE bonding. - `ADVERTISES_SERIAL`: Advertises serial number. - `STATUS_CHARACTERISTIC`: BLE Status Characteristic. - `TRACKER_CHANNEL_CHARACTERISTIC`: BLE Tracker Channel Characteristic. - `PING_CHARACTERISTIC`: BLE Ping Characteristic. **Cellular & Wi-Fi** - `MOBILE_DATA`: LTE cellular support. - `SINGLE_AP_WIFI`: Single AP Wi-Fi. - `MULTI_AP_WIFI`: Multi AP Wi-Fi. - `WIFI_FWUP`: Firmware updates over Wi-Fi. **Data Sync & Transfer** - `APP_SYNC`: Background app sync. - `LIVE_DATA`: Real-time data streaming. - `EVENT_BASED_SYNC_SUPPORTED`: Event-based sync. - `TIME_SERVICE`: Time synchronization service. - `REMOTE_FILE_PROVIDER`: Remote file transfer. - `DIRECT_COMMS_ALARMS`: Direct communication for alarms. - `DIRECT_COMMS_EXERCISE`: Direct communication for exercise. - `DIRECT_COMMS_BATTERY_ALERTS`: Direct communication for battery alerts. **Google Integrations** - `PARROT_TREE_SUPPORTED`: Find My Device support.
    pub features: Option<Vec<String>>,
    /// Output only. The time of last sync with the Fitbit mobile application.
    #[serde(rename = "lastSyncTime")]
    pub last_sync_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Output only. Mac ID number of the device.
    #[serde(rename = "macAddress")]
    pub mac_address: Option<String>,
    /// Identifier. The resource name of this Device resource. Format: `users/{user}/pairedDevices/{paired_device}` Example: `users/1234567890/pairedDevices/123` or `users/me/pairedDevices/123`
    pub name: Option<String>,
}

impl common::ResponseResult for PairedDevice {}

/// Profile details.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get profile users](UserGetProfileCall) (response)
/// * [update profile users](UserUpdateProfileCall) (request|response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Profile {
    /// Optional. The age in years based on the user's birth date. Updates to this field are currently not supported.
    pub age: Option<i32>,
    /// Output only. The automatically calculated running stride length, in millimeters. The user must consent to one of the following access scopes to access this field: - `https://www.googleapis.com/auth/googlehealth.activity_and_fitness.readonly` - `https://www.googleapis.com/auth/googlehealth.activity_and_fitness`
    #[serde(rename = "autoRunningStrideLengthMm")]
    pub auto_running_stride_length_mm: Option<i32>,
    /// Output only. The automatically calculated walking stride length, in millimeters. The user must consent to one of the following access scopes to access this field: - `https://www.googleapis.com/auth/googlehealth.activity_and_fitness.readonly` - `https://www.googleapis.com/auth/googlehealth.activity_and_fitness`
    #[serde(rename = "autoWalkingStrideLengthMm")]
    pub auto_walking_stride_length_mm: Option<i32>,
    /// Output only. The date the user created their account. Updates to this field are currently not supported.
    #[serde(rename = "membershipStartDate")]
    pub membership_start_date: Option<Date>,
    /// Identifier. The resource name of this Profile resource. Format: `users/{user}/profile` Example: `users/1234567890/profile` or `users/me/profile` The {user} ID is a system-generated Google Health API user ID, a string of 1-63 characters consisting of lowercase and uppercase letters, numbers, and hyphens. The literal `me` can also be used to refer to the authenticated user.
    pub name: Option<String>,
    /// Optional. The user's user configured running stride length, in millimeters. The user must consent to one of the following access scopes to access this field: - `https://www.googleapis.com/auth/googlehealth.activity_and_fitness.readonly` - `https://www.googleapis.com/auth/googlehealth.activity_and_fitness`
    #[serde(rename = "userConfiguredRunningStrideLengthMm")]
    pub user_configured_running_stride_length_mm: Option<i32>,
    /// Optional. The user's user configured walking stride length, in millimeters. The user must consent to one of the following access scopes to access this field: - `https://www.googleapis.com/auth/googlehealth.activity_and_fitness.readonly` - `https://www.googleapis.com/auth/googlehealth.activity_and_fitness`
    #[serde(rename = "userConfiguredWalkingStrideLengthMm")]
    pub user_configured_walking_stride_length_mm: Option<i32>,
}

impl common::RequestValue for Profile {}
impl common::ResponseResult for Profile {}

/// Response containing the list of reconciled DataPoints.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [data types data points reconcile users](UserDataTypeDataPointReconcileCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ReconcileDataPointsResponse {
    /// Data points matching the query
    #[serde(rename = "dataPoints")]
    pub data_points: Option<Vec<ReconciledDataPoint>>,
    /// Next page token, empty if the response is complete
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

impl common::ResponseResult for ReconcileDataPointsResponse {}

/// A reconciled computed or recorded metric.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ReconciledDataPoint {
    /// Data for points in the `active-energy-burned` interval data type collection.
    #[serde(rename = "activeEnergyBurned")]
    pub active_energy_burned: Option<ActiveEnergyBurned>,
    /// Data for points in the `active-minutes` interval data type collection.
    #[serde(rename = "activeMinutes")]
    pub active_minutes: Option<ActiveMinutes>,
    /// Data for points in the `active-zone-minutes` interval data type collection, measured in minutes.
    #[serde(rename = "activeZoneMinutes")]
    pub active_zone_minutes: Option<ActiveZoneMinutes>,
    /// Data for points in the `activity-level` daily data type collection.
    #[serde(rename = "activityLevel")]
    pub activity_level: Option<ActivityLevel>,
    /// Data for points in the `altitude` interval data type collection.
    pub altitude: Option<Altitude>,
    /// Data for points in the `basal-energy-burned` interval data type collection.
    #[serde(rename = "basalEnergyBurned")]
    pub basal_energy_burned: Option<BasalEnergyBurned>,
    /// Data for points in the `blood-glucose` sample data type collection.
    #[serde(rename = "bloodGlucose")]
    pub blood_glucose: Option<BloodGlucose>,
    /// Data for points in the `body-fat` sample data type collection.
    #[serde(rename = "bodyFat")]
    pub body_fat: Option<BodyFat>,
    /// Data for points in the `core-body-temperature` sample data type collection.
    #[serde(rename = "coreBodyTemperature")]
    pub core_body_temperature: Option<CoreBodyTemperature>,
    /// Data for points in the `daily-heart-rate-variability` daily data type collection.
    #[serde(rename = "dailyHeartRateVariability")]
    pub daily_heart_rate_variability: Option<DailyHeartRateVariability>,
    /// Data for points in the `daily-heart-rate-zones` daily data type collection.
    #[serde(rename = "dailyHeartRateZones")]
    pub daily_heart_rate_zones: Option<DailyHeartRateZones>,
    /// Data for points in the `daily-oxygen-saturation` daily data type collection.
    #[serde(rename = "dailyOxygenSaturation")]
    pub daily_oxygen_saturation: Option<DailyOxygenSaturation>,
    /// Data for points in the `daily-respiratory-rate` daily data type collection.
    #[serde(rename = "dailyRespiratoryRate")]
    pub daily_respiratory_rate: Option<DailyRespiratoryRate>,
    /// Data for points in the `daily-resting-heart-rate` daily data type collection.
    #[serde(rename = "dailyRestingHeartRate")]
    pub daily_resting_heart_rate: Option<DailyRestingHeartRate>,
    /// Data for points in the `daily-sleep-temperature-derivations` daily data type collection.
    #[serde(rename = "dailySleepTemperatureDerivations")]
    pub daily_sleep_temperature_derivations: Option<DailySleepTemperatureDerivations>,
    /// Data for points in the `daily-vo2-max` daily data type collection.
    #[serde(rename = "dailyVo2Max")]
    pub daily_vo2_max: Option<DailyVO2Max>,
    /// Identifier. Data point name, only supported for the subset of identifiable data types. For the majority of the data types, individual data points do not need to be identified and this field would be empty. Format: `users/{user}/dataTypes/{data_type}/dataPoints/{data_point}` Example: `users/abcd1234/dataTypes/sleep/dataPoints/a1b2c3d4-e5f6-7890-1234-567890abcdef` The `{user}` ID is a system-generated identifier, as described in Identity.health_user_id. The `{data_type}` ID corresponds to the kebab-case version of the field names in the DataPoint data union field, e.g. `heart-rate` for the `heart_rate` field. The `{data_point}` ID can be client-provided or system-generated. If client-provided, it must be a string of 4-63 characters, containing only lowercase letters, numbers, and hyphens.
    #[serde(rename = "dataPointName")]
    pub data_point_name: Option<String>,
    /// Data for points in the `distance` interval data type collection.
    pub distance: Option<Distance>,
    /// Data for points in the `exercise` session data type collection.
    pub exercise: Option<Exercise>,
    /// Data for points in the `floors` interval data type collection.
    pub floors: Option<Floors>,
    /// Data for points in the `heart-rate` sample data type collection.
    #[serde(rename = "heartRate")]
    pub heart_rate: Option<HeartRate>,
    /// Data for points in the `heart-rate-variability` sample data type collection.
    #[serde(rename = "heartRateVariability")]
    pub heart_rate_variability: Option<HeartRateVariability>,
    /// Data for points in the `height` sample data type collection.
    pub height: Option<Height>,
    /// Data for points in the `hydration-log` session data type collection.
    #[serde(rename = "hydrationLog")]
    pub hydration_log: Option<HydrationLog>,
    /// Data for points in the `nutrition-log` session data type collection.
    #[serde(rename = "nutritionLog")]
    pub nutrition_log: Option<NutritionLog>,
    /// Data for points in the `oxygen-saturation` sample data type collection.
    #[serde(rename = "oxygenSaturation")]
    pub oxygen_saturation: Option<OxygenSaturation>,
    /// Data for points in the `respiratory-rate-sleep-summary` sample data type collection.
    #[serde(rename = "respiratoryRateSleepSummary")]
    pub respiratory_rate_sleep_summary: Option<RespiratoryRateSleepSummary>,
    /// Data for points in the `run-vo2-max` sample data type collection.
    #[serde(rename = "runVo2Max")]
    pub run_vo2_max: Option<RunVO2Max>,
    /// Data for points in the `sedentary-period` interval data type collection.
    #[serde(rename = "sedentaryPeriod")]
    pub sedentary_period: Option<SedentaryPeriod>,
    /// Data for points in the `sleep` session data type collection.
    pub sleep: Option<Sleep>,
    /// Data for points in the `steps` interval data type collection.
    pub steps: Option<Steps>,
    /// Data for points in the `swim-lengths-data` interval data type collection.
    #[serde(rename = "swimLengthsData")]
    pub swim_lengths_data: Option<SwimLengthsData>,
    /// Data for points in the `time-in-heart-rate-zone` interval data type collection.
    #[serde(rename = "timeInHeartRateZone")]
    pub time_in_heart_rate_zone: Option<TimeInHeartRateZone>,
    /// Data for points in the `vo2-max` sample data type collection.
    #[serde(rename = "vo2Max")]
    pub vo2_max: Option<VO2Max>,
    /// Data for points in the `weight` sample data type collection.
    pub weight: Option<Weight>,
}

impl common::Part for ReconciledDataPoint {}

/// Records respiratory rate details during sleep. Can have multiple per day if the user sleeps multiple times.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RespiratoryRateSleepSummary {
    /// Optional. Respiratory rate statistics for deep sleep.
    #[serde(rename = "deepSleepStats")]
    pub deep_sleep_stats: Option<RespiratoryRateSleepSummaryStatistics>,
    /// Required. Full respiratory rate statistics.
    #[serde(rename = "fullSleepStats")]
    pub full_sleep_stats: Option<RespiratoryRateSleepSummaryStatistics>,
    /// Optional. Respiratory rate statistics for light sleep.
    #[serde(rename = "lightSleepStats")]
    pub light_sleep_stats: Option<RespiratoryRateSleepSummaryStatistics>,
    /// Optional. Respiratory rate statistics for REM sleep.
    #[serde(rename = "remSleepStats")]
    pub rem_sleep_stats: Option<RespiratoryRateSleepSummaryStatistics>,
    /// Required. The time at which respiratory rate was measured.
    #[serde(rename = "sampleTime")]
    pub sample_time: Option<ObservationSampleTime>,
}

impl common::Part for RespiratoryRateSleepSummary {}

/// Respiratory rate statistics for a given sleep stage.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RespiratoryRateSleepSummaryStatistics {
    /// Required. Average breaths per minute.
    #[serde(rename = "breathsPerMinute")]
    pub breaths_per_minute: Option<f64>,
    /// Optional. How trustworthy the data is for the computation.
    #[serde(rename = "signalToNoise")]
    pub signal_to_noise: Option<f64>,
    /// Optional. Standard deviation of the respiratory rate during sleep.
    #[serde(rename = "standardDeviation")]
    pub standard_deviation: Option<f64>,
}

impl common::Part for RespiratoryRateSleepSummaryStatistics {}

/// Represents the rollup value for the daily resting heart rate data type.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RestingHeartRatePersonalRangeRollupValue {
    /// The upper bound of the user's daily resting heart rate personal range.
    #[serde(rename = "beatsPerMinuteMax")]
    pub beats_per_minute_max: Option<f64>,
    /// The lower bound of the user's daily resting heart rate personal range.
    #[serde(rename = "beatsPerMinuteMin")]
    pub beats_per_minute_min: Option<f64>,
}

impl common::Part for RestingHeartRatePersonalRangeRollupValue {}

/// Request to roll up data points by physical time intervals.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [data types data points roll up users](UserDataTypeDataPointRollUpCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RollUpDataPointsRequest {
    /// Optional. The data source family name to roll up. If empty, data points from all available data sources will be rolled up. Format: `users/me/dataSourceFamilies/{data_source_family}` The supported values are: - `users/me/dataSourceFamilies/all-sources` - default value - `users/me/dataSourceFamilies/google-wearables` - tracker devices - `users/me/dataSourceFamilies/google-sources` - Google first party sources
    #[serde(rename = "dataSourceFamily")]
    pub data_source_family: Option<String>,
    /// Optional. The maximum number of data points to return. If unspecified, at most 1440 data points will be returned. The maximum page size is 10000; values above that will be truncated accordingly.
    #[serde(rename = "pageSize")]
    pub page_size: Option<i32>,
    /// Optional. The next_page_token from a previous request, if any. All other request fields need to be the same as in the initial request when the page token is specified.
    #[serde(rename = "pageToken")]
    pub page_token: Option<String>,
    /// Required. Closed-open range of data points that will be rolled up. The maximum range for `calories-in-heart-rate-zone`, `heart-rate`, `active-minutes` and `total-calories` is 14 days. The maximum range for all other data types is 90 days.
    pub range: Option<Interval>,
    /// Required. The size of the time window to group data points into before applying the aggregation functions. Must be at least 1 second.
    #[serde(rename = "windowSize")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub window_size: Option<chrono::Duration>,
}

impl common::RequestValue for RollUpDataPointsRequest {}

/// Response containing the list of rolled up data points.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [data types data points roll up users](UserDataTypeDataPointRollUpCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RollUpDataPointsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    /// Values for each aggregation time window.
    #[serde(rename = "rollupDataPoints")]
    pub rollup_data_points: Option<Vec<RollupDataPoint>>,
}

impl common::ResponseResult for RollUpDataPointsResponse {}

/// Value of a rollup for a single physical time interval (aggregation window) of reconciled data points from all data sources, excluding those data points that are identified as recorded by wearables in intervals when they were not actually worn.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RollupDataPoint {
    /// Returned by default when rolling up data points from the `active-energy-burned` data type.
    #[serde(rename = "activeEnergyBurned")]
    pub active_energy_burned: Option<ActiveEnergyBurnedRollupValue>,
    /// Returned by default when rolling up data points from the `active-minutes` data type, or when requested explicitly using the `active-minutes` rollup type identifier.
    #[serde(rename = "activeMinutes")]
    pub active_minutes: Option<ActiveMinutesRollupValue>,
    /// Returned by default when rolling up data points from the `active-zone-minutes` data type, or when requested explicitly using the `active-zone-minutes` rollup type identifier.
    #[serde(rename = "activeZoneMinutes")]
    pub active_zone_minutes: Option<ActiveZoneMinutesRollupValue>,
    /// Returned by default when rolling up data points from the `activity-level` data type, or when requested explicitly using the `activity-level` rollup type identifier.
    #[serde(rename = "activityLevel")]
    pub activity_level: Option<ActivityLevelRollupValue>,
    /// Returned by default when rolling up data points from the `altitude` data type, or when requested explicitly using the `altitude` rollup type identifier.
    pub altitude: Option<AltitudeRollupValue>,
    /// Returned by default when rolling up data points from the `blood-glucose` data type.
    #[serde(rename = "bloodGlucose")]
    pub blood_glucose: Option<BloodGlucoseRollupValue>,
    /// Returned by default when rolling up data points from the `body-fat` data type, or when requested explicitly using the `body-fat` rollup type identifier.
    #[serde(rename = "bodyFat")]
    pub body_fat: Option<BodyFatRollupValue>,
    /// Returned by default when rolling up data points from the `calories-in-heart-rate-zone` data type, or when requested explicitly using the `calories-in-heart-rate-zone` rollup type identifier.
    #[serde(rename = "caloriesInHeartRateZone")]
    pub calories_in_heart_rate_zone: Option<CaloriesInHeartRateZoneRollupValue>,
    /// Returned by default when rolling up data points from the `core-body-temperature` data type, or when requested explicitly using the `core-body-temperature` rollup type identifier.
    #[serde(rename = "coreBodyTemperature")]
    pub core_body_temperature: Option<CoreBodyTemperatureRollupValue>,
    /// Returned by default when rolling up data points from the `distance` data type, or when requested explicitly using the `distance` rollup type identifier.
    pub distance: Option<DistanceRollupValue>,
    /// End time of the window this value aggregates over
    #[serde(rename = "endTime")]
    pub end_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Returned by default when rolling up data points from the `floors` data type, or when requested explicitly using the `floors` rollup type identifier.
    pub floors: Option<FloorsRollupValue>,
    /// Returned by default when rolling up data points from the `heart-rate` data type, or when requested explicitly using the `heart-rate` rollup type identifier.
    #[serde(rename = "heartRate")]
    pub heart_rate: Option<HeartRateRollupValue>,
    /// Returned by default when rolling up data points from the `hydration-log` data type, or when requested explicitly using the `hydration-log` rollup type identifier.
    #[serde(rename = "hydrationLog")]
    pub hydration_log: Option<HydrationLogRollupValue>,
    /// Returned by default when rolling up data points from the `nutrition-log` data type, or when requested explicitly using the `nutrition-log` rollup type identifier.
    #[serde(rename = "nutritionLog")]
    pub nutrition_log: Option<NutritionLogRollupValue>,
    /// Returned by default when rolling up data points from the `run-vo2-max` data type, or when requested explicitly using the `run-vo2-max` rollup type identifier.
    #[serde(rename = "runVo2Max")]
    pub run_vo2_max: Option<RunVO2MaxRollupValue>,
    /// Returned by default when rolling up data points from the `sedentary-period` data type, or when requested explicitly using the `sedentary-period` rollup type identifier.
    #[serde(rename = "sedentaryPeriod")]
    pub sedentary_period: Option<SedentaryPeriodRollupValue>,
    /// Start time of the window this value aggregates over
    #[serde(rename = "startTime")]
    pub start_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Returned by default when rolling up data points from the `steps` data type, or when requested explicitly using the `steps` rollup type identifier.
    pub steps: Option<StepsRollupValue>,
    /// Returned by default when rolling up data points from the `swim-lengths-data` data type, or when requested explicitly using the `swim-lengths-data` rollup type identifier.
    #[serde(rename = "swimLengthsData")]
    pub swim_lengths_data: Option<SwimLengthsDataRollupValue>,
    /// Returned by default when rolling up data points from the `time-in-heart-rate-zone` data type, or when requested explicitly using the `time-in-heart-rate-zone` rollup type identifier.
    #[serde(rename = "timeInHeartRateZone")]
    pub time_in_heart_rate_zone: Option<TimeInHeartRateZoneRollupValue>,
    /// Returned by default when rolling up data points from the `total-calories` data type, or when requested explicitly using the `total-calories` rollup type identifier.
    #[serde(rename = "totalCalories")]
    pub total_calories: Option<TotalCaloriesRollupValue>,
    /// Returned by default when rolling up data points from the `weight` data type, or when requested explicitly using the `weight` rollup type identifier.
    pub weight: Option<WeightRollupValue>,
}

impl common::Part for RollupDataPoint {}

/// VO2 max value calculated based on the user's running activity. Value stored in ml/kg/min.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RunVO2Max {
    /// Required. Run VO2 max value in ml/kg/min.
    #[serde(rename = "runVo2Max")]
    pub run_vo2_max: Option<f64>,
    /// Required. The time at which the metric was measured.
    #[serde(rename = "sampleTime")]
    pub sample_time: Option<ObservationSampleTime>,
}

impl common::Part for RunVO2Max {}

/// Represents the result of the rollup of the user's daily heart rate variability personal range.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RunVO2MaxRollupValue {
    /// Average value of run VO2 max in the interval.
    #[serde(rename = "rateAvg")]
    pub rate_avg: Option<f64>,
    /// Maximum value of run VO2 max in the interval.
    #[serde(rename = "rateMax")]
    pub rate_max: Option<f64>,
    /// Minimum value of run VO2 max in the interval..
    #[serde(rename = "rateMin")]
    pub rate_min: Option<f64>,
}

impl common::Part for RunVO2MaxRollupValue {}

/// SedentaryPeriod SedentaryPeriod data represents the periods of time that the user was sedentary (i.e. not moving while wearing the device).
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SedentaryPeriod {
    /// Required. Observed interval.
    pub interval: Option<ObservationTimeInterval>,
}

impl common::Part for SedentaryPeriod {}

/// Represents the result of the rollup of the user's sedentary periods.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SedentaryPeriodRollupValue {
    /// The total time user spent sedentary during the interval.
    #[serde(rename = "durationSum")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub duration_sum: Option<chrono::Duration>,
}

impl common::Part for SedentaryPeriodRollupValue {}

/// Represents different properties and information about the serving of a specific food.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Serving {
    /// Optional. Amount of food consumed, fractional values are supported.
    pub amount: Option<f64>,
    /// Required. Food measurement unit
    #[serde(rename = "foodMeasurementUnit")]
    pub food_measurement_unit: Option<String>,
    /// Output only. Legacy measurement unit for serving size in singular form (e.g. "piece", "gram").
    #[serde(rename = "foodMeasurementUnitDisplayName")]
    pub food_measurement_unit_display_name: Option<String>,
}

impl common::Part for Serving {}

/// Represents a time interval of session data point, which bundles multiple observed metrics together.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SessionTimeInterval {
    /// Output only. Session end time in civil time in the timezone the subject is in at the end of the session.
    #[serde(rename = "civilEndTime")]
    pub civil_end_time: Option<CivilDateTime>,
    /// Output only. Session start time in civil time in the timezone the subject is in at the start of the session.
    #[serde(rename = "civilStartTime")]
    pub civil_start_time: Option<CivilDateTime>,
    /// Required. The end time of the observed session.
    #[serde(rename = "endTime")]
    pub end_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Required. The offset of the user's local time at the end of the session relative to the Coordinated Universal Time (UTC).
    #[serde(rename = "endUtcOffset")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub end_utc_offset: Option<chrono::Duration>,
    /// Required. The start time of the observed session.
    #[serde(rename = "startTime")]
    pub start_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Required. The offset of the user's local time at the start of the session relative to the Coordinated Universal Time (UTC).
    #[serde(rename = "startUtcOffset")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub start_utc_offset: Option<chrono::Duration>,
}

impl common::Part for SessionTimeInterval {}

/// Settings details.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get settings users](UserGetSettingCall) (response)
/// * [update settings users](UserUpdateSettingCall) (request|response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    /// Optional. True if the user's stride length is determined automatically. Updates to this field are currently not supported.
    #[serde(rename = "autoStrideEnabled")]
    pub auto_stride_enabled: Option<bool>,
    /// Optional. The measurement unit defined in the user's account settings. Updates to this field are currently not supported.
    #[serde(rename = "distanceUnit")]
    pub distance_unit: Option<String>,
    /// Output only. The food language code derived from the user's food database. Possible values: `'en-US'`, `'en-GB'`, `'de-DE'`, `'es-ES'`, `'fr-FR'`, `'zh-CN'`, `'zh-TW'`, `'ja-JP'`, `'en-AU'`, `'en-CA'`, `'it-IT'`, `'ko-KR'`, `'es-MX'`, `'en-IN'`, `'en-SG'`, `'en-PH'`, `'en-IE'`, `'fr-CA'`. Updates to this field are currently not supported.
    #[serde(rename = "foodLanguageCode")]
    pub food_language_code: Option<String>,
    /// Optional. The measurement unit defined in the user's account settings.
    #[serde(rename = "glucoseUnit")]
    pub glucose_unit: Option<String>,
    /// Optional. The measurement unit defined in the user's account settings.
    #[serde(rename = "heightUnit")]
    pub height_unit: Option<String>,
    /// Optional. The locale defined in the user's account settings. Updates to this field are currently not supported.
    #[serde(rename = "languageLocale")]
    pub language_locale: Option<String>,
    /// Identifier. The resource name of this Settings resource. Format: `users/{user}/settings` Example: `users/1234567890/settings` or `users/me/settings` The {user} ID is a system-generated Google Health API user ID, a string of 1-63 characters consisting of lowercase and uppercase letters, numbers, and hyphens. The literal `me` can also be used to refer to the authenticated user.
    pub name: Option<String>,
    /// Optional. The stride length type defined in the user's account settings for running. Updates to this field are currently not supported.
    #[serde(rename = "strideLengthRunningType")]
    pub stride_length_running_type: Option<String>,
    /// Optional. The stride length type defined in the user's account settings for walking. Updates to this field are currently not supported.
    #[serde(rename = "strideLengthWalkingType")]
    pub stride_length_walking_type: Option<String>,
    /// Optional. The measurement unit defined in the user's account settings.
    #[serde(rename = "swimUnit")]
    pub swim_unit: Option<String>,
    /// Optional. The measurement unit defined in the user's account settings.
    #[serde(rename = "temperatureUnit")]
    pub temperature_unit: Option<String>,
    /// Optional. The timezone defined in the user's account settings. This follows the IANA [Time Zone Database](https://www.iana.org/time-zones). Updates to this field are currently not supported.
    #[serde(rename = "timeZone")]
    pub time_zone: Option<String>,
    /// Optional. The user's timezone offset relative to UTC. Updates to this field are currently not supported.
    #[serde(rename = "utcOffset")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub utc_offset: Option<chrono::Duration>,
    /// Optional. The measurement unit defined in the user's account settings.
    #[serde(rename = "waterUnit")]
    pub water_unit: Option<String>,
    /// Optional. The measurement unit defined in the user's account settings.
    #[serde(rename = "weightUnit")]
    pub weight_unit: Option<String>,
}

impl common::RequestValue for Settings {}
impl common::ResponseResult for Settings {}

/// A sleep session possibly including stages.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Sleep {
    /// Output only. Creation time of this sleep observation.
    #[serde(rename = "createTime")]
    pub create_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Required. Observed sleep interval.
    pub interval: Option<SessionTimeInterval>,
    /// Optional. Sleep metadata: processing, main, manually edited, stages status.
    pub metadata: Option<SleepMetadata>,
    /// Optional. “Out of bed” segments that can overlap with sleep stages.
    #[serde(rename = "outOfBedSegments")]
    pub out_of_bed_segments: Option<Vec<OutOfBedSegment>>,
    /// Optional. List of non-overlapping contiguous sleep stage segments that cover the sleep period.
    pub stages: Option<Vec<SleepStage>>,
    /// Output only. Sleep summary: metrics and stages summary.
    pub summary: Option<SleepSummary>,
    /// Optional. SleepType: classic or stages.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Output only. Last update time of this sleep observation.
    #[serde(rename = "updateTime")]
    pub update_time: Option<chrono::DateTime<chrono::offset::Utc>>,
}

impl common::Part for Sleep {}

/// Additional information about how the sleep was processed.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SleepMetadata {
    /// Optional. Sleep identifier relevant in the context of the data source.
    #[serde(rename = "externalId")]
    pub external_id: Option<String>,
    /// Output only. Some sleeps autodetected by algorithms can be manually edited by users.
    #[serde(rename = "manuallyEdited")]
    pub manually_edited: Option<bool>,
    /// Output only. Naps are sleeps without stages and relatively short durations.
    pub nap: Option<bool>,
    /// Output only. Sleep and sleep stages algorithms finished processing. A `true` value indicates whether all data processing for the session is complete. A `false` value means sleep period is detected but sleep stages is still processing.
    pub processed: Option<bool>,
    /// Output only. Sleep stages algorithm processing status.
    #[serde(rename = "stagesStatus")]
    pub stages_status: Option<String>,
}

impl common::Part for SleepMetadata {}

/// Sleep stage segment.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SleepStage {
    /// Output only. Creation time of this sleep stages segment.
    #[serde(rename = "createTime")]
    pub create_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Required. Sleep stage end time.
    #[serde(rename = "endTime")]
    pub end_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Required. The offset of the user's local time at the end of the sleep stage relative to the Coordinated Universal Time (UTC).
    #[serde(rename = "endUtcOffset")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub end_utc_offset: Option<chrono::Duration>,
    /// Required. Sleep stage start time.
    #[serde(rename = "startTime")]
    pub start_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Required. The offset of the user's local time at the start of the sleep stage relative to the Coordinated Universal Time (UTC).
    #[serde(rename = "startUtcOffset")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub start_utc_offset: Option<chrono::Duration>,
    /// Required. Sleep stage type: AWAKE, DEEP, REM, LIGHT etc.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Output only. Last update time of this sleep stages segment.
    #[serde(rename = "updateTime")]
    pub update_time: Option<chrono::DateTime<chrono::offset::Utc>>,
}

impl common::Part for SleepStage {}

///  Sleep summary: metrics and stages summary.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SleepSummary {
    /// Output only. Minutes after wake up calculated by restlessness algorithm.
    #[serde(rename = "minutesAfterWakeUp")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub minutes_after_wake_up: Option<i64>,
    /// Output only. Total number of minutes asleep. For classic sleep it is the sum of ASLEEP stages (excluding AWAKE and RESTLESS). For "stages" sleep it is the sum of LIGHT, REM and DEEP stages (excluding AWAKE).
    #[serde(rename = "minutesAsleep")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub minutes_asleep: Option<i64>,
    /// Output only. Total number of minutes awake. It is a sum of all AWAKE stages.
    #[serde(rename = "minutesAwake")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub minutes_awake: Option<i64>,
    /// Output only. Delta between wake time and bedtime. It is the sum of all stages.
    #[serde(rename = "minutesInSleepPeriod")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub minutes_in_sleep_period: Option<i64>,
    /// Output only. Minutes to fall asleep calculated by restlessness algorithm.
    #[serde(rename = "minutesToFallAsleep")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub minutes_to_fall_asleep: Option<i64>,
    /// Output only. List of summaries (total duration and segment count) per each sleep stage type.
    #[serde(rename = "stagesSummary")]
    pub stages_summary: Option<Vec<StageSummary>>,
}

impl common::Part for SleepSummary {}

/// Represents splits or laps recorded within an exercise. Lap events partition a workout into segments based on criteria like distance, time, or calories.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SplitSummary {
    /// Output only. Lap time excluding the pauses.
    #[serde(rename = "activeDuration")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub active_duration: Option<chrono::Duration>,
    /// Required. Lap end time
    #[serde(rename = "endTime")]
    pub end_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Required. Lap end time offset from UTC
    #[serde(rename = "endUtcOffset")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub end_utc_offset: Option<chrono::Duration>,
    /// Required. Summary metrics for this split.
    #[serde(rename = "metricsSummary")]
    pub metrics_summary: Option<MetricsSummary>,
    /// Required. Method used to split the exercise laps. Users may manually mark the lap as complete even if the tracking is automatic.
    #[serde(rename = "splitType")]
    pub split_type: Option<String>,
    /// Required. Lap start time
    #[serde(rename = "startTime")]
    pub start_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Required. Lap start time offset from UTC
    #[serde(rename = "startUtcOffset")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub start_utc_offset: Option<chrono::Duration>,
}

impl common::Part for SplitSummary {}

/// Total duration and segment count for a stage.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct StageSummary {
    /// Output only. Number of sleep stages segments.
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// Output only. Total duration in minutes of a sleep stage.
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub minutes: Option<i64>,
    /// Output only. Sleep stage type: AWAKE, DEEP, REM, LIGHT etc.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl common::Part for StageSummary {}

/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Status {
    /// The status code, which should be an enum value of google.rpc.Code.
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    pub details: Option<Vec<HashMap<String, serde_json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    pub message: Option<String>,
}

impl common::Part for Status {}

/// Step count over the time interval.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Steps {
    /// Required. Number of steps in the recorded interval.
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// Required. Observed interval.
    pub interval: Option<ObservationTimeInterval>,
}

impl common::Part for Steps {}

/// Represents the result of the rollup of the steps data type.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct StepsRollupValue {
    /// Total number of steps in the interval.
    #[serde(rename = "countSum")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub count_sum: Option<i64>,
}

impl common::Part for StepsRollupValue {}

/// – Resource Messages – A subscriber receives notifications from Google Health API.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [subscribers patch projects](ProjectSubscriberPatchCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Subscriber {
    /// Output only. The time at which the subscriber was created.
    #[serde(rename = "createTime")]
    pub create_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Required. Authorization mechanism for a subscriber endpoint. This is required to ensure the endpoint can be verified.
    #[serde(rename = "endpointAuthorization")]
    pub endpoint_authorization: Option<EndpointAuthorization>,
    /// Required. The full HTTPS URI where update notifications will be sent. The URI must be a valid URL and use HTTPS as the scheme. This endpoint will be verified during CreateSubscriber and UpdateSubscriber calls. See RPC documentation for verification details.
    #[serde(rename = "endpointUri")]
    pub endpoint_uri: Option<String>,
    /// Identifier. The resource name of the Subscriber. Format: projects/{project}/subscribers/{subscriber} The {project} ID is a Google Cloud Project ID or Project Number. The {subscriber} ID is user-settable (4-36 characters, matching /[a-z]([a-z0-9-]{2,34}[a-z0-9])/) if provided during creation, or system-generated otherwise (e.g., a UUID). Example (User-settable subscriber ID): projects/my-project/subscribers/my-sub-123 Example (System-generated subscriber ID): projects/my-project/subscribers/a1b2c3d4-e5f6-7890-1234-567890abcdef
    pub name: Option<String>,
    /// Output only. The state of the subscriber.
    pub state: Option<String>,
    /// Optional. Configuration for the subscriber.
    #[serde(rename = "subscriberConfigs")]
    pub subscriber_configs: Option<Vec<SubscriberConfig>>,
    /// Output only. The time at which the subscriber was last updated.
    #[serde(rename = "updateTime")]
    pub update_time: Option<chrono::DateTime<chrono::offset::Utc>>,
}

impl common::RequestValue for Subscriber {}

/// Configuration for a subscriber. A notification is sent to a subscription ONLY if the subscriber has a config for the data type.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscriberConfig {
    /// Required. See [Google Health API data types](https://developers.google.com/health/data-types) for the list of supported data types. Values should be in kebab-case.
    #[serde(rename = "dataTypes")]
    pub data_types: Option<Vec<String>>,
    /// Required. Policy for subscription creation.
    #[serde(rename = "subscriptionCreatePolicy")]
    pub subscription_create_policy: Option<String>,
}

impl common::Part for SubscriberConfig {}

/// A subscription to a data collection for a specific user, to be delivered to a subscriber.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [subscribers subscriptions create projects](ProjectSubscriberSubscriptionCreateCall) (response)
/// * [subscribers subscriptions patch projects](ProjectSubscriberSubscriptionPatchCall) (request|response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Subscription {
    /// Optional. Data types subscribed to. A subscriber will only receive notifications for data types that are declared here. A subscription can only subscribe to the data types of the subscriber. The values should be in the format "users/{health_user_id}/dataTypes/{data_type}" where `{data_type}` is one of "altitude", "distance", "floors", "sleep", "steps", "weight".
    #[serde(rename = "dataTypes")]
    pub data_types: Option<Vec<String>>,
    /// Identifier. The resource name of the Subscription. Format: `projects/{project}/subscribers/{subscriber}/subscriptions/{subscription}` Example: `projects/my-project/subscribers/my-subscriber-123/subscriptions/my-subscription-456` The {project} ID is mandatory (6-30 characters, matching /a-z{6,30}/) The {subscriber} ID is user-settable (4-36 characters, matching /[a-z]([a-z0-9-]{2,34}[a-z0-9])/) if provided during creation, or system-generated otherwise. The {subscription} ID is user-settable (4-36 chars, matching /[a-z]([a-z0-9-]{2,34}[a-z0-9])/) or system-generated otherwise.
    pub name: Option<String>,
    /// Immutable. The resource name of the user for whom this subscription is active. Format: `users/{user}` where `{user}` is the public `healthUserId` as returned by the `GetIdentity` action in the profile PAPI (see `google.devicesandservices.health.v4main.HealthProfileService.GetIdentity`).
    pub user: Option<String>,
}

impl common::RequestValue for Subscription {}
impl common::ResponseResult for Subscription {}

/// Swim lengths data over the time interval.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SwimLengthsData {
    /// Required. Observed interval.
    pub interval: Option<ObservationTimeInterval>,
    /// Required. Number of strokes in the lap.
    #[serde(rename = "strokeCount")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub stroke_count: Option<i64>,
    /// Required. Swim stroke type.
    #[serde(rename = "swimStrokeType")]
    pub swim_stroke_type: Option<String>,
}

impl common::Part for SwimLengthsData {}

/// Represents the result of the rollup of the swim lengths data type.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SwimLengthsDataRollupValue {
    /// Total number of swim strokes in the interval.
    #[serde(rename = "strokeCountSum")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub stroke_count_sum: Option<i64>,
}

impl common::Part for SwimLengthsDataRollupValue {}

/// Time in heart rate zone record. It's an interval spent in specific heart rate zone.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TimeInHeartRateZone {
    /// Required. Heart rate zone type.
    #[serde(rename = "heartRateZoneType")]
    pub heart_rate_zone_type: Option<String>,
    /// Required. Observed interval.
    pub interval: Option<ObservationTimeInterval>,
}

impl common::Part for TimeInHeartRateZone {}

/// Represents the result of the rollup of the time in heart rate zone data type.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TimeInHeartRateZoneRollupValue {
    /// List of time spent in each heart rate zone.
    #[serde(rename = "timeInHeartRateZones")]
    pub time_in_heart_rate_zones: Option<Vec<TimeInHeartRateZoneValue>>,
}

impl common::Part for TimeInHeartRateZoneRollupValue {}

/// Represents the total time spent in a specific heart rate zone.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TimeInHeartRateZoneValue {
    /// The total time spent in the specified heart rate zone.
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub duration: Option<chrono::Duration>,
    /// The heart rate zone.
    #[serde(rename = "heartRateZone")]
    pub heart_rate_zone: Option<String>,
}

impl common::Part for TimeInHeartRateZoneValue {}

/// Time spent in each heart rate zone.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TimeInHeartRateZones {
    /// Optional. Time spent in light heart rate zone.
    #[serde(rename = "lightTime")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub light_time: Option<chrono::Duration>,
    /// Optional. Time spent in moderate heart rate zone.
    #[serde(rename = "moderateTime")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub moderate_time: Option<chrono::Duration>,
    /// Optional. Time spent in peak heart rate zone.
    #[serde(rename = "peakTime")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub peak_time: Option<chrono::Duration>,
    /// Optional. Time spent in vigorous heart rate zone.
    #[serde(rename = "vigorousTime")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub vigorous_time: Option<chrono::Duration>,
}

impl common::Part for TimeInHeartRateZones {}

/// Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TimeOfDay {
    /// Hours of a day in 24 hour format. Must be greater than or equal to 0 and typically must be less than or equal to 23. An API may choose to allow the value "24:00:00" for scenarios like business closing time.
    pub hours: Option<i32>,
    /// Minutes of an hour. Must be greater than or equal to 0 and less than or equal to 59.
    pub minutes: Option<i32>,
    /// Fractions of seconds, in nanoseconds. Must be greater than or equal to 0 and less than or equal to 999,999,999.
    pub nanos: Option<i32>,
    /// Seconds of a minute. Must be greater than or equal to 0 and typically must be less than or equal to 59. An API may allow the value 60 if it allows leap-seconds.
    pub seconds: Option<i32>,
}

impl common::Part for TimeOfDay {}

/// Represents the result of the rollup of the user's total calories.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TotalCaloriesRollupValue {
    /// Sum of the total calories in kilocalories.
    #[serde(rename = "kcalSum")]
    pub kcal_sum: Option<f64>,
}

impl common::Part for TotalCaloriesRollupValue {}

/// VO2 max measurement.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VO2Max {
    /// Optional. The method used to measure the VO2 max value.
    #[serde(rename = "measurementMethod")]
    pub measurement_method: Option<String>,
    /// Required. The time at which VO2 max was measured.
    #[serde(rename = "sampleTime")]
    pub sample_time: Option<ObservationSampleTime>,
    /// Required. VO2 max value measured as in ml consumed oxygen / kg of body weight / min.
    #[serde(rename = "vo2Max")]
    pub vo2_max: Option<f64>,
}

impl common::Part for VO2Max {}

/// Represents the volume quantity.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VolumeQuantity {
    /// Required. Value representing the volume in milliliters.
    pub milliliters: Option<f64>,
    /// Optional. Value representing the user provided unit, used only for user-facing input and display purposes. In the API format, all volume quantities are converted to milliliters.
    #[serde(rename = "userProvidedUnit")]
    pub user_provided_unit: Option<String>,
}

impl common::Part for VolumeQuantity {}

/// Rollup for volume quantity.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VolumeQuantityRollup {
    /// Required. The sum of volume in milliliters.
    #[serde(rename = "millilitersSum")]
    pub milliliters_sum: Option<f64>,
    /// Optional. The user provided unit on the last element.
    #[serde(rename = "userProvidedUnitLast")]
    pub user_provided_unit_last: Option<String>,
}

impl common::Part for VolumeQuantityRollup {}

/// Body weight measurement.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Weight {
    /// Optional. Standard free-form notes captured at manual logging.
    pub notes: Option<String>,
    /// Required. The time at which the weight was measured
    #[serde(rename = "sampleTime")]
    pub sample_time: Option<ObservationSampleTime>,
    /// Required. Weight of a user in grams.
    #[serde(rename = "weightGrams")]
    pub weight_grams: Option<f64>,
}

impl common::Part for Weight {}

/// Represents the weight quantity.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct WeightQuantity {
    /// Required. Value representing the weight in grams.
    pub grams: Option<f64>,
    /// Optional. Value representing the user provided unit.
    #[serde(rename = "userProvidedUnit")]
    pub user_provided_unit: Option<String>,
}

impl common::Part for WeightQuantity {}

/// Rollup for the weight.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct WeightQuantityRollup {
    /// Required. The sum of the weight in grams.
    #[serde(rename = "gramsSum")]
    pub grams_sum: Option<f64>,
    /// Optional. The user provided unit on the last element.
    #[serde(rename = "userProvidedUnitLast")]
    pub user_provided_unit_last: Option<String>,
}

impl common::Part for WeightQuantityRollup {}

/// Represents the result of the rollup of the weight data type.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct WeightRollupValue {
    /// Average weight in grams.
    #[serde(rename = "weightGramsAvg")]
    pub weight_grams_avg: Option<f64>,
}

impl common::Part for WeightRollupValue {}

// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`GoogleHealthAPI`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_health4 as health4;
///
/// # async fn dox() {
/// use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let connector = hyper_rustls::HttpsConnectorBuilder::new()
///     .with_native_roots()
///     .unwrap()
///     .https_only()
///     .enable_http2()
///     .build();
///
/// let executor = hyper_util::rt::TokioExecutor::new();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     yup_oauth2::client::CustomHyperClientBuilder::from(
///         hyper_util::client::legacy::Client::builder(executor).build(connector),
///     ),
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http2()
///         .build()
/// );
/// let mut hub = GoogleHealthAPI::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `subscribers_create(...)`, `subscribers_delete(...)`, `subscribers_list(...)`, `subscribers_patch(...)`, `subscribers_subscriptions_create(...)`, `subscribers_subscriptions_delete(...)`, `subscribers_subscriptions_list(...)` and `subscribers_subscriptions_patch(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
}

impl<'a, C> common::MethodsBuilder for ProjectMethods<'a, C> {}

impl<'a, C> ProjectMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a subscription for a specific user to a specific subscriber. This method requires the subscriber to have a `SubscriptionCreatePolicy` set to `MANUAL` for the given data types.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent subscriber. Format: projects/{project}/subscribers/{subscriber} The {subscriber} ID is user-settable (4-36 characters, matching /[a-z]([a-z0-9-]{2,34}[a-z0-9])/) if provided during creation, or system-generated otherwise.
    pub fn subscribers_subscriptions_create(
        &self,
        request: CreateSubscriptionPayload,
        parent: &str,
    ) -> ProjectSubscriberSubscriptionCreateCall<'a, C> {
        ProjectSubscriberSubscriptionCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _subscription_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a specific user subscription, stopping notifications for this user to this subscriber.
    ///
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the subscription to delete. Format: `projects/{project}/subscribers/{subscriber}/subscriptions/{subscription}` Example: `projects/my-project/subscribers/my-subscriber-123/subscriptions/my-subscription-456` The {subscriber} ID is user-settable (4-36 characters, matching /[a-z]([a-z0-9-]{2,34}[a-z0-9])/) if provided during creation, or system-generated otherwise. The {subscription} ID is user-settable (4-36 characters, matching /[a-z]([a-z0-9-]{2,34}[a-z0-9])/) or system-generated if not provided during creation.
    pub fn subscribers_subscriptions_delete(
        &self,
        name: &str,
    ) -> ProjectSubscriberSubscriptionDeleteCall<'a, C> {
        ProjectSubscriberSubscriptionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Lists all active subscriptions for a given subscriber. This can be filtered, for example, by user or data type.
    ///
    /// # Arguments
    ///
    /// * `parent` - Required. The parent subscriber. Format: projects/{project}/subscribers/{subscriber} The {subscriber} ID is user-settable (4-36 characters, matching /[a-z]([a-z0-9-]{2,34}[a-z0-9])/) if provided during creation, or system-generated otherwise.
    pub fn subscribers_subscriptions_list(
        &self,
        parent: &str,
    ) -> ProjectSubscriberSubscriptionListCall<'a, C> {
        ProjectSubscriberSubscriptionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Updates the data types for an existing user subscription.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Identifier. The resource name of the Subscription. Format: `projects/{project}/subscribers/{subscriber}/subscriptions/{subscription}` Example: `projects/my-project/subscribers/my-subscriber-123/subscriptions/my-subscription-456` The {project} ID is mandatory (6-30 characters, matching /a-z{6,30}/) The {subscriber} ID is user-settable (4-36 characters, matching /[a-z]([a-z0-9-]{2,34}[a-z0-9])/) if provided during creation, or system-generated otherwise. The {subscription} ID is user-settable (4-36 chars, matching /[a-z]([a-z0-9-]{2,34}[a-z0-9])/) or system-generated otherwise.
    pub fn subscribers_subscriptions_patch(
        &self,
        request: Subscription,
        name: &str,
    ) -> ProjectSubscriberSubscriptionPatchCall<'a, C> {
        ProjectSubscriberSubscriptionPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Registers a new subscriber endpoint to receive notifications. A subscriber represents an application or service that wishes to receive data change notifications for users who have granted consent. **Endpoint Verification:** For a subscriber to be successfully created, the provided `endpoint_uri` must be a valid HTTPS endpoint and must pass an automated verification check. The backend will send two HTTP POST requests to the `endpoint_uri`: 1. **Verification with Authorization:** * **Headers:** Includes `Content-Type: application/json` and `Authorization` (with the exact value from `CreateSubscriberPayload.endpoint_authorization.secret`). * **Body:** `{"type": "verification"}` * **Expected Response:** HTTP `201 Created`. 2. **Verification without Authorization:** * **Headers:** Includes `Content-Type: application/json`. The `Authorization` header is OMITTED. * **Body:** `{"type": "verification"}` * **Expected Response:** HTTP `401 Unauthorized` or `403 Forbidden`. Both tests must pass for the subscriber creation to succeed. If verification fails, the operation will not be completed and an error will be returned. This process ensures the endpoint is reachable and correctly validates the `Authorization` header.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource where this subscriber will be created. Format: projects/{project_number} Example: projects/1234567890
    pub fn subscribers_create(
        &self,
        request: CreateSubscriberPayload,
        parent: &str,
    ) -> ProjectSubscriberCreateCall<'a, C> {
        ProjectSubscriberCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _subscriber_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a subscriber registration. This will stop all notifications to the subscriber's endpoint.
    ///
    /// # Arguments
    ///
    /// * `name` - Required. The name of the subscriber to delete. Format: projects/{project}/subscribers/{subscriber} Example: projects/my-project/subscribers/my-subscriber-123 The {subscriber} ID is user-settable (4-36 characters, matching /[a-z]([a-z0-9-]{2,34}[a-z0-9])/) or system-generated if not provided during creation.
    pub fn subscribers_delete(&self, name: &str) -> ProjectSubscriberDeleteCall<'a, C> {
        ProjectSubscriberDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _force: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Lists all subscribers registered within the owned Google Cloud Project.
    ///
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns this collection of subscribers. Format: projects/{project}
    pub fn subscribers_list(&self, parent: &str) -> ProjectSubscriberListCall<'a, C> {
        ProjectSubscriberListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Updates the configuration of an existing subscriber, such as the endpoint URI or the data types it's interested in. **Endpoint Verification:** If the `endpoint_uri` or `endpoint_authorization` field is included in the `update_mask`, the backend will re-verify the endpoint. The verification process is the same as described in `CreateSubscriber`: 1. **Verification with Authorization:** POST to the new or existing `endpoint_uri` with the new or existing `Authorization` secret. Expects HTTP `201 Created`. 2. **Verification without Authorization:** POST to the `endpoint_uri` without the `Authorization` header. Expects HTTP `401 Unauthorized` or `403 Forbidden`. Both tests must pass using the potentially updated values for the subscriber update to succeed. If verification fails, the update will not be applied, and an error will be returned.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Identifier. The resource name of the Subscriber. Format: projects/{project}/subscribers/{subscriber} The {project} ID is a Google Cloud Project ID or Project Number. The {subscriber} ID is user-settable (4-36 characters, matching /[a-z]([a-z0-9-]{2,34}[a-z0-9])/) if provided during creation, or system-generated otherwise (e.g., a UUID). Example (User-settable subscriber ID): projects/my-project/subscribers/my-sub-123 Example (System-generated subscriber ID): projects/my-project/subscribers/a1b2c3d4-e5f6-7890-1234-567890abcdef
    pub fn subscribers_patch(
        &self,
        request: Subscriber,
        name: &str,
    ) -> ProjectSubscriberPatchCall<'a, C> {
        ProjectSubscriberPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *user* resources.
/// It is not used directly, but through the [`GoogleHealthAPI`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_health4 as health4;
///
/// # async fn dox() {
/// use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let connector = hyper_rustls::HttpsConnectorBuilder::new()
///     .with_native_roots()
///     .unwrap()
///     .https_only()
///     .enable_http2()
///     .build();
///
/// let executor = hyper_util::rt::TokioExecutor::new();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     yup_oauth2::client::CustomHyperClientBuilder::from(
///         hyper_util::client::legacy::Client::builder(executor).build(connector),
///     ),
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http2()
///         .build()
/// );
/// let mut hub = GoogleHealthAPI::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `data_types_data_points_batch_delete(...)`, `data_types_data_points_create(...)`, `data_types_data_points_daily_roll_up(...)`, `data_types_data_points_export_exercise_tcx(...)`, `data_types_data_points_get(...)`, `data_types_data_points_list(...)`, `data_types_data_points_patch(...)`, `data_types_data_points_reconcile(...)`, `data_types_data_points_roll_up(...)`, `get_identity(...)`, `get_irn_profile(...)`, `get_profile(...)`, `get_settings(...)`, `paired_devices_get(...)`, `paired_devices_list(...)`, `update_profile(...)` and `update_settings(...)`
/// // to build up your call.
/// let rb = hub.users();
/// # }
/// ```
pub struct UserMethods<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
}

impl<'a, C> common::MethodsBuilder for UserMethods<'a, C> {}

impl<'a, C> UserMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a batch of identifyable data points.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Optional. Parent (data type) for the Data Point collection Format: `users/me/dataTypes/{data_type}`, e.g.: - `users/me/dataTypes/steps` - `users/me/dataTypes/-` For a list of the supported data types see the DataPoint data union field. Deleting data points across multiple data type collections is supported following https://aip.dev/159. If this is set, the parent of all of the data points specified in `names` must match this field.
    pub fn data_types_data_points_batch_delete(
        &self,
        request: BatchDeleteDataPointsRequest,
        parent: &str,
    ) -> UserDataTypeDataPointBatchDeleteCall<'a, C> {
        UserDataTypeDataPointBatchDeleteCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Creates a single identifiable data point.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource name where the data point will be created. Format: `users/{user}/dataTypes/{data_type}`
    pub fn data_types_data_points_create(
        &self,
        request: DataPoint,
        parent: &str,
    ) -> UserDataTypeDataPointCreateCall<'a, C> {
        UserDataTypeDataPointCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Roll up data points over civil time intervals for supported data types.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent data type of the Data Point collection. Format: `users/{user}/dataTypes/{data_type}`, e.g.: - `users/me/dataTypes/steps` - `users/me/dataTypes/distance` For a list of the supported data types see the DailyRollupDataPoint value union field.
    pub fn data_types_data_points_daily_roll_up(
        &self,
        request: DailyRollUpDataPointsRequest,
        parent: &str,
    ) -> UserDataTypeDataPointDailyRollUpCall<'a, C> {
        UserDataTypeDataPointDailyRollUpCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Exports exercise data in TCX format. **IMPORTANT:** HTTP clients must append `?alt=media` to the request URL to download the raw TCX file. Example: `https://health.googleapis.com/v4/users/me/dataTypes/exercise/dataPoints/EXERCISE_ID:exportExerciseTcx?alt=media` Without `alt=media`, the server returns a JSON response (`ExportExerciseTcxResponse`) which is intended primarily for gRPC clients. **Note:** While the Authorization section below states that any one of the listed scopes is accepted, this specific method requires the user to provide both one of the `activity_and_fitness` scopes (`normal` or `readonly`) AND one of the `location` scopes (`normal` or `readonly`) in their access token to succeed.
    ///
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the exercise data point to export. Format: `users/{user}/dataTypes/exercise/dataPoints/{data_point}` Example: `users/me/dataTypes/exercise/dataPoints/2026443605080188808` The `{user}` is the alias `"me"` currently. Future versions may support user IDs. The `{data_point}` ID maps to the exercise ID, which is a long integer.
    pub fn data_types_data_points_export_exercise_tcx(
        &self,
        name: &str,
    ) -> UserDataTypeDataPointExportExerciseTcxCall<'a, C> {
        UserDataTypeDataPointExportExerciseTcxCall {
            hub: self.hub,
            _name: name.to_string(),
            _partial_data: Default::default(),
            _delegate: Default::default(),
            _range: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Get a single identifyable data point.
    ///
    /// # Arguments
    ///
    /// * `name` - Required. The name of the data point to retrieve. Format: `users/{user}/dataTypes/{data_type}/dataPoints/{data_point}` See DataPoint.name for examples and possible values.
    pub fn data_types_data_points_get(&self, name: &str) -> UserDataTypeDataPointGetCall<'a, C> {
        UserDataTypeDataPointGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Query user health and fitness data points.
    ///
    /// # Arguments
    ///
    /// * `parent` - Required. Parent data type of the Data Point collection. Format: `users/me/dataTypes/{data_type}`, e.g.: - `users/me/dataTypes/steps` - `users/me/dataTypes/weight` For a list of the supported data types see the DataPoint data union field.
    pub fn data_types_data_points_list(
        &self,
        parent: &str,
    ) -> UserDataTypeDataPointListCall<'a, C> {
        UserDataTypeDataPointListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Updates a single identifiable data point. If a data point with the specified `name` is not found, the request will fail.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Identifier. Data point name, only supported for the subset of identifiable data types. For the majority of the data types, individual data points do not need to be identified and this field would be empty. Format: `users/{user}/dataTypes/{data_type}/dataPoints/{data_point}` Example: `users/abcd1234/dataTypes/sleep/dataPoints/a1b2c3d4-e5f6-7890-1234-567890abcdef` The `{user}` ID is a system-generated identifier, as described in Identity.health_user_id. The `{data_type}` ID corresponds to the kebab-case version of the field names in the DataPoint data union field, e.g. `heart-rate` for the `heart_rate` field. The `{data_point}` ID can be client-provided or system-generated. If client-provided, it must be a string of 4-63 characters, containing only lowercase letters, numbers, and hyphens.
    pub fn data_types_data_points_patch(
        &self,
        request: DataPoint,
        name: &str,
    ) -> UserDataTypeDataPointPatchCall<'a, C> {
        UserDataTypeDataPointPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Reconcile data points from multiple data sources into a single data stream.
    ///
    /// # Arguments
    ///
    /// * `parent` - Required. Parent data type of the Data Point collection. Format: `users/me/dataTypes/{data_type}`, e.g.: - `users/me/dataTypes/steps` - `users/me/dataTypes/heart-rate` For a list of the supported data types see the DataPoint data union field.
    pub fn data_types_data_points_reconcile(
        &self,
        parent: &str,
    ) -> UserDataTypeDataPointReconcileCall<'a, C> {
        UserDataTypeDataPointReconcileCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _data_source_family: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Roll up data points over physical time intervals for supported data types.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent data type of the Data Point collection. Format: `users/{user}/dataTypes/{data_type}`, e.g.: - `users/me/dataTypes/steps` - `users/me/dataTypes/distance` For a list of the supported data types see the RollupDataPoint value union field.
    pub fn data_types_data_points_roll_up(
        &self,
        request: RollUpDataPointsRequest,
        parent: &str,
    ) -> UserDataTypeDataPointRollUpCall<'a, C> {
        UserDataTypeDataPointRollUpCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Returns user's Device.
    ///
    /// # Arguments
    ///
    /// * `name` - Required. The name of the device to retrieve. Format: users/{user}/devices/{device}
    pub fn paired_devices_get(&self, name: &str) -> UserPairedDeviceGetCall<'a, C> {
        UserPairedDeviceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Returns the user's list of paired 1P trackers and smartwatches.
    ///
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns this collection of devices. Format: users/{user}
    pub fn paired_devices_list(&self, parent: &str) -> UserPairedDeviceListCall<'a, C> {
        UserPairedDeviceListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Gets the user's identity. It includes the legacy Fitbit user ID and the Google user ID and it can be used by migrating clients to map identifiers between the two systems.
    ///
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the Identity. Format: `users/me/identity`
    pub fn get_identity(&self, name: &str) -> UserGetIdentityCall<'a, C> {
        UserGetIdentityCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Returns user's IRN Profile details.
    ///
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the IRN Profile. Format: `users/{user}/irnProfile` Example: `users/1234567890/irnProfile` or `users/me/irnProfile` The {user} ID is a system-generated Google Health API user ID, a string of 1-63 characters consisting of lowercase and uppercase letters, numbers, and hyphens. The literal `me` can also be used to refer to the authenticated user.
    pub fn get_irn_profile(&self, name: &str) -> UserGetIrnProfileCall<'a, C> {
        UserGetIrnProfileCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Returns user Profile details.
    ///
    /// # Arguments
    ///
    /// * `name` - Required. The name of the Profile. Format: `users/me/profile`.
    pub fn get_profile(&self, name: &str) -> UserGetProfileCall<'a, C> {
        UserGetProfileCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Returns user settings details.
    ///
    /// # Arguments
    ///
    /// * `name` - Required. The name of the Settings. Format: `users/me/settings`.
    pub fn get_settings(&self, name: &str) -> UserGetSettingCall<'a, C> {
        UserGetSettingCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Updates the user's profile details.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Identifier. The resource name of this Profile resource. Format: `users/{user}/profile` Example: `users/1234567890/profile` or `users/me/profile` The {user} ID is a system-generated Google Health API user ID, a string of 1-63 characters consisting of lowercase and uppercase letters, numbers, and hyphens. The literal `me` can also be used to refer to the authenticated user.
    pub fn update_profile(&self, request: Profile, name: &str) -> UserUpdateProfileCall<'a, C> {
        UserUpdateProfileCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Updates the user's settings details.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Identifier. The resource name of this Settings resource. Format: `users/{user}/settings` Example: `users/1234567890/settings` or `users/me/settings` The {user} ID is a system-generated Google Health API user ID, a string of 1-63 characters consisting of lowercase and uppercase letters, numbers, and hyphens. The literal `me` can also be used to refer to the authenticated user.
    pub fn update_settings(&self, request: Settings, name: &str) -> UserUpdateSettingCall<'a, C> {
        UserUpdateSettingCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

// ###################
// CallBuilders   ###
// #################

/// Creates a subscription for a specific user to a specific subscriber. This method requires the subscriber to have a `SubscriptionCreatePolicy` set to `MANUAL` for the given data types.
///
/// A builder for the *subscribers.subscriptions.create* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// use health4::api::CreateSubscriptionPayload;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CreateSubscriptionPayload::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().subscribers_subscriptions_create(req, "parent")
///              .subscription_id("sed")
///              .doit().await;
/// # }
/// ```
pub struct ProjectSubscriberSubscriptionCreateCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _request: CreateSubscriptionPayload,
    _parent: String,
    _subscription_id: Option<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectSubscriberSubscriptionCreateCall<'a, C> {}

impl<'a, C> ProjectSubscriberSubscriptionCreateCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Subscription)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.projects.subscribers.subscriptions.create",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "parent", "subscriptionId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("parent", self._parent);
        if let Some(value) = self._subscription_id.as_ref() {
            params.push("subscriptionId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+parent}/subscriptions";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["parent"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(
        mut self,
        new_value: CreateSubscriptionPayload,
    ) -> ProjectSubscriberSubscriptionCreateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The parent subscriber. Format: projects/{project}/subscribers/{subscriber} The {subscriber} ID is user-settable (4-36 characters, matching /[a-z]([a-z0-9-]{2,34}[a-z0-9])/) if provided during creation, or system-generated otherwise.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(
        mut self,
        new_value: impl Into<String>,
    ) -> ProjectSubscriberSubscriptionCreateCall<'a, C> {
        self._parent = new_value.into();
        self
    }
    /// Optional. The {subscription_id} is user-settable (4-36 chars, matching /[a-z]([a-z0-9-]{2,34}[a-z0-9])/) or system-generated otherwise. If provided, the ID must be unique within the parent subscriber.
    ///
    /// Sets the *subscription id* query property to the given value.
    pub fn subscription_id(
        mut self,
        new_value: impl Into<String>,
    ) -> ProjectSubscriberSubscriptionCreateCall<'a, C> {
        self._subscription_id = Some(new_value.into());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectSubscriberSubscriptionCreateCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectSubscriberSubscriptionCreateCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectSubscriberSubscriptionCreateCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectSubscriberSubscriptionCreateCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectSubscriberSubscriptionCreateCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Deletes a specific user subscription, stopping notifications for this user to this subscriber.
///
/// A builder for the *subscribers.subscriptions.delete* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().subscribers_subscriptions_delete("name")
///              .doit().await;
/// # }
/// ```
pub struct ProjectSubscriberSubscriptionDeleteCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _name: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectSubscriberSubscriptionDeleteCall<'a, C> {}

impl<'a, C> ProjectSubscriberSubscriptionDeleteCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Empty)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.projects.subscribers.subscriptions.delete",
            http_method: hyper::Method::DELETE,
        });

        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("name", self._name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+name}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::DELETE)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// Required. The resource name of the subscription to delete. Format: `projects/{project}/subscribers/{subscriber}/subscriptions/{subscription}` Example: `projects/my-project/subscribers/my-subscriber-123/subscriptions/my-subscription-456` The {subscriber} ID is user-settable (4-36 characters, matching /[a-z]([a-z0-9-]{2,34}[a-z0-9])/) if provided during creation, or system-generated otherwise. The {subscription} ID is user-settable (4-36 characters, matching /[a-z]([a-z0-9-]{2,34}[a-z0-9])/) or system-generated if not provided during creation.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(
        mut self,
        new_value: impl Into<String>,
    ) -> ProjectSubscriberSubscriptionDeleteCall<'a, C> {
        self._name = new_value.into();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectSubscriberSubscriptionDeleteCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectSubscriberSubscriptionDeleteCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectSubscriberSubscriptionDeleteCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectSubscriberSubscriptionDeleteCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectSubscriberSubscriptionDeleteCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Lists all active subscriptions for a given subscriber. This can be filtered, for example, by user or data type.
///
/// A builder for the *subscribers.subscriptions.list* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().subscribers_subscriptions_list("parent")
///              .page_token("amet.")
///              .page_size(-20)
///              .filter("ipsum")
///              .doit().await;
/// # }
/// ```
pub struct ProjectSubscriberSubscriptionListCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _parent: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _filter: Option<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectSubscriberSubscriptionListCall<'a, C> {}

impl<'a, C> ProjectSubscriberSubscriptionListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, ListSubscriptionsResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.projects.subscribers.subscriptions.list",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "parent", "pageToken", "pageSize", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("parent", self._parent);
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._filter.as_ref() {
            params.push("filter", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+parent}/subscriptions";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["parent"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// Required. The parent subscriber. Format: projects/{project}/subscribers/{subscriber} The {subscriber} ID is user-settable (4-36 characters, matching /[a-z]([a-z0-9-]{2,34}[a-z0-9])/) if provided during creation, or system-generated otherwise.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(
        mut self,
        new_value: impl Into<String>,
    ) -> ProjectSubscriberSubscriptionListCall<'a, C> {
        self._parent = new_value.into();
        self
    }
    /// Optional. A page token, received from a previous `ListSubscriptions` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListSubscriptions` must match the call that provided the page token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(
        mut self,
        new_value: impl Into<String>,
    ) -> ProjectSubscriberSubscriptionListCall<'a, C> {
        self._page_token = Some(new_value.into());
        self
    }
    /// Optional. The maximum number of subscriptions to return. The service may return fewer than this value. If unspecified, at most 50 subscriptions will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ProjectSubscriberSubscriptionListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// Optional. A filter to apply to the list of subscriptions. The filter syntax is described in https://google.aip.dev/160. The filter can be applied to the following fields: - `user` - `data_type` The `user` identifier (e.g., `user1` in `users/user1`) refers to the public `health_user_id` Example: user = "users/user1" Example: user = "users/user1" OR user = "users/user2" Example: user = "users/user1" AND (data_type = "sleep" OR data_type = "weight")
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(
        mut self,
        new_value: impl Into<String>,
    ) -> ProjectSubscriberSubscriptionListCall<'a, C> {
        self._filter = Some(new_value.into());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectSubscriberSubscriptionListCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectSubscriberSubscriptionListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectSubscriberSubscriptionListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectSubscriberSubscriptionListCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectSubscriberSubscriptionListCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Updates the data types for an existing user subscription.
///
/// A builder for the *subscribers.subscriptions.patch* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// use health4::api::Subscription;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Subscription::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().subscribers_subscriptions_patch(req, "name")
///              .update_mask(FieldMask::new::<&str>(&[]))
///              .doit().await;
/// # }
/// ```
pub struct ProjectSubscriberSubscriptionPatchCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _request: Subscription,
    _name: String,
    _update_mask: Option<common::FieldMask>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectSubscriberSubscriptionPatchCall<'a, C> {}

impl<'a, C> ProjectSubscriberSubscriptionPatchCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Subscription)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.projects.subscribers.subscriptions.patch",
            http_method: hyper::Method::PATCH,
        });

        for &field in ["alt", "name", "updateMask"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("name", self._name);
        if let Some(value) = self._update_mask.as_ref() {
            params.push("updateMask", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+name}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PATCH)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(
        mut self,
        new_value: Subscription,
    ) -> ProjectSubscriberSubscriptionPatchCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Identifier. The resource name of the Subscription. Format: `projects/{project}/subscribers/{subscriber}/subscriptions/{subscription}` Example: `projects/my-project/subscribers/my-subscriber-123/subscriptions/my-subscription-456` The {project} ID is mandatory (6-30 characters, matching /a-z{6,30}/) The {subscriber} ID is user-settable (4-36 characters, matching /[a-z]([a-z0-9-]{2,34}[a-z0-9])/) if provided during creation, or system-generated otherwise. The {subscription} ID is user-settable (4-36 chars, matching /[a-z]([a-z0-9-]{2,34}[a-z0-9])/) or system-generated otherwise.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(
        mut self,
        new_value: impl Into<String>,
    ) -> ProjectSubscriberSubscriptionPatchCall<'a, C> {
        self._name = new_value.into();
        self
    }
    /// Optional. The list of fields to update.
    ///
    /// Sets the *update mask* query property to the given value.
    pub fn update_mask(
        mut self,
        new_value: common::FieldMask,
    ) -> ProjectSubscriberSubscriptionPatchCall<'a, C> {
        self._update_mask = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectSubscriberSubscriptionPatchCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectSubscriberSubscriptionPatchCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectSubscriberSubscriptionPatchCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectSubscriberSubscriptionPatchCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectSubscriberSubscriptionPatchCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Registers a new subscriber endpoint to receive notifications. A subscriber represents an application or service that wishes to receive data change notifications for users who have granted consent. **Endpoint Verification:** For a subscriber to be successfully created, the provided `endpoint_uri` must be a valid HTTPS endpoint and must pass an automated verification check. The backend will send two HTTP POST requests to the `endpoint_uri`: 1. **Verification with Authorization:** * **Headers:** Includes `Content-Type: application/json` and `Authorization` (with the exact value from `CreateSubscriberPayload.endpoint_authorization.secret`). * **Body:** `{"type": "verification"}` * **Expected Response:** HTTP `201 Created`. 2. **Verification without Authorization:** * **Headers:** Includes `Content-Type: application/json`. The `Authorization` header is OMITTED. * **Body:** `{"type": "verification"}` * **Expected Response:** HTTP `401 Unauthorized` or `403 Forbidden`. Both tests must pass for the subscriber creation to succeed. If verification fails, the operation will not be completed and an error will be returned. This process ensures the endpoint is reachable and correctly validates the `Authorization` header.
///
/// A builder for the *subscribers.create* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// use health4::api::CreateSubscriberPayload;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CreateSubscriberPayload::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().subscribers_create(req, "parent")
///              .subscriber_id("gubergren")
///              .doit().await;
/// # }
/// ```
pub struct ProjectSubscriberCreateCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _request: CreateSubscriberPayload,
    _parent: String,
    _subscriber_id: Option<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectSubscriberCreateCall<'a, C> {}

impl<'a, C> ProjectSubscriberCreateCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Operation)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.projects.subscribers.create",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "parent", "subscriberId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("parent", self._parent);
        if let Some(value) = self._subscriber_id.as_ref() {
            params.push("subscriberId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+parent}/subscribers";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["parent"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(
        mut self,
        new_value: CreateSubscriberPayload,
    ) -> ProjectSubscriberCreateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The parent resource where this subscriber will be created. Format: projects/{project_number} Example: projects/1234567890
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: impl Into<String>) -> ProjectSubscriberCreateCall<'a, C> {
        self._parent = new_value.into();
        self
    }
    /// Optional. The ID to use for the subscriber, which will become the final component of the subscriber’s resource name. This value should be 4-36 characters, and valid characters are /[a-z]([a-z0-9-]{2,34}[a-z0-9])/.
    ///
    /// Sets the *subscriber id* query property to the given value.
    pub fn subscriber_id(
        mut self,
        new_value: impl Into<String>,
    ) -> ProjectSubscriberCreateCall<'a, C> {
        self._subscriber_id = Some(new_value.into());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectSubscriberCreateCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectSubscriberCreateCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectSubscriberCreateCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectSubscriberCreateCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectSubscriberCreateCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Deletes a subscriber registration. This will stop all notifications to the subscriber's endpoint.
///
/// A builder for the *subscribers.delete* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().subscribers_delete("name")
///              .force(true)
///              .doit().await;
/// # }
/// ```
pub struct ProjectSubscriberDeleteCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _name: String,
    _force: Option<bool>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectSubscriberDeleteCall<'a, C> {}

impl<'a, C> ProjectSubscriberDeleteCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Operation)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.projects.subscribers.delete",
            http_method: hyper::Method::DELETE,
        });

        for &field in ["alt", "name", "force"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("name", self._name);
        if let Some(value) = self._force.as_ref() {
            params.push("force", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+name}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::DELETE)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// Required. The name of the subscriber to delete. Format: projects/{project}/subscribers/{subscriber} Example: projects/my-project/subscribers/my-subscriber-123 The {subscriber} ID is user-settable (4-36 characters, matching /[a-z]([a-z0-9-]{2,34}[a-z0-9])/) or system-generated if not provided during creation.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: impl Into<String>) -> ProjectSubscriberDeleteCall<'a, C> {
        self._name = new_value.into();
        self
    }
    /// Optional. If set to true, any child resources (e.g., subscriptions) will also be deleted. If false (default) and child resources exist, the request will fail.
    ///
    /// Sets the *force* query property to the given value.
    pub fn force(mut self, new_value: bool) -> ProjectSubscriberDeleteCall<'a, C> {
        self._force = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectSubscriberDeleteCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectSubscriberDeleteCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectSubscriberDeleteCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectSubscriberDeleteCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectSubscriberDeleteCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Lists all subscribers registered within the owned Google Cloud Project.
///
/// A builder for the *subscribers.list* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().subscribers_list("parent")
///              .page_token("amet")
///              .page_size(-20)
///              .doit().await;
/// # }
/// ```
pub struct ProjectSubscriberListCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _parent: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectSubscriberListCall<'a, C> {}

impl<'a, C> ProjectSubscriberListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, ListSubscribersResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.projects.subscribers.list",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "parent", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("parent", self._parent);
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+parent}/subscribers";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["parent"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// Required. The parent, which owns this collection of subscribers. Format: projects/{project}
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: impl Into<String>) -> ProjectSubscriberListCall<'a, C> {
        self._parent = new_value.into();
        self
    }
    /// Optional. A page token, received from a previous `ListSubscribers` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListSubscribers` must match the call that provided the page token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: impl Into<String>) -> ProjectSubscriberListCall<'a, C> {
        self._page_token = Some(new_value.into());
        self
    }
    /// Optional. The maximum number of subscribers to return. The service may return fewer than this value. If unspecified, at most 50 subscribers will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ProjectSubscriberListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectSubscriberListCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectSubscriberListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectSubscriberListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectSubscriberListCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectSubscriberListCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Updates the configuration of an existing subscriber, such as the endpoint URI or the data types it's interested in. **Endpoint Verification:** If the `endpoint_uri` or `endpoint_authorization` field is included in the `update_mask`, the backend will re-verify the endpoint. The verification process is the same as described in `CreateSubscriber`: 1. **Verification with Authorization:** POST to the new or existing `endpoint_uri` with the new or existing `Authorization` secret. Expects HTTP `201 Created`. 2. **Verification without Authorization:** POST to the `endpoint_uri` without the `Authorization` header. Expects HTTP `401 Unauthorized` or `403 Forbidden`. Both tests must pass using the potentially updated values for the subscriber update to succeed. If verification fails, the update will not be applied, and an error will be returned.
///
/// A builder for the *subscribers.patch* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// use health4::api::Subscriber;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Subscriber::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().subscribers_patch(req, "name")
///              .update_mask(FieldMask::new::<&str>(&[]))
///              .doit().await;
/// # }
/// ```
pub struct ProjectSubscriberPatchCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _request: Subscriber,
    _name: String,
    _update_mask: Option<common::FieldMask>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectSubscriberPatchCall<'a, C> {}

impl<'a, C> ProjectSubscriberPatchCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Operation)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.projects.subscribers.patch",
            http_method: hyper::Method::PATCH,
        });

        for &field in ["alt", "name", "updateMask"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("name", self._name);
        if let Some(value) = self._update_mask.as_ref() {
            params.push("updateMask", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+name}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PATCH)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Subscriber) -> ProjectSubscriberPatchCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Identifier. The resource name of the Subscriber. Format: projects/{project}/subscribers/{subscriber} The {project} ID is a Google Cloud Project ID or Project Number. The {subscriber} ID is user-settable (4-36 characters, matching /[a-z]([a-z0-9-]{2,34}[a-z0-9])/) if provided during creation, or system-generated otherwise (e.g., a UUID). Example (User-settable subscriber ID): projects/my-project/subscribers/my-sub-123 Example (System-generated subscriber ID): projects/my-project/subscribers/a1b2c3d4-e5f6-7890-1234-567890abcdef
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: impl Into<String>) -> ProjectSubscriberPatchCall<'a, C> {
        self._name = new_value.into();
        self
    }
    /// Optional. A field mask that specifies which fields of the Subscriber message are to be updated. This allows for partial updates. Supported fields: - endpoint_uri - subscriber_configs - endpoint_authorization
    ///
    /// Sets the *update mask* query property to the given value.
    pub fn update_mask(
        mut self,
        new_value: common::FieldMask,
    ) -> ProjectSubscriberPatchCall<'a, C> {
        self._update_mask = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectSubscriberPatchCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectSubscriberPatchCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectSubscriberPatchCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectSubscriberPatchCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectSubscriberPatchCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Delete a batch of identifyable data points.
///
/// A builder for the *dataTypes.dataPoints.batchDelete* method supported by a *user* resource.
/// It is not used directly, but through a [`UserMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// use health4::api::BatchDeleteDataPointsRequest;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BatchDeleteDataPointsRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().data_types_data_points_batch_delete(req, "parent")
///              .doit().await;
/// # }
/// ```
pub struct UserDataTypeDataPointBatchDeleteCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _request: BatchDeleteDataPointsRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for UserDataTypeDataPointBatchDeleteCall<'a, C> {}

impl<'a, C> UserDataTypeDataPointBatchDeleteCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Operation)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.users.dataTypes.dataPoints.batchDelete",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("parent", self._parent);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+parent}/dataPoints:batchDelete";
        if self._scopes.is_empty() {
            self._scopes.insert(
                Scope::GooglehealthActivityAndFitnesWriteonly
                    .as_ref()
                    .to_string(),
            );
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["parent"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(
        mut self,
        new_value: BatchDeleteDataPointsRequest,
    ) -> UserDataTypeDataPointBatchDeleteCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Optional. Parent (data type) for the Data Point collection Format: `users/me/dataTypes/{data_type}`, e.g.: - `users/me/dataTypes/steps` - `users/me/dataTypes/-` For a list of the supported data types see the DataPoint data union field. Deleting data points across multiple data type collections is supported following https://aip.dev/159. If this is set, the parent of all of the data points specified in `names` must match this field.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(
        mut self,
        new_value: impl Into<String>,
    ) -> UserDataTypeDataPointBatchDeleteCall<'a, C> {
        self._parent = new_value.into();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> UserDataTypeDataPointBatchDeleteCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> UserDataTypeDataPointBatchDeleteCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::GooglehealthActivityAndFitnesWriteonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> UserDataTypeDataPointBatchDeleteCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> UserDataTypeDataPointBatchDeleteCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> UserDataTypeDataPointBatchDeleteCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Creates a single identifiable data point.
///
/// A builder for the *dataTypes.dataPoints.create* method supported by a *user* resource.
/// It is not used directly, but through a [`UserMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// use health4::api::DataPoint;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = DataPoint::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().data_types_data_points_create(req, "parent")
///              .doit().await;
/// # }
/// ```
pub struct UserDataTypeDataPointCreateCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _request: DataPoint,
    _parent: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for UserDataTypeDataPointCreateCall<'a, C> {}

impl<'a, C> UserDataTypeDataPointCreateCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Operation)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.users.dataTypes.dataPoints.create",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("parent", self._parent);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+parent}/dataPoints";
        if self._scopes.is_empty() {
            self._scopes.insert(
                Scope::GooglehealthActivityAndFitnesWriteonly
                    .as_ref()
                    .to_string(),
            );
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["parent"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: DataPoint) -> UserDataTypeDataPointCreateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The parent resource name where the data point will be created. Format: `users/{user}/dataTypes/{data_type}`
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(
        mut self,
        new_value: impl Into<String>,
    ) -> UserDataTypeDataPointCreateCall<'a, C> {
        self._parent = new_value.into();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> UserDataTypeDataPointCreateCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> UserDataTypeDataPointCreateCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::GooglehealthActivityAndFitnesWriteonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> UserDataTypeDataPointCreateCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> UserDataTypeDataPointCreateCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> UserDataTypeDataPointCreateCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Roll up data points over civil time intervals for supported data types.
///
/// A builder for the *dataTypes.dataPoints.dailyRollUp* method supported by a *user* resource.
/// It is not used directly, but through a [`UserMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// use health4::api::DailyRollUpDataPointsRequest;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = DailyRollUpDataPointsRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().data_types_data_points_daily_roll_up(req, "parent")
///              .doit().await;
/// # }
/// ```
pub struct UserDataTypeDataPointDailyRollUpCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _request: DailyRollUpDataPointsRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for UserDataTypeDataPointDailyRollUpCall<'a, C> {}

impl<'a, C> UserDataTypeDataPointDailyRollUpCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(
        mut self,
    ) -> common::Result<(common::Response, DailyRollUpDataPointsResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.users.dataTypes.dataPoints.dailyRollUp",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("parent", self._parent);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+parent}/dataPoints:dailyRollUp";
        if self._scopes.is_empty() {
            self._scopes.insert(
                Scope::GooglehealthActivityAndFitnesReadonly
                    .as_ref()
                    .to_string(),
            );
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["parent"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(
        mut self,
        new_value: DailyRollUpDataPointsRequest,
    ) -> UserDataTypeDataPointDailyRollUpCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. Parent data type of the Data Point collection. Format: `users/{user}/dataTypes/{data_type}`, e.g.: - `users/me/dataTypes/steps` - `users/me/dataTypes/distance` For a list of the supported data types see the DailyRollupDataPoint value union field.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(
        mut self,
        new_value: impl Into<String>,
    ) -> UserDataTypeDataPointDailyRollUpCall<'a, C> {
        self._parent = new_value.into();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> UserDataTypeDataPointDailyRollUpCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> UserDataTypeDataPointDailyRollUpCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::GooglehealthActivityAndFitnesReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> UserDataTypeDataPointDailyRollUpCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> UserDataTypeDataPointDailyRollUpCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> UserDataTypeDataPointDailyRollUpCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Exports exercise data in TCX format. **IMPORTANT:** HTTP clients must append `?alt=media` to the request URL to download the raw TCX file. Example: `https://health.googleapis.com/v4/users/me/dataTypes/exercise/dataPoints/EXERCISE_ID:exportExerciseTcx?alt=media` Without `alt=media`, the server returns a JSON response (`ExportExerciseTcxResponse`) which is intended primarily for gRPC clients. **Note:** While the Authorization section below states that any one of the listed scopes is accepted, this specific method requires the user to provide both one of the `activity_and_fitness` scopes (`normal` or `readonly`) AND one of the `location` scopes (`normal` or `readonly`) in their access token to succeed.
///
/// This method supports **media download**. To enable it, adjust the builder like this:
/// `.param("alt", "media")`.
/// Please note that due to missing multi-part support on the server side, you will only receive the media,
/// but not the `ExportExerciseTcxResponse` structure that you would usually get. The latter will be a default value.
///
/// A builder for the *dataTypes.dataPoints.exportExerciseTcx* method supported by a *user* resource.
/// It is not used directly, but through a [`UserMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().data_types_data_points_export_exercise_tcx("name")
///              .partial_data(true)
///              .doit().await;
/// # }
/// ```
pub struct UserDataTypeDataPointExportExerciseTcxCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _name: String,
    _partial_data: Option<bool>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _range: Option<String>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for UserDataTypeDataPointExportExerciseTcxCall<'a, C> {}

impl<'a, C> UserDataTypeDataPointExportExerciseTcxCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, ExportExerciseTcxResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.users.dataTypes.dataPoints.exportExerciseTcx",
            http_method: hyper::Method::GET,
        });

        for &field in ["name", "partialData"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("name", self._name);
        if let Some(value) = self._partial_data.as_ref() {
            params.push("partialData", value.to_string());
        }

        params.extend(self._additional_params.iter());

        let (alt_field_missing, enable_resource_parsing) = {
            if let Some(value) = params.get("alt") {
                (false, value == "json")
            } else {
                (true, true)
            }
        };
        if alt_field_missing {
            params.push("alt", "json");
        }
        let mut url = self.hub._base_url.clone() + "v4/{+name}:exportExerciseTcx";
        if self._scopes.is_empty() {
            self._scopes.insert(
                Scope::GooglehealthActivityAndFitnesReadonly
                    .as_ref()
                    .to_string(),
            );
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                if let Some(range_value) = self._range.as_ref() {
                    req_builder = req_builder.header("Range", range_value.clone());
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = if enable_resource_parsing {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    } else {
                        (
                            common::Response::from_parts(parts, body),
                            Default::default(),
                        )
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// Required. The resource name of the exercise data point to export. Format: `users/{user}/dataTypes/exercise/dataPoints/{data_point}` Example: `users/me/dataTypes/exercise/dataPoints/2026443605080188808` The `{user}` is the alias `"me"` currently. Future versions may support user IDs. The `{data_point}` ID maps to the exercise ID, which is a long integer.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(
        mut self,
        new_value: impl Into<String>,
    ) -> UserDataTypeDataPointExportExerciseTcxCall<'a, C> {
        self._name = new_value.into();
        self
    }
    /// Optional. Indicates whether to include the TCX data points when the GPS data is not available. If not specified, defaults to `false` and partial data will not be included.
    ///
    /// Sets the *partial data* query property to the given value.
    pub fn partial_data(
        mut self,
        new_value: bool,
    ) -> UserDataTypeDataPointExportExerciseTcxCall<'a, C> {
        self._partial_data = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> UserDataTypeDataPointExportExerciseTcxCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(
        mut self,
        name: T,
        value: T,
    ) -> UserDataTypeDataPointExportExerciseTcxCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::GooglehealthActivityAndFitnesReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> UserDataTypeDataPointExportExerciseTcxCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(
        mut self,
        scopes: I,
    ) -> UserDataTypeDataPointExportExerciseTcxCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> UserDataTypeDataPointExportExerciseTcxCall<'a, C> {
        self._scopes.clear();
        self
    }

    /// Sets the *Range* header for partial downloads.
    ///
    /// Use this to download only a portion of the file by specifying a byte range.
    /// For example: "bytes=0-1023" downloads the first 1024 bytes.
    ///
    /// This is only effective when using `alt=media` parameter.
    pub fn range(
        mut self,
        value: impl Into<String>,
    ) -> UserDataTypeDataPointExportExerciseTcxCall<'a, C> {
        self._range = Some(value.into());
        self
    }
}

/// Get a single identifyable data point.
///
/// A builder for the *dataTypes.dataPoints.get* method supported by a *user* resource.
/// It is not used directly, but through a [`UserMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().data_types_data_points_get("name")
///              .doit().await;
/// # }
/// ```
pub struct UserDataTypeDataPointGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _name: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for UserDataTypeDataPointGetCall<'a, C> {}

impl<'a, C> UserDataTypeDataPointGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, DataPoint)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.users.dataTypes.dataPoints.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("name", self._name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+name}";
        if self._scopes.is_empty() {
            self._scopes.insert(
                Scope::GooglehealthActivityAndFitnesReadonly
                    .as_ref()
                    .to_string(),
            );
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// Required. The name of the data point to retrieve. Format: `users/{user}/dataTypes/{data_type}/dataPoints/{data_point}` See DataPoint.name for examples and possible values.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: impl Into<String>) -> UserDataTypeDataPointGetCall<'a, C> {
        self._name = new_value.into();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> UserDataTypeDataPointGetCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> UserDataTypeDataPointGetCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::GooglehealthActivityAndFitnesReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> UserDataTypeDataPointGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> UserDataTypeDataPointGetCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> UserDataTypeDataPointGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Query user health and fitness data points.
///
/// A builder for the *dataTypes.dataPoints.list* method supported by a *user* resource.
/// It is not used directly, but through a [`UserMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().data_types_data_points_list("parent")
///              .page_token("est")
///              .page_size(-62)
///              .filter("ea")
///              .doit().await;
/// # }
/// ```
pub struct UserDataTypeDataPointListCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _parent: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _filter: Option<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for UserDataTypeDataPointListCall<'a, C> {}

impl<'a, C> UserDataTypeDataPointListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, ListDataPointsResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.users.dataTypes.dataPoints.list",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "parent", "pageToken", "pageSize", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("parent", self._parent);
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._filter.as_ref() {
            params.push("filter", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+parent}/dataPoints";
        if self._scopes.is_empty() {
            self._scopes.insert(
                Scope::GooglehealthActivityAndFitnesReadonly
                    .as_ref()
                    .to_string(),
            );
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["parent"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// Required. Parent data type of the Data Point collection. Format: `users/me/dataTypes/{data_type}`, e.g.: - `users/me/dataTypes/steps` - `users/me/dataTypes/weight` For a list of the supported data types see the DataPoint data union field.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: impl Into<String>) -> UserDataTypeDataPointListCall<'a, C> {
        self._parent = new_value.into();
        self
    }
    /// Optional. The `next_page_token` from a previous request, if any.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(
        mut self,
        new_value: impl Into<String>,
    ) -> UserDataTypeDataPointListCall<'a, C> {
        self._page_token = Some(new_value.into());
        self
    }
    /// Optional. The maximum number of data points to return. If unspecified, at most 1440 data points will be returned. The maximum page size is 10000; values above that will be truncated accordingly. For `exercise` and `sleep` the default page size is 25. The maximum page size for `exercise` and `sleep` is 25.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> UserDataTypeDataPointListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// Optional. Filter expression following https://google.aip.dev/160. A time range (either physical or civil) can be specified. The supported filter fields are: - Interval start time: - Pattern: `{interval_data_type}.interval.start_time` - Supported comparison operators: `>=`, `<` - Timestamp literal expected in RFC-3339 format - Supported logical operators: `AND` - Example: - `steps.interval.start_time >= "2023-11-24T00:00:00Z" AND steps.interval.start_time < "2023-11-25T00:00:00Z"` - `distance.interval.start_time >= "2024-08-14T12:34:56Z"` - Interval civil start time: - Pattern: `{interval_data_type}.interval.civil_start_time` - Supported comparison operators: `>=`, `<` - Date with optional time literal expected in ISO 8601 `YYYY-MM-DD[THH:mm:ss]` format - Supported logical operators: `AND` - Example: - `steps.interval.civil_start_time >= "2023-11-24" AND steps.interval.civil_start_time < "2023-11-25"` - `distance.interval.civil_start_time >= "2024-08-14T12:34:56"` - Sample observation physical time: - Pattern: `{sample_data_type}.sample_time.physical_time` - Supported comparison operators: `>=`, `<` - Timestamp literal expected in RFC-3339 format - Supported logical operators: `AND` - Example: - `weight.sample_time.physical_time >= "2023-11-24T00:00:00Z" AND weight.sample_time.physical_time < "2023-11-25T00:00:00Z"` - `weight.sample_time.physical_time >= "2024-08-14T12:34:56Z"` - Sample observation civil time: - Pattern: `{sample_data_type}.sample_time.civil_time` - Supported comparison operators: `>=`, `<` - Date with optional time literal expected in ISO 8601 `YYYY-MM-DD[THH:mm:ss]` format - Supported logical operators: `AND` - Example: - `weight.sample_time.civil_time >= "2023-11-24" AND weight.sample_time.civil_time < "2023-11-25"` - `weight.sample_time.civil_time >= "2024-08-14T12:34:56"` - Daily summary date: - Pattern: `{daily_summary_data_type}.date` - Supported comparison operators: `>=`, `<` - Date literal expected in ISO 8601 `YYYY-MM-DD` format - Supported logical operators: `AND` - Example: - `daily_heart_rate_variability.date < "2024-08-15"` - Session civil start time (**Excluding Sleep and ECG**): - Pattern: `{session_data_type}.interval.civil_start_time` - Supported comparison operators: `>=`, `<` - Date with optional time literal expected in ISO 8601 `YYYY-MM-DD[THH:mm:ss]` format - Supported logical operators: `AND` - Example: - `exercise.interval.civil_start_time >= "2023-11-24" AND exercise.interval.civil_start_time < "2023-11-25"` - `exercise.interval.civil_start_time >= "2024-08-14T12:34:56"` - Session start time (**ECG specific**): - Pattern: `electrocardiogram.interval.start_time` - Supported comparison operators: `>=` - Timestamp literal expected in RFC-3339 format - Example: - `electrocardiogram.interval.start_time >= "2024-08-14T12:34:56Z"` - Note: Only filtering by start time is supported for ECG. Filtering by end time (e.g., `electrocardiogram.interval.end_time`) is not supported. - Session end time (**Sleep specific**): - Pattern: `sleep.interval.end_time` - Supported comparison operators: `>=`, `<` - Timestamp literal expected in RFC-3339 format - Supported logical operators: `AND`, `OR` - Example: - `sleep.interval.end_time >= "2023-11-24T00:00:00Z" AND sleep.interval.end_time < "2023-11-25T00:00:00Z"` - Session civil end time (**Sleep specific**): - Pattern: `sleep.interval.civil_end_time` - Supported comparison operators: `>=`, `<` - Date with optional time literal expected in ISO 8601 `YYYY-MM-DD[THH:mm:ss]` format - Supported logical operators: `AND`, `OR` - Example: - `sleep.interval.civil_end_time >= "2023-11-24" AND sleep.interval.civil_end_time < "2023-11-25"` Data points in the response will be ordered by the interval start time in descending order.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: impl Into<String>) -> UserDataTypeDataPointListCall<'a, C> {
        self._filter = Some(new_value.into());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> UserDataTypeDataPointListCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> UserDataTypeDataPointListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::GooglehealthActivityAndFitnesReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> UserDataTypeDataPointListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> UserDataTypeDataPointListCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> UserDataTypeDataPointListCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Updates a single identifiable data point. If a data point with the specified `name` is not found, the request will fail.
///
/// A builder for the *dataTypes.dataPoints.patch* method supported by a *user* resource.
/// It is not used directly, but through a [`UserMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// use health4::api::DataPoint;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = DataPoint::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().data_types_data_points_patch(req, "name")
///              .doit().await;
/// # }
/// ```
pub struct UserDataTypeDataPointPatchCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _request: DataPoint,
    _name: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for UserDataTypeDataPointPatchCall<'a, C> {}

impl<'a, C> UserDataTypeDataPointPatchCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Operation)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.users.dataTypes.dataPoints.patch",
            http_method: hyper::Method::PATCH,
        });

        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("name", self._name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+name}";
        if self._scopes.is_empty() {
            self._scopes.insert(
                Scope::GooglehealthActivityAndFitnesWriteonly
                    .as_ref()
                    .to_string(),
            );
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PATCH)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: DataPoint) -> UserDataTypeDataPointPatchCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Identifier. Data point name, only supported for the subset of identifiable data types. For the majority of the data types, individual data points do not need to be identified and this field would be empty. Format: `users/{user}/dataTypes/{data_type}/dataPoints/{data_point}` Example: `users/abcd1234/dataTypes/sleep/dataPoints/a1b2c3d4-e5f6-7890-1234-567890abcdef` The `{user}` ID is a system-generated identifier, as described in Identity.health_user_id. The `{data_type}` ID corresponds to the kebab-case version of the field names in the DataPoint data union field, e.g. `heart-rate` for the `heart_rate` field. The `{data_point}` ID can be client-provided or system-generated. If client-provided, it must be a string of 4-63 characters, containing only lowercase letters, numbers, and hyphens.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: impl Into<String>) -> UserDataTypeDataPointPatchCall<'a, C> {
        self._name = new_value.into();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> UserDataTypeDataPointPatchCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> UserDataTypeDataPointPatchCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::GooglehealthActivityAndFitnesWriteonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> UserDataTypeDataPointPatchCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> UserDataTypeDataPointPatchCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> UserDataTypeDataPointPatchCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Reconcile data points from multiple data sources into a single data stream.
///
/// A builder for the *dataTypes.dataPoints.reconcile* method supported by a *user* resource.
/// It is not used directly, but through a [`UserMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().data_types_data_points_reconcile("parent")
///              .page_token("eos")
///              .page_size(-86)
///              .filter("sed")
///              .data_source_family("duo")
///              .doit().await;
/// # }
/// ```
pub struct UserDataTypeDataPointReconcileCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _parent: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _filter: Option<String>,
    _data_source_family: Option<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for UserDataTypeDataPointReconcileCall<'a, C> {}

impl<'a, C> UserDataTypeDataPointReconcileCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, ReconcileDataPointsResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.users.dataTypes.dataPoints.reconcile",
            http_method: hyper::Method::GET,
        });

        for &field in [
            "alt",
            "parent",
            "pageToken",
            "pageSize",
            "filter",
            "dataSourceFamily",
        ]
        .iter()
        {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("parent", self._parent);
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._filter.as_ref() {
            params.push("filter", value);
        }
        if let Some(value) = self._data_source_family.as_ref() {
            params.push("dataSourceFamily", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+parent}/dataPoints:reconcile";
        if self._scopes.is_empty() {
            self._scopes.insert(
                Scope::GooglehealthActivityAndFitnesReadonly
                    .as_ref()
                    .to_string(),
            );
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["parent"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// Required. Parent data type of the Data Point collection. Format: `users/me/dataTypes/{data_type}`, e.g.: - `users/me/dataTypes/steps` - `users/me/dataTypes/heart-rate` For a list of the supported data types see the DataPoint data union field.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(
        mut self,
        new_value: impl Into<String>,
    ) -> UserDataTypeDataPointReconcileCall<'a, C> {
        self._parent = new_value.into();
        self
    }
    /// Optional. The `next_page_token` from a previous request, if any.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(
        mut self,
        new_value: impl Into<String>,
    ) -> UserDataTypeDataPointReconcileCall<'a, C> {
        self._page_token = Some(new_value.into());
        self
    }
    /// Optional. The maximum number of data points to return. If unspecified, at most 1440 data points will be returned. The maximum page size is 10000; values above that will be truncated accordingly. For `exercise` and `sleep` the default page size is 25. The maximum page size for `exercise` and `sleep` is 25.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> UserDataTypeDataPointReconcileCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// Optional. Filter expression based on https://aip.dev/160. A time range, either physical or civil, can be specified. See the ListDataPointsRequest.filter for the supported fields and syntax.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(
        mut self,
        new_value: impl Into<String>,
    ) -> UserDataTypeDataPointReconcileCall<'a, C> {
        self._filter = Some(new_value.into());
        self
    }
    /// Optional. The data source family name to reconcile. If empty, data points from all data sources will be reconciled. Format: `users/me/dataSourceFamilies/{data_source_family}` The supported values are: - `users/me/dataSourceFamilies/all-sources` - default value - `users/me/dataSourceFamilies/google-wearables` - tracker devices - `users/me/dataSourceFamilies/google-sources` - Google first party sources
    ///
    /// Sets the *data source family* query property to the given value.
    pub fn data_source_family(
        mut self,
        new_value: impl Into<String>,
    ) -> UserDataTypeDataPointReconcileCall<'a, C> {
        self._data_source_family = Some(new_value.into());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> UserDataTypeDataPointReconcileCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> UserDataTypeDataPointReconcileCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::GooglehealthActivityAndFitnesReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> UserDataTypeDataPointReconcileCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> UserDataTypeDataPointReconcileCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> UserDataTypeDataPointReconcileCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Roll up data points over physical time intervals for supported data types.
///
/// A builder for the *dataTypes.dataPoints.rollUp* method supported by a *user* resource.
/// It is not used directly, but through a [`UserMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// use health4::api::RollUpDataPointsRequest;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = RollUpDataPointsRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().data_types_data_points_roll_up(req, "parent")
///              .doit().await;
/// # }
/// ```
pub struct UserDataTypeDataPointRollUpCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _request: RollUpDataPointsRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for UserDataTypeDataPointRollUpCall<'a, C> {}

impl<'a, C> UserDataTypeDataPointRollUpCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, RollUpDataPointsResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.users.dataTypes.dataPoints.rollUp",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("parent", self._parent);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+parent}/dataPoints:rollUp";
        if self._scopes.is_empty() {
            self._scopes.insert(
                Scope::GooglehealthActivityAndFitnesReadonly
                    .as_ref()
                    .to_string(),
            );
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["parent"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(
        mut self,
        new_value: RollUpDataPointsRequest,
    ) -> UserDataTypeDataPointRollUpCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. Parent data type of the Data Point collection. Format: `users/{user}/dataTypes/{data_type}`, e.g.: - `users/me/dataTypes/steps` - `users/me/dataTypes/distance` For a list of the supported data types see the RollupDataPoint value union field.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(
        mut self,
        new_value: impl Into<String>,
    ) -> UserDataTypeDataPointRollUpCall<'a, C> {
        self._parent = new_value.into();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> UserDataTypeDataPointRollUpCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> UserDataTypeDataPointRollUpCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::GooglehealthActivityAndFitnesReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> UserDataTypeDataPointRollUpCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> UserDataTypeDataPointRollUpCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> UserDataTypeDataPointRollUpCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Returns user's Device.
///
/// A builder for the *pairedDevices.get* method supported by a *user* resource.
/// It is not used directly, but through a [`UserMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().paired_devices_get("name")
///              .doit().await;
/// # }
/// ```
pub struct UserPairedDeviceGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _name: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for UserPairedDeviceGetCall<'a, C> {}

impl<'a, C> UserPairedDeviceGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, PairedDevice)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.users.pairedDevices.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("name", self._name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+name}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::GooglehealthSettingReadonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// Required. The name of the device to retrieve. Format: users/{user}/devices/{device}
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: impl Into<String>) -> UserPairedDeviceGetCall<'a, C> {
        self._name = new_value.into();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> UserPairedDeviceGetCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> UserPairedDeviceGetCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::GooglehealthSettingReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> UserPairedDeviceGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> UserPairedDeviceGetCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> UserPairedDeviceGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Returns the user's list of paired 1P trackers and smartwatches.
///
/// A builder for the *pairedDevices.list* method supported by a *user* resource.
/// It is not used directly, but through a [`UserMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().paired_devices_list("parent")
///              .page_token("kasd")
///              .page_size(-24)
///              .doit().await;
/// # }
/// ```
pub struct UserPairedDeviceListCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _parent: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for UserPairedDeviceListCall<'a, C> {}

impl<'a, C> UserPairedDeviceListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, ListPairedDevicesResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.users.pairedDevices.list",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "parent", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("parent", self._parent);
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+parent}/pairedDevices";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::GooglehealthSettingReadonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["parent"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// Required. The parent, which owns this collection of devices. Format: users/{user}
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: impl Into<String>) -> UserPairedDeviceListCall<'a, C> {
        self._parent = new_value.into();
        self
    }
    /// Optional. A page token, received from a previous `ListPairedDevices` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListPairedDevices` must match the call that provided the page token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: impl Into<String>) -> UserPairedDeviceListCall<'a, C> {
        self._page_token = Some(new_value.into());
        self
    }
    /// Optional. The maximum number of devices to return. The service may return fewer than this value. If unspecified, at most 5 devices will be returned. The maximum value is 100. values above 100 will be coerced to 100.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> UserPairedDeviceListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> UserPairedDeviceListCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> UserPairedDeviceListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::GooglehealthSettingReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> UserPairedDeviceListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> UserPairedDeviceListCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> UserPairedDeviceListCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Gets the user's identity. It includes the legacy Fitbit user ID and the Google user ID and it can be used by migrating clients to map identifiers between the two systems.
///
/// A builder for the *getIdentity* method supported by a *user* resource.
/// It is not used directly, but through a [`UserMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().get_identity("name")
///              .doit().await;
/// # }
/// ```
pub struct UserGetIdentityCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _name: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for UserGetIdentityCall<'a, C> {}

impl<'a, C> UserGetIdentityCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Identity)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.users.getIdentity",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("name", self._name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+name}";
        if self._scopes.is_empty() {
            self._scopes.insert(
                Scope::GooglehealthActivityAndFitnesReadonly
                    .as_ref()
                    .to_string(),
            );
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// Required. The resource name of the Identity. Format: `users/me/identity`
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: impl Into<String>) -> UserGetIdentityCall<'a, C> {
        self._name = new_value.into();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> UserGetIdentityCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> UserGetIdentityCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::GooglehealthActivityAndFitnesReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> UserGetIdentityCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> UserGetIdentityCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> UserGetIdentityCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Returns user's IRN Profile details.
///
/// A builder for the *getIrnProfile* method supported by a *user* resource.
/// It is not used directly, but through a [`UserMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().get_irn_profile("name")
///              .doit().await;
/// # }
/// ```
pub struct UserGetIrnProfileCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _name: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for UserGetIrnProfileCall<'a, C> {}

impl<'a, C> UserGetIrnProfileCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, IrnProfile)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.users.getIrnProfile",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("name", self._name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+name}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::GooglehealthIrnReadonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// Required. The resource name of the IRN Profile. Format: `users/{user}/irnProfile` Example: `users/1234567890/irnProfile` or `users/me/irnProfile` The {user} ID is a system-generated Google Health API user ID, a string of 1-63 characters consisting of lowercase and uppercase letters, numbers, and hyphens. The literal `me` can also be used to refer to the authenticated user.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: impl Into<String>) -> UserGetIrnProfileCall<'a, C> {
        self._name = new_value.into();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> UserGetIrnProfileCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> UserGetIrnProfileCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::GooglehealthIrnReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> UserGetIrnProfileCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> UserGetIrnProfileCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> UserGetIrnProfileCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Returns user Profile details.
///
/// A builder for the *getProfile* method supported by a *user* resource.
/// It is not used directly, but through a [`UserMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().get_profile("name")
///              .doit().await;
/// # }
/// ```
pub struct UserGetProfileCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _name: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for UserGetProfileCall<'a, C> {}

impl<'a, C> UserGetProfileCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Profile)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.users.getProfile",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("name", self._name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+name}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::GooglehealthProfileReadonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// Required. The name of the Profile. Format: `users/me/profile`.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: impl Into<String>) -> UserGetProfileCall<'a, C> {
        self._name = new_value.into();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> UserGetProfileCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> UserGetProfileCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::GooglehealthProfileReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> UserGetProfileCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> UserGetProfileCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> UserGetProfileCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Returns user settings details.
///
/// A builder for the *getSettings* method supported by a *user* resource.
/// It is not used directly, but through a [`UserMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().get_settings("name")
///              .doit().await;
/// # }
/// ```
pub struct UserGetSettingCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _name: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for UserGetSettingCall<'a, C> {}

impl<'a, C> UserGetSettingCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Settings)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.users.getSettings",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("name", self._name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+name}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::GooglehealthSettingReadonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// Required. The name of the Settings. Format: `users/me/settings`.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: impl Into<String>) -> UserGetSettingCall<'a, C> {
        self._name = new_value.into();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> UserGetSettingCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> UserGetSettingCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::GooglehealthSettingReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> UserGetSettingCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> UserGetSettingCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> UserGetSettingCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Updates the user's profile details.
///
/// A builder for the *updateProfile* method supported by a *user* resource.
/// It is not used directly, but through a [`UserMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// use health4::api::Profile;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Profile::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().update_profile(req, "name")
///              .update_mask(FieldMask::new::<&str>(&[]))
///              .doit().await;
/// # }
/// ```
pub struct UserUpdateProfileCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _request: Profile,
    _name: String,
    _update_mask: Option<common::FieldMask>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for UserUpdateProfileCall<'a, C> {}

impl<'a, C> UserUpdateProfileCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Profile)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.users.updateProfile",
            http_method: hyper::Method::PATCH,
        });

        for &field in ["alt", "name", "updateMask"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("name", self._name);
        if let Some(value) = self._update_mask.as_ref() {
            params.push("updateMask", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+name}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::GooglehealthProfileWriteonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PATCH)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Profile) -> UserUpdateProfileCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Identifier. The resource name of this Profile resource. Format: `users/{user}/profile` Example: `users/1234567890/profile` or `users/me/profile` The {user} ID is a system-generated Google Health API user ID, a string of 1-63 characters consisting of lowercase and uppercase letters, numbers, and hyphens. The literal `me` can also be used to refer to the authenticated user.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: impl Into<String>) -> UserUpdateProfileCall<'a, C> {
        self._name = new_value.into();
        self
    }
    /// Optional. The list of fields to be updated.
    ///
    /// Sets the *update mask* query property to the given value.
    pub fn update_mask(mut self, new_value: common::FieldMask) -> UserUpdateProfileCall<'a, C> {
        self._update_mask = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> UserUpdateProfileCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> UserUpdateProfileCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::GooglehealthProfileWriteonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> UserUpdateProfileCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> UserUpdateProfileCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> UserUpdateProfileCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Updates the user's settings details.
///
/// A builder for the *updateSettings* method supported by a *user* resource.
/// It is not used directly, but through a [`UserMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_health4 as health4;
/// use health4::api::Settings;
/// # async fn dox() {
/// # use health4::{GoogleHealthAPI, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = GoogleHealthAPI::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Settings::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().update_settings(req, "name")
///              .update_mask(FieldMask::new::<&str>(&[]))
///              .doit().await;
/// # }
/// ```
pub struct UserUpdateSettingCall<'a, C>
where
    C: 'a,
{
    hub: &'a GoogleHealthAPI<C>,
    _request: Settings,
    _name: String,
    _update_mask: Option<common::FieldMask>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for UserUpdateSettingCall<'a, C> {}

impl<'a, C> UserUpdateSettingCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Settings)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "health.users.updateSettings",
            http_method: hyper::Method::PATCH,
        });

        for &field in ["alt", "name", "updateMask"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("name", self._name);
        if let Some(value) = self._update_mask.as_ref() {
            params.push("updateMask", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/{+name}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::GooglehealthSettingWriteonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PATCH)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Settings) -> UserUpdateSettingCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Identifier. The resource name of this Settings resource. Format: `users/{user}/settings` Example: `users/1234567890/settings` or `users/me/settings` The {user} ID is a system-generated Google Health API user ID, a string of 1-63 characters consisting of lowercase and uppercase letters, numbers, and hyphens. The literal `me` can also be used to refer to the authenticated user.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: impl Into<String>) -> UserUpdateSettingCall<'a, C> {
        self._name = new_value.into();
        self
    }
    /// Optional. The list of fields to be updated.
    ///
    /// Sets the *update mask* query property to the given value.
    pub fn update_mask(mut self, new_value: common::FieldMask) -> UserUpdateSettingCall<'a, C> {
        self._update_mask = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> UserUpdateSettingCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> UserUpdateSettingCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::GooglehealthSettingWriteonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> UserUpdateSettingCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> UserUpdateSettingCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> UserUpdateSettingCall<'a, C> {
        self._scopes.clear();
        self
    }
}
