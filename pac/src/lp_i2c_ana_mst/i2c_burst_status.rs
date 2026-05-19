#[doc = "Register `I2C_BURST_STATUS` reader"]
pub type R = crate::R<I2cBurstStatusSpec>;
#[doc = "Register `I2C_BURST_STATUS` writer"]
pub type W = crate::W<I2cBurstStatusSpec>;
#[doc = "Field `I2C_MST_BURST_DONE` reader - need des"]
pub type I2cMstBurstDoneR = crate::BitReader;
#[doc = "Field `I2C_MST0_BURST_ERR_FLAG` reader - need des"]
pub type I2cMst0BurstErrFlagR = crate::BitReader;
#[doc = "Field `I2C_MST1_BURST_ERR_FLAG` reader - need des"]
pub type I2cMst1BurstErrFlagR = crate::BitReader;
#[doc = "Field `I2C_MST_BURST_TIMEOUT_CNT` reader - need des"]
pub type I2cMstBurstTimeoutCntR = crate::FieldReader<u16>;
#[doc = "Field `I2C_MST_BURST_TIMEOUT_CNT` writer - need des"]
pub type I2cMstBurstTimeoutCntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - need des"]
    #[inline(always)]
    pub fn i2c_mst_burst_done(&self) -> I2cMstBurstDoneR {
        I2cMstBurstDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need des"]
    #[inline(always)]
    pub fn i2c_mst0_burst_err_flag(&self) -> I2cMst0BurstErrFlagR {
        I2cMst0BurstErrFlagR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need des"]
    #[inline(always)]
    pub fn i2c_mst1_burst_err_flag(&self) -> I2cMst1BurstErrFlagR {
        I2cMst1BurstErrFlagR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 20:31 - need des"]
    #[inline(always)]
    pub fn i2c_mst_burst_timeout_cnt(&self) -> I2cMstBurstTimeoutCntR {
        I2cMstBurstTimeoutCntR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 20:31 - need des"]
    #[inline(always)]
    pub fn i2c_mst_burst_timeout_cnt(&mut self) -> I2cMstBurstTimeoutCntW<'_, I2cBurstStatusSpec> {
        I2cMstBurstTimeoutCntW::new(self, 20)
    }
}
#[doc = "need des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_burst_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_burst_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cBurstStatusSpec;
impl crate::RegisterSpec for I2cBurstStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_burst_status::R`](R) reader structure"]
impl crate::Readable for I2cBurstStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_burst_status::W`](W) writer structure"]
impl crate::Writable for I2cBurstStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_BURST_STATUS to value 0x4000_0000"]
impl crate::Resettable for I2cBurstStatusSpec {
    const RESET_VALUE: u32 = 0x4000_0000;
}
