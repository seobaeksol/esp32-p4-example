#[doc = "Register `CORE_ERR_RESP_DIS` reader"]
pub type R = crate::R<CoreErrRespDisSpec>;
#[doc = "Register `CORE_ERR_RESP_DIS` writer"]
pub type W = crate::W<CoreErrRespDisSpec>;
#[doc = "Field `CORE_ERR_RESP_DIS` reader - Set bit0 to disable ibus err resp. Set bit1 to disable dbus err resp. Set bit 2 to disable ahb err resp."]
pub type CoreErrRespDisR = crate::FieldReader;
#[doc = "Field `CORE_ERR_RESP_DIS` writer - Set bit0 to disable ibus err resp. Set bit1 to disable dbus err resp. Set bit 2 to disable ahb err resp."]
pub type CoreErrRespDisW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Set bit0 to disable ibus err resp. Set bit1 to disable dbus err resp. Set bit 2 to disable ahb err resp."]
    #[inline(always)]
    pub fn core_err_resp_dis(&self) -> CoreErrRespDisR {
        CoreErrRespDisR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Set bit0 to disable ibus err resp. Set bit1 to disable dbus err resp. Set bit 2 to disable ahb err resp."]
    #[inline(always)]
    pub fn core_err_resp_dis(&mut self) -> CoreErrRespDisW<'_, CoreErrRespDisSpec> {
        CoreErrRespDisW::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`core_err_resp_dis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_err_resp_dis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoreErrRespDisSpec;
impl crate::RegisterSpec for CoreErrRespDisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_err_resp_dis::R`](R) reader structure"]
impl crate::Readable for CoreErrRespDisSpec {}
#[doc = "`write(|w| ..)` method takes [`core_err_resp_dis::W`](W) writer structure"]
impl crate::Writable for CoreErrRespDisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_ERR_RESP_DIS to value 0"]
impl crate::Resettable for CoreErrRespDisSpec {}
