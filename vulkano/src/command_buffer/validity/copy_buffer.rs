// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use std::cmp;
use std::error;
use std::fmt;

use VulkanObject;
use buffer::BufferAccess;
use device::Device;
use device::DeviceOwned;

/// Checks whether a copy buffer command is valid.
///
/// # Panic
///
/// - Panics if the source and destination were not created with `device`.
///
// FIXME: type safety
pub fn check_copy_buffer<S, D>(device: &Device, source: &S, destination: &D)
                               -> Result<(), CheckCopyBufferError>
    where S: ?Sized + BufferAccess,
          D: ?Sized + BufferAccess
{
    assert_eq!(source.inner().buffer.device().internal_object(),
               device.internal_object());
    assert_eq!(destination.inner().buffer.device().internal_object(),
               device.internal_object());

    if !source.inner().buffer.usage_transfer_src() {
        return Err(CheckCopyBufferError::SourceMissingTransferUsage);
    }

    if !destination.inner().buffer.usage_transfer_dest() {
        return Err(CheckCopyBufferError::DestinationMissingTransferUsage);
    }

    let size = cmp::min(source.size(), destination.size());

    if source.conflicts_buffer(0, size, &destination, 0, size) {
        return Err(CheckCopyBufferError::OverlappingRanges);
    } else {
        debug_assert!(!destination.conflicts_buffer(0, size, &source, 0, size));
    }

    Ok(())
}

/// Error that can happen from `check_copy_buffer`.
#[derive(Debug, Copy, Clone)]
pub enum CheckCopyBufferError {
    /// The source buffer is missing the transfer source usage.
    SourceMissingTransferUsage,
    /// The destination buffer is missing the transfer destination usage.
    DestinationMissingTransferUsage,
    /// The source and destination are overlapping.
    OverlappingRanges,
}

impl error::Error for CheckCopyBufferError {
    #[inline]
    fn description(&self) -> &str {
        match *self {
            CheckCopyBufferError::SourceMissingTransferUsage => {
                "the source buffer is missing the transfer source usage"
            },
            CheckCopyBufferError::DestinationMissingTransferUsage => {
                "the destination buffer is missing the transfer destination usage"
            },
            CheckCopyBufferError::OverlappingRanges => {
                "the source and destination are overlapping"
            },
        }
    }
}

impl fmt::Display for CheckCopyBufferError {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", error::Error::description(self))
    }
}
