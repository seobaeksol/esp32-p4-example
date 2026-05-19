#[doc = "Register `RX_ERR_CNT` reader"]
pub type R = crate::R<RxErrCntSpec>;
#[doc = "Register `RX_ERR_CNT` writer"]
pub type W = crate::W<RxErrCntSpec>;
#[doc = "Field `RX_ERR_CNT` reader - The RX error counter register reflects the current value of the transmit error counter. Software has R/W permission in reset mode and RO in operation mode."]
pub type RxErrCntR = crate::FieldReader;
#[doc = "Field `RX_ERR_CNT` writer - The RX error counter register reflects the current value of the transmit error counter. Software has R/W permission in reset mode and RO in operation mode."]
pub type RxErrCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The RX error counter register reflects the current value of the transmit error counter. Software has R/W permission in reset mode and RO in operation mode."]
    #[inline(always)]
    pub fn rx_err_cnt(&self) -> RxErrCntR {
        RxErrCntR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The RX error counter register reflects the current value of the transmit error counter. Software has R/W permission in reset mode and RO in operation mode."]
    #[inline(always)]
    pub fn rx_err_cnt(&mut self) -> RxErrCntW<'_, RxErrCntSpec> {
        RxErrCntW::new(self, 0)
    }
}
#[doc = "Rx error counter register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_err_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_err_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxErrCntSpec;
impl crate::RegisterSpec for RxErrCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_err_cnt::R`](R) reader structure"]
impl crate::Readable for RxErrCntSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_err_cnt::W`](W) writer structure"]
impl crate::Writable for RxErrCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_ERR_CNT to value 0"]
impl crate::Resettable for RxErrCntSpec {}
