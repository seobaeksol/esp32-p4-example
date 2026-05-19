#[doc = "Register `SWHSDST0` reader"]
pub type R = crate::R<Swhsdst0Spec>;
#[doc = "Register `SWHSDST0` writer"]
pub type W = crate::W<Swhsdst0Spec>;
#[doc = "Field `CH1_SWHS_REQ_DST` reader - NA"]
pub type Ch1SwhsReqDstR = crate::BitReader;
#[doc = "Field `CH1_SWHS_REQ_DST` writer - NA"]
pub type Ch1SwhsReqDstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_SWHS_REQ_DST_WE` writer - NA"]
pub type Ch1SwhsReqDstWeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_SWHS_SGLREQ_DST` reader - NA"]
pub type Ch1SwhsSglreqDstR = crate::BitReader;
#[doc = "Field `CH1_SWHS_SGLREQ_DST` writer - NA"]
pub type Ch1SwhsSglreqDstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_SWHS_SGLREQ_DST_WE` writer - NA"]
pub type Ch1SwhsSglreqDstWeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_SWHS_LST_DST` reader - NA"]
pub type Ch1SwhsLstDstR = crate::BitReader;
#[doc = "Field `CH1_SWHS_LST_DST` writer - NA"]
pub type Ch1SwhsLstDstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_SWHS_LST_DST_WE` writer - NA"]
pub type Ch1SwhsLstDstWeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch1_swhs_req_dst(&self) -> Ch1SwhsReqDstR {
        Ch1SwhsReqDstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn ch1_swhs_sglreq_dst(&self) -> Ch1SwhsSglreqDstR {
        Ch1SwhsSglreqDstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn ch1_swhs_lst_dst(&self) -> Ch1SwhsLstDstR {
        Ch1SwhsLstDstR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch1_swhs_req_dst(&mut self) -> Ch1SwhsReqDstW<'_, Swhsdst0Spec> {
        Ch1SwhsReqDstW::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn ch1_swhs_req_dst_we(&mut self) -> Ch1SwhsReqDstWeW<'_, Swhsdst0Spec> {
        Ch1SwhsReqDstWeW::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn ch1_swhs_sglreq_dst(&mut self) -> Ch1SwhsSglreqDstW<'_, Swhsdst0Spec> {
        Ch1SwhsSglreqDstW::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn ch1_swhs_sglreq_dst_we(&mut self) -> Ch1SwhsSglreqDstWeW<'_, Swhsdst0Spec> {
        Ch1SwhsSglreqDstWeW::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn ch1_swhs_lst_dst(&mut self) -> Ch1SwhsLstDstW<'_, Swhsdst0Spec> {
        Ch1SwhsLstDstW::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn ch1_swhs_lst_dst_we(&mut self) -> Ch1SwhsLstDstWeW<'_, Swhsdst0Spec> {
        Ch1SwhsLstDstWeW::new(self, 5)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`swhsdst0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swhsdst0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swhsdst0Spec;
impl crate::RegisterSpec for Swhsdst0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swhsdst0::R`](R) reader structure"]
impl crate::Readable for Swhsdst0Spec {}
#[doc = "`write(|w| ..)` method takes [`swhsdst0::W`](W) writer structure"]
impl crate::Writable for Swhsdst0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWHSDST0 to value 0"]
impl crate::Resettable for Swhsdst0Spec {}
