#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `RECEIVE_BUFFER` reader - 1: full, one or more complete messages are available in the RXFIFO. 0: empty, no message is available"]
pub type ReceiveBufferR = crate::BitReader;
#[doc = "Field `OVERRUN` reader - 1: overrun, a message was lost because there was not enough space for that message in the RXFIFO. 0: absent, no data overrun has occurred since the last clear data overrun command was given"]
pub type OverrunR = crate::BitReader;
#[doc = "Field `TRANSMIT_BUFFER` reader - 1: released, the CPU may write a message into the transmit buffer. 0: locked, the CPU cannot access the transmit buffer, a message is either waiting for transmission or is in the process of being transmitted"]
pub type TransmitBufferR = crate::BitReader;
#[doc = "Field `TRANSMISSION_COMPLETE` reader - 1: complete, last requested transmission has been successfully completed. 0: incomplete, previously requested transmission is not yet completed"]
pub type TransmissionCompleteR = crate::BitReader;
#[doc = "Field `RECEIVE` reader - 1: receive, the TWAI controller is receiving a message. 0: idle"]
pub type ReceiveR = crate::BitReader;
#[doc = "Field `TRANSMIT` reader - 1: transmit, the TWAI controller is transmitting a message. 0: idle"]
pub type TransmitR = crate::BitReader;
#[doc = "Field `ERR` reader - 1: error, at least one of the error counters has reached or exceeded the CPU warning limit defined by the Error Warning Limit Register (EWLR). 0: ok, both error counters are below the warning limit"]
pub type ErrR = crate::BitReader;
#[doc = "Field `NODE_BUS_OFF` reader - 1: bus-off, the TWAI controller is not involved in bus activities. 0: bus-on, the TWAI controller is involved in bus activities"]
pub type NodeBusOffR = crate::BitReader;
#[doc = "Field `MISS` reader - 1: current message is destroyed because of FIFO overflow."]
pub type MissR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1: full, one or more complete messages are available in the RXFIFO. 0: empty, no message is available"]
    #[inline(always)]
    pub fn receive_buffer(&self) -> ReceiveBufferR {
        ReceiveBufferR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: overrun, a message was lost because there was not enough space for that message in the RXFIFO. 0: absent, no data overrun has occurred since the last clear data overrun command was given"]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: released, the CPU may write a message into the transmit buffer. 0: locked, the CPU cannot access the transmit buffer, a message is either waiting for transmission or is in the process of being transmitted"]
    #[inline(always)]
    pub fn transmit_buffer(&self) -> TransmitBufferR {
        TransmitBufferR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: complete, last requested transmission has been successfully completed. 0: incomplete, previously requested transmission is not yet completed"]
    #[inline(always)]
    pub fn transmission_complete(&self) -> TransmissionCompleteR {
        TransmissionCompleteR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: receive, the TWAI controller is receiving a message. 0: idle"]
    #[inline(always)]
    pub fn receive(&self) -> ReceiveR {
        ReceiveR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: transmit, the TWAI controller is transmitting a message. 0: idle"]
    #[inline(always)]
    pub fn transmit(&self) -> TransmitR {
        TransmitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: error, at least one of the error counters has reached or exceeded the CPU warning limit defined by the Error Warning Limit Register (EWLR). 0: ok, both error counters are below the warning limit"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: bus-off, the TWAI controller is not involved in bus activities. 0: bus-on, the TWAI controller is involved in bus activities"]
    #[inline(always)]
    pub fn node_bus_off(&self) -> NodeBusOffR {
        NodeBusOffR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: current message is destroyed because of FIFO overflow."]
    #[inline(always)]
    pub fn miss(&self) -> MissR {
        MissR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "TWAI status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
