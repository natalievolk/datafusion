// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use datafusion_common::instant::Instant;
use datafusion_physical_plan::metrics::{
    Count, ExecutionPlanMetricsSet, MetricBuilder, MetricsSet, Time,
};

/// Shared metrics for file sink write operations.
///
/// Holds `rows_written`, `bytes_written`, and `elapsed_compute` metrics
/// pre-wired into an [`ExecutionPlanMetricsSet`]. All file sinks
/// (`ParquetSink`, `CsvSink`, `JsonSink`, etc.) use this struct to avoid
/// duplicating metric setup boilerplate.
pub struct FileSinkMetrics {
    metrics: ExecutionPlanMetricsSet,
    rows_written: Count,
    bytes_written: Count,
    elapsed_compute: Time,
}

impl FileSinkMetrics {
    /// Create a new [`FileSinkMetrics`] with pre-wired counters.
    pub fn new() -> Self {
        let metrics = ExecutionPlanMetricsSet::new();
        let rows_written = MetricBuilder::new(&metrics).global_counter("rows_written");
        let bytes_written = MetricBuilder::new(&metrics).global_counter("bytes_written");
        let elapsed_compute = MetricBuilder::new(&metrics).elapsed_compute(0);
        Self {
            metrics,
            rows_written,
            bytes_written,
            elapsed_compute,
        }
    }

    /// Counter for total rows written.
    pub fn rows_written(&self) -> &Count {
        &self.rows_written
    }

    /// Counter for total bytes written.
    pub fn bytes_written(&self) -> &Count {
        &self.bytes_written
    }

    /// Record elapsed wall-clock time since `start`.
    pub fn record_elapsed(&self, start: Instant) {
        self.elapsed_compute.add_elapsed(start);
    }

    /// Snapshot of the metrics set — return from `DataSink::metrics()`.
    pub fn metrics_set(&self) -> MetricsSet {
        self.metrics.clone_inner()
    }
}

impl Default for FileSinkMetrics {
    fn default() -> Self {
        Self::new()
    }
}
