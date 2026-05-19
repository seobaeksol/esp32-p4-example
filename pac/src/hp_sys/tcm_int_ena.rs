#[doc = "Register `TCM_INT_ENA` reader"]
pub type R = crate::R<TcmIntEnaSpec>;
#[doc = "Register `TCM_INT_ENA` writer"]
pub type W = crate::W<TcmIntEnaSpec>;
#[doc = "Field `TCM_PARITY_ERR_INT_ENA` reader - need_des"]
pub type TcmParityErrIntEnaR = crate::BitReader;
#[doc = "Field `TCM_PARITY_ERR_INT_ENA` writer - need_des"]
pub type TcmParityErrIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn tcm_parity_err_int_ena(&self) -> TcmParityErrIntEnaR {
        TcmParityErrIntEnaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn tcm_parity_err_int_ena(&mut self) -> TcmParityErrIntEnaW<'_, TcmIntEnaSpec> {
        TcmParityErrIntEnaW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcmIntEnaSpec;
impl crate::RegisterSpec for TcmIntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcm_int_ena::R`](R) reader structure"]
impl crate::Readable for TcmIntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`tcm_int_ena::W`](W) writer structure"]
impl crate::Writable for TcmIntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCM_INT_ENA to value 0"]
impl crate::Resettable for TcmIntEnaSpec {}
