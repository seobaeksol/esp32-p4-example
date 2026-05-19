#[doc = "Register `TCM_INT_RAW` reader"]
pub type R = crate::R<TcmIntRawSpec>;
#[doc = "Register `TCM_INT_RAW` writer"]
pub type W = crate::W<TcmIntRawSpec>;
#[doc = "Field `TCM_PARITY_ERR_INT_RAW` reader - need_des"]
pub type TcmParityErrIntRawR = crate::BitReader;
#[doc = "Field `TCM_PARITY_ERR_INT_RAW` writer - need_des"]
pub type TcmParityErrIntRawW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn tcm_parity_err_int_raw(&self) -> TcmParityErrIntRawR {
        TcmParityErrIntRawR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn tcm_parity_err_int_raw(&mut self) -> TcmParityErrIntRawW<'_, TcmIntRawSpec> {
        TcmParityErrIntRawW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcmIntRawSpec;
impl crate::RegisterSpec for TcmIntRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcm_int_raw::R`](R) reader structure"]
impl crate::Readable for TcmIntRawSpec {}
#[doc = "`write(|w| ..)` method takes [`tcm_int_raw::W`](W) writer structure"]
impl crate::Writable for TcmIntRawSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCM_INT_RAW to value 0"]
impl crate::Resettable for TcmIntRawSpec {}
