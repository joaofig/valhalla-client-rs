use crate::route::ShapePoint;
use crate::{DateTime, costing};
use serde::Serialize;

#[derive(Serialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
/// Type of the directions
pub enum ShapeMatchType {
    /// indicating an edge walking algorithm can be used.
    #[serde(rename = "edge_walk")]
    EdgeWalk,

    /// indicating hat a map-matching algorithm should be used because the input shape might not
    /// closely match Valhalla edges.
    #[serde(rename = "map_snap")]
    MapSnap,

    /// indicating that maneuvers with instructions should be returned (this is the default if not
    /// specified).
    #[default]
    #[serde(rename = "walk_or_snap")]
    WalkOrSnap,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Default, Debug)]
/// Trace options
pub struct TraceOptions {
    search_radius: Option<f64>,
    gps_accuracy: Option<f64>,
    breakage_distance: Option<f64>,
    interpolation_distance: Option<f64>,
}

impl TraceOptions {
    #[must_use]
    /// Create a new [`crate::trace_route::TraceOptions`] builder
    pub fn builder() -> Self {
        Self::default()
    }

    /// ```rust
    /// Sets the search radius for the object.
    ///
    /// This method allows you to configure the search radius by providing a `f64` value.
    /// The specified radius will be stored internally and can be used for distance-based
    /// operations or calculations. Once the search radius is set, the method returns
    /// the modified object for further chaining.
    ///
    /// # Arguments
    ///
    /// * `search_radius` - A `f64` representing the radius value to be set.
    ///
    /// # Returns
    ///
    /// * `Self` - The modified instance of the object with the search radius configured.
    ///
    /// # Example
    ///
    /// ```rust
    /// let object = MyStruct::new()
    ///     .search_radius(10.0);
    /// ```
    /// ```
    pub fn search_radius(mut self, search_radius: f64) -> Self {
        self.search_radius = Some(search_radius);
        self
    }

    /// ```rust
    /// Sets the GPS accuracy for the current instance.
    ///
    /// This method allows configuring the GPS accuracy value, typically in meters.
    /// The provided value is wrapped in an `Option` and stored. The function
    /// follows a builder pattern, enabling method chaining for a fluent API.
    ///
    /// # Arguments
    /// * `gps_accuracy` - A `f64` representing the GPS accuracy in meters.
    ///
    /// # Returns
    /// Returns `Self`, allowing method calls to be chained.
    ///
    /// # Example
    /// ```
    /// let instance = MyStruct::default()
    ///     .gps_accuracy(5.0);
    /// ```
    /// ```
    pub fn gps_accuracy(mut self, gps_accuracy: f64) -> Self {
        self.gps_accuracy = Some(gps_accuracy);
        self
    }

    /// ```rust
    /// Sets the breakage distance for the current object.
    ///
    /// This method updates the `breakage_distance` property with the provided value
    /// and returns the updated object. The `breakage_distance` represents the
    /// threshold distance at which breakage occurs.
    ///
    /// # Arguments
    ///
    /// * `breakage_distance` - A `f64` value representing the desired breakage distance.
    ///
    /// # Returns
    ///
    /// Returns `Self`, the updated instance of the object with the new
    /// `breakage_distance` set.
    ///
    /// # Example
    ///
    /// ```rust
    /// let updated_object = object.breakage_distance(10.5);
    /// ```
    /// ```
    pub fn breakage_distance(mut self, breakage_distance: f64) -> Self {
        self.breakage_distance = Some(breakage_distance);
        self
    }

    /// ```rust
    /// Sets the interpolation distance for the object and returns the modified instance.
    ///
    /// # Parameters
    /// - `interpolation_distance` (`f64`): The interpolation distance to be set. This value
    ///   determines the spacing or interval for interpolation operations within the object.
    ///
    /// # Returns
    /// - `Self`: The modified instance of the object with the updated interpolation distance.
    ///
    /// # Example
    /// ```
    /// let instance = SomeStruct::new()
    ///     .interpolation_distance(10.5);
    /// ```
    ///
    /// In this example, the `interpolation_distance` method is used to set the interpolation distance
    /// to 10.5 for the instance of `SomeStruct`.
    /// ```
    pub fn interpolation_distance(mut self, interpolation_distance: f64) -> Self {
        self.interpolation_distance = Some(interpolation_distance);
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Default, Debug)]
/// Trace route request
pub struct Manifest {
    #[serde(flatten)]
    costing: Option<costing::Costing>,
    shape_match: Option<ShapeMatchType>,
    begin_time: Option<DateTime>,
    durations: Option<Vec<i64>>,
    use_timestamps: Option<bool>,
    trace_options: Option<TraceOptions>,
    linear_references: Option<bool>,
    shape: Option<Vec<ShapePoint>>,
    verbose: Option<bool>,
}

impl Manifest {
    #[must_use]
    /// Create a new [`crate::route::Manifest`] builder
    pub fn builder() -> Self {
        Self::default()
    }

    /// ```rust
    /// Sets the shape of the object using an iterator of `ShapePoint`.
    ///
    /// This method takes an iterable collection of `ShapePoint` items, converts it into an iterator,
    /// collects the items into a collection, and sets it as the shape of the object.
    ///
    /// # Parameters
    /// - `shape`: An iterable collection of items that implement the `ShapePoint` trait.
    ///   The shape defines the geometric or logical structure of the object.
    ///
    /// # Returns
    /// - `Self`: Returns the modified instance of the object, allowing for method chaining.
    ///
    /// # Example
    /// ```
    /// let points = vec![ShapePoint::new(0, 0), ShapePoint::new(1, 1)];
    /// let manifest = Manifest::builder().shape(points);
    /// ```
    /// ```
    pub fn shape(mut self, shape: impl IntoIterator<Item = ShapePoint>) -> Self {
        self.shape = Some(shape.into_iter().collect());
        self
    }

