#[doc = "Register `FIFO_ST` reader"]
pub type R = crate::R<FifoStSpec>;
#[doc = "Field `RXFIFO_RADDR` reader - Represents the offset address of the APB reading from RXFIFO."]
pub type RxfifoRaddrR = crate::FieldReader;
#[doc = "Field `RXFIFO_WADDR` reader - Represents the offset address of i2c module receiving data and writing to RXFIFO."]
pub type RxfifoWaddrR = crate::FieldReader;
#[doc = "Field `TXFIFO_RADDR` reader - Represents the offset address of i2c module reading from TXFIFO."]
pub type TxfifoRaddrR = crate::FieldReader;
#[doc = "Field `TXFIFO_WADDR` reader - Represents the offset address of APB bus writing to TXFIFO."]
pub type TxfifoWaddrR = crate::FieldReader;
#[doc = "Field `SLAVE_RW_POINT` reader - Represents the offset address in the I2C Slave RAM addressed by I2C Master when in I2C slave mode."]
pub type SlaveRwPointR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Represents the offset address of the APB reading from RXFIFO."]
    #[inline(always)]
    pub fn rxfifo_raddr(&self) -> RxfifoRaddrR {
        RxfifoRaddrR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Represents the offset address of i2c module receiving data and writing to RXFIFO."]
    #[inline(always)]
    pub fn rxfifo_waddr(&self) -> RxfifoWaddrR {
        RxfifoWaddrR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Represents the offset address of i2c module reading from TXFIFO."]
    #[inline(always)]
    pub fn txfifo_raddr(&self) -> TxfifoRaddrR {
        TxfifoRaddrR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Represents the offset address of APB bus writing to TXFIFO."]
    #[inline(always)]
    pub fn txfifo_waddr(&self) -> TxfifoWaddrR {
        TxfifoWaddrR::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 22:29 - Represents the offset address in the I2C Slave RAM addressed by I2C Master when in I2C slave mode."]
    #[inline(always)]
    pub fn slave_rw_point(&self) -> SlaveRwPointR {
        SlaveRwPointR::new(((self.bits >> 22) & 0xff) as u8)
    }
}
#[doc = "FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoStSpec;
impl crate::RegisterSpec for FifoStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_st::R`](R) reader structure"]
impl crate::Readable for FifoStSpec {}
#[doc = "`reset()` method sets FIFO_ST to value 0"]
impl crate::Resettable for FifoStSpec {}
