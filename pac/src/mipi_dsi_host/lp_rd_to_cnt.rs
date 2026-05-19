#[doc = "Register `LP_RD_TO_CNT` reader"]
pub type R = crate::R<LpRdToCntSpec>;
#[doc = "Register `LP_RD_TO_CNT` writer"]
pub type W = crate::W<LpRdToCntSpec>;
#[doc = "Field `LP_RD_TO_CNT` reader - NA"]
pub type LpRdToCntR = crate::FieldReader<u16>;
#[doc = "Field `LP_RD_TO_CNT` writer - NA"]
pub type LpRdToCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    pub fn lp_rd_to_cnt(&self) -> LpRdToCntR {
        LpRdToCntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    pub fn lp_rd_to_cnt(&mut self) -> LpRdToCntW<'_, LpRdToCntSpec> {
        LpRdToCntW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_rd_to_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_rd_to_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpRdToCntSpec;
impl crate::RegisterSpec for LpRdToCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_rd_to_cnt::R`](R) reader structure"]
impl crate::Readable for LpRdToCntSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_rd_to_cnt::W`](W) writer structure"]
impl crate::Writable for LpRdToCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_RD_TO_CNT to value 0"]
impl crate::Resettable for LpRdToCntSpec {}