    /// ```rust
    /// Sets the durations for the object.
    ///
    /// This method accepts an iterable collection of `i64` values representing
    /// durations and assigns them to the `durations` field of the object. The provided
    /// durations are collected into a `Vec<i64>` and stored internally.
    ///
    /// # Parameters
    /// - `durations`: An iterable collection of `i64` values, such as a vector,
    ///   slice, or any other type that implements the `IntoIterator` trait.
    ///
    /// # Returns
    /// Returns the modified object with the durations field updated, allowing
    /// method chaining.
    ///
    /// # Example
    /// ```
    /// let obj = Manifest::builder()
    ///     .durations(vec![100, 200, 300]);
    /// ```
    ///
    /// In the example above, the `durations` method is called with a vector of `i64`
    /// values, updating the `durations` field of the `MyStruct` object.
    /// ```
    pub fn durations(mut self, durations: impl IntoIterator<Item = i64>) -> Self {
        self.durations = Some(durations.into_iter().collect());
        self
    }


    /// ```rust
    ///     /// Sets the shape matching type for the current instance.
    ///     ///
    ///     /// This method allows you to specify the type of shape matching to be used by assigning
    ///     /// a `ShapeMatchType` value to the `shape_match` field of the instance. The method consumes
    ///     /// the current instance (`self`), modifies the `shape_match` field, and returns the updated instance.
    ///     ///
    ///     /// # Parameters
    ///     /// - `shape_match`: The type of shape matching to be applied, represented as a `ShapeMatchType` enum.
    ///     ///
    ///     /// # Returns
    ///     /// - `Self`: An updated instance of the struct, with the `shape_match` field set to the provided value.
    ///     ///
    ///     /// # Example
    ///     /// ```
    ///     /// let instance = Manifest::builder()
    ///     ///     .shape_match(ShapeMatchType::Exact);
    ///     /// ```
    /// ```
    pub fn shape_match(mut self, shape_match: ShapeMatchType) -> Self {
        self.shape_match = Some(shape_match);
        self
    }

    /// ```rust
    /// Sets the `costing` field of the current object.
    ///
    /// This function takes a `Costing` object and assigns it to the `costing`
    /// field of the object. The function consumes the current object, updates
    /// its `costing` field, and then returns the updated object for method
    /// chaining.
    ///
    /// # Arguments
    ///
    /// * `costing` - A `costing::Costing` instance that represents the costing
    ///   information to be set.
    ///
    /// # Returns
    ///
    /// Returns the updated instance of `Self` with the `costing` field set.
    ///
    /// # Example
    ///
    /// ```
    /// use your_crate::costing::Costing;
    ///
    /// let costing_instance = Manifest::builder()
    ///     .costing(costing_instance);
    /// ```
    /// ```
    pub fn costing(mut self, costing: costing::Costing) -> Self {
        self.costing = Some(costing);
        self
    }

    /// ```rust
    /// Sets whether timestamps should be used.
    ///
    /// # Parameters
    /// - `use_timestamps` - A boolean indicating whether timestamps should be enabled (`true`)
    ///   or disabled (`false`).
    ///
    /// # Returns
    /// Returns the updated instance of `Self` with the `use_timestamps` option modified.
    ///
    /// # Example
    /// ```
    /// let manifest = Manifest::builder().use_timestamps(true);
    /// ```
    /// ```
    pub fn use_timestamps(mut self, use_timestamps: bool) -> Self {
        self.use_timestamps = Some(use_timestamps);
        self
    }

    /// ```rust
    /// Sets the verbosity level for the object.
    ///
    /// This method allows you to enable or disable verbose mode by passing
    /// a boolean value. When `true`, verbose mode is enabled; when `false`,
    /// it is disabled. The `verbose` setting is stored as an `Option<bool>`.
    ///
    /// # Parameters
    /// - `verbose`: A boolean value indicating whether verbose mode
    ///   should be enabled (`true`) or disabled (`false`).
    ///
    /// # Returns
    /// Returns the updated instance of `Self`, allowing method chaining.
    ///
    /// # Example
    /// ```
    /// let manifest = Manifest::builder()
    ///     .verbose(true);
    /// ```
    /// ```
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = Some(verbose);
        self
    }

    /// ```rust
    /// /**
    ///  * Sets the trace options for the current instance.
    ///  *
    ///  * This method allows you to configure tracing behavior by providing a `TraceOptions` object.
    ///  * The provided `trace_options` will be stored and used as part of the tracing configuration.
    ///  *
    ///  * # Parameters
    ///  * - `trace_options`: An instance of `TraceOptions` that specifies the desired tracing configuration.
    ///  *
    ///  * # Returns
    ///  * Returns `Self`, allowing method chaining for further configuration or usage.
    ///  *
    ///  * # Example
    ///  * ```
    ///  * let instance = Manifest::builder()
    ///  *     .trace_options(TraceOptions::builder());
    ///  * ```
    ///  */
    /// ```
    pub fn trace_options(mut self, trace_options: TraceOptions) -> Self {
        self.trace_options = Some(trace_options);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn serialisation() {
        assert_eq!(
            serde_json::to_value(Manifest::default()).unwrap(),
            serde_json::json!({"shape": []})
        );
    }
}
