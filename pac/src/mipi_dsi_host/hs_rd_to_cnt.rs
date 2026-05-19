#[doc = "Register `HS_RD_TO_CNT` reader"]
pub type R = crate::R<HsRdToCntSpec>;
#[doc = "Register `HS_RD_TO_CNT` writer"]
pub type W = crate::W<HsRdToCntSpec>;
#[doc = "Field `HS_RD_TO_CNT` reader - NA"]
pub type HsRdToCntR = crate::FieldReader<u16>;
#[doc = "Field `HS_RD_TO_CNT` writer - NA"]
pub type HsRdToCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    pub fn hs_rd_to_cnt(&self) -> HsRdToCntR {
        HsRdToCntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    pub fn hs_rd_to_cnt(&mut self) -> HsRdToCntW<'_, HsRdToCntSpec> {
        HsRdToCntW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hs_rd_to_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hs_rd_to_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsRdToCntSpec;
impl crate::RegisterSpec for HsRdToCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hs_rd_to_cnt::R`](R) reader structure"]
impl crate::Readable for HsRdToCntSpec {}
#[doc = "`write(|w| ..)` method takes [`hs_rd_to_cnt::W`](W) writer structure"]
impl crate::Writable for HsRdToCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HS_RD_TO_CNT to value 0"]
impl crate::Resettable for HsRdToCntSpec {}
