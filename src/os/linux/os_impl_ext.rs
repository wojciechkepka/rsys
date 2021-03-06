use super::{cpu::*, kernel_version, mem::*, misc::*, net::*, ps::*, storage::*, Linux};
use crate::Result;

/// Trait extending Rsys functionality with linux specific api
pub trait OsImplExt {
    //
    // storage
    //

    /// Parses a StorageDevice object from system. If the provided name
    /// doesn't start with `sd` returns an error.
    fn stat_block_device(&self, name: &str) -> Result<StorageDevice>;

    /// Parses a DeviceMapper object from system. If the provided name
    /// doesn't start with `dm` returns an error.
    fn stat_device_mapper(&self, name: &str) -> Result<DeviceMapper>;

    /// Parses a ScsiCdrom object from system. If the provided name
    /// doesn't start with `sr` returns an error.
    fn stat_scsi_cdrom(&self, name: &str) -> Result<ScsiCdrom>;

    /// Parses a MultipleDeviceStorage object from system. If the provided name
    /// doesn't start with `md` returns an error.
    fn stat_multiple_device_storage(&self, name: &str) -> Result<MultipleDeviceStorage>;

    /// Returns block size of device in bytes
    fn block_size(&self, name: &str) -> Result<i64>;

    //
    // memory
    //

    /// Returns detailed information about memory
    fn memory(&self) -> Result<Memory>;

    //
    // ps
    //

    /// Returns detailed Process information parsed from /proc/[pid]/stat
    fn stat_process(&self, pid: i32) -> Result<ProcessStat>;

    /// Returns a list of pids read from /proc
    fn pids(&self) -> Result<Vec<i32>>;

    /// Returns all processes currently seen in /proc parsed as Processes
    fn processes(&self) -> Result<Processes>;

    //
    // other
    //

    /// Returns kernel version of host os.
    fn kernel_version(&self) -> Result<String>;

    /// Returns MountPoints read from /proc/mounts
    fn mounts(&self) -> Result<MountPoints>;

    //
    // cpu
    //

    /// Returns virtual Cores of host cpu
    fn cores(&self) -> Result<Cores>;

    /// Returns a Processor object containing gathered information about host cpu
    fn processor(&self) -> Result<Processor>;

    //
    // net
    //

    /// Returns network interfaces on host os
    fn ifaces(&self) -> Result<Interfaces>;
}

impl OsImplExt for Linux {
    //
    // storage
    //

    fn stat_block_device(&self, name: &str) -> Result<StorageDevice> {
        stat_block_device(name, true)
    }

    fn stat_device_mapper(&self, name: &str) -> Result<DeviceMapper> {
        stat_device_mapper(name, true)
    }

    fn stat_scsi_cdrom(&self, name: &str) -> Result<ScsiCdrom> {
        stat_scsi_cdrom(name, true)
    }

    fn stat_multiple_device_storage(&self, name: &str) -> Result<MultipleDeviceStorage> {
        stat_multiple_device_storage(name, true)
    }

    fn block_size(&self, name: &str) -> Result<i64> {
        block_size(name)
    }

    //
    // mem
    //

    fn memory(&self) -> Result<Memory> {
        memory()
    }

    //
    // ps
    //

    fn stat_process(&self, pid: i32) -> Result<ProcessStat> {
        stat_process(pid)
    }

    fn pids(&self) -> Result<Vec<i32>> {
        pids()
    }

    fn processes(&self) -> Result<Processes> {
        processes()
    }

    //
    // other
    //

    fn kernel_version(&self) -> Result<String> {
        kernel_version()
    }

    fn mounts(&self) -> Result<MountPoints> {
        mounts()
    }

    //
    // cpu
    //

    fn cores(&self) -> Result<Cores> {
        cores()
    }

    fn processor(&self) -> Result<Processor> {
        processor()
    }

    //
    // net
    //

    fn ifaces(&self) -> Result<Interfaces> {
        ifaces()
    }
}
