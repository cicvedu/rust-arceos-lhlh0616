#[cfg(not(feature = "net"))]
use crate::virtio::*;
#[cfg(feature = "net")]
use driver_net::*;
use driver_virtio::*;
#[cfg(not(feature = "net"))]
pub type NetFilter<T> = T;
#[cfg(feature = "net")]
pub struct NetFilter<T: NetDriverOps> {
    inner: T,
}

#[cfg(feature = "net")]
impl<T: NetDriverOps> NetFilter<T> {
    pub fn device_name(&self) -> &str {
        self.inner.device_name()
    }

    pub fn device_type(&self) -> DeviceType {
        self.inner.device_type()
    }

    pub fn transmit(&mut self, tx_buf: NetBufPtr) -> DevResult {
        warn!("Filter: transmit len[{}]", tx_buf.packet_len());
        self.inner.transmit(tx_buf)
    }

    pub fn receive(&mut self) -> DevResult<NetBufPtr> {
        let b = self.inner.receive();
        if let Ok(ref v) = b {
            warn!("Filter: receive len[{}]", v.packet_len());
        }
        b
    }

    pub fn mac_address(&self) -> EthernetAddress {
        self.inner.mac_address()
    }

    pub fn can_transmit(&self) -> bool {
        self.inner.can_transmit()
    }

    pub fn can_receive(&self) -> bool {
        self.inner.can_receive()
    }

    pub fn recycle_rx_buffer(&mut self, rx_buf: NetBufPtr) -> DevResult {
        self.inner.recycle_rx_buffer(rx_buf)
    }

    pub fn recycle_tx_buffers(&mut self) -> DevResult {
        self.inner.recycle_tx_buffers()
    }

    pub fn alloc_tx_buffer(&mut self, size: usize) -> DevResult<NetBufPtr> {
        self.inner.alloc_tx_buffer(size)
    }
}
