use cfg_if::cfg_if;

pub struct NetFilter<T> {
    pub inner: T,
}

cfg_if! {
    if #[cfg(bus = "pci")] {
        use driver_pci::{PciRoot, DeviceFunction, DeviceFunctionInfo};
        type VirtIoTransport = driver_virtio::PciTransport;
    } else if #[cfg(bus =  "mmio")] {
        type VirtIoTransport = driver_virtio::MmioTransport;
    }
}

cfg_if! {
    if #[cfg(net_dev = "virtio-net")] {
        use super::prelude::*;
        use driver_common::BaseDriverOps;
        use crate::virtio::VirtIoHalImpl;

        impl BaseDriverOps for NetFilter<driver_virtio::VirtIoNetDev<VirtIoHalImpl, VirtIoTransport, 64>> {
            #[inline]
            fn mac_address(&self) -> driver_net::EthernetAddress {
                self.inner.mac_address()
            }

            #[inline]
            fn can_transmit(&self) -> bool {
                self.inner.can_transmit()
            }

            #[inline]
            fn can_receive(&self) -> bool {
                self.inner.can_receive()
            }

            #[inline]
            fn rx_queue_size(&self) -> usize {
                self.inner.rx_queue_size()
            }

            #[inline]
            fn tx_queue_size(&self) -> usize {
                self.inner.tx_queue_size()
            }

            #[inline]
            fn recycle_rx_buffer(&mut self, rx_buf: driver_net::NetBufPtr) -> driver_common::DevResult {
                self.inner.recycle_rx_buffer(rx_buf)
            }

            #[inline]
            fn recycle_tx_buffers(&mut self) -> driver_common::DevResult {
                self.inner.recycle_tx_buffers()
            }

            #[inline]
            fn transmit(&mut self, tx_buf: driver_net::NetBufPtr) -> driver_common::DevResult {
                log::warn!("Filter: transmit len [{}]", tx_buf.packet_len());
                self.inner.transmit(tx_buf)
            }

            #[inline]
            fn receive(&mut self) -> driver_common::DevResult<driver_net::NetBufPtr> {
                let ret = self.inner.receive()?;
                log::warn!("Filter: receive len[{:?}]", ret.packet_len());
                Ok(ret)
            }

            #[inline]
            fn alloc_tx_buffer(&mut self, size: usize) -> driver_common::DevResult<driver_net::NetBufPtr> {
                self.inner.alloc_tx_buffer(size)
            }
        }
    }
}
