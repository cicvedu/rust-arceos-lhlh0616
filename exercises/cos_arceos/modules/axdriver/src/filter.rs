use cfg_if::cfg_if;
use log::warn;

use super::prelude::*;
use driver_common::{BaseDriverOps, DevResult, DeviceType};
use driver_pci::{PciRoot, DeviceFunction, DeviceFunctionInfo};
use crate::virtio::VirtIoHalImpl;

pub struct NetFilter<T> {
    pub inner: T,
}

enum VirtIoTransport {
    Pci(driver_virtio::PciTransport),
    Mmio(driver_virtio::MmioTransport),
}

cfg_if! {
    if #[cfg(bus = "pci")] {
        fn get_virtio_transport() -> VirtIoTransport {
            VirtIoTransport::Pci(driver_virtio::PciTransport)
        }
    } else if #[cfg(bus =  "mmio")] {
        fn get_virtio_transport() -> VirtIoTransport {
            VirtIoTransport::Mmio(driver_virtio::MmioTransport)
        }
    }
}

impl<T> NetFilter<T> {
    #[inline]
    fn inner(&self) -> &T {
        &self.inner
    }

    #[inline]
    fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T> BaseDriverOps for NetFilter<driver_virtio::VirtIoNetDev<VirtIoHalImpl, VirtIoTransport, 64>>
where
    T: BaseDriverOps,
{
    fn device_type(&self) -> DeviceType {
        DeviceType::Net
    }

    fn device_name(&self) -> &str {
        "my-net"
    }

    #[inline]
    fn mac_address(&self) -> driver_net::EthernetAddress {
        self.inner().mac_address()
    }

    #[inline]
    fn can_transmit(&self) -> bool {
        self.inner().can_transmit()
    }

    #[inline]
    fn can_receive(&self) -> bool {
        self.inner().can_receive()
    }

    #[inline]
    fn rx_queue_size(&self) -> usize {
        self.inner().rx_queue_size()
    }

    #[inline]
    fn tx_queue_size(&self) -> usize {
        self.inner().tx_queue_size()
    }

    #[inline]
    fn recycle_rx_buffer(&mut self, rx_buf: driver_net::NetBufPtr) -> DevResult {
        self.inner_mut().recycle_rx_buffer(rx_buf)
    }

    #[inline]
    fn recycle_tx_buffers(&mut self) -> DevResult {
        self.inner_mut().recycle_tx_buffers()
    }

    #[inline]
    fn transmit(&mut self, tx_buf: driver_net::NetBufPtr) -> DevResult {
        warn!("Filter: transmit len [{}]", tx_buf.packet_len());
        self.inner_mut().transmit(tx_buf)
    }

    #[inline]
    fn receive(&mut self) -> DevResult<driver_net::NetBufPtr> {
        let ret = self.inner_mut().receive()?;
        warn!("Filter: receive len[{:?}]", ret.packet_len());
        Ok(ret)
    }

    #[inline]
    fn alloc_tx_buffer(&mut self, size: usize) -> DevResult<driver_net::NetBufPtr> {
        self.inner_mut().alloc_tx_buffer(size)
    }
}
