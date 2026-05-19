#[doc = "Register `OP%s_TSTMP_E1` reader"]
pub type R = crate::R<OpTstmpE1Spec>;
#[doc = "Register `OP%s_TSTMP_E1` writer"]
pub type W = crate::W<OpTstmpE1Spec>;
#[doc = "Field `OP_TSTMP_E1` reader - Configures generator%s timer stamp E1 value register"]
pub type OpTstmpE1R = crate::FieldReader<u16>;
#[doc = "Field `OP_TSTMP_E1` writer - Configures generator%s timer stamp E1 value register"]
pub type OpTstmpE1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Configures generator%s timer stamp E1 value register"]
    #[inline(always)]
    pub fn op_tstmp_e1(&self) -> OpTstmpE1R {
        OpTstmpE1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configures generator%s timer stamp E1 value register"]
    #[inline(always)]
    pub fn op_tstmp_e1(&mut self) -> OpTstmpE1W<'_, OpTstmpE1Spec> {
        OpTstmpE1W::new(self, 0)
    }
}
#[doc = "Generator%s timer stamp E1 value register\n\nYou can [`read`](crate::Reg::read) this register and get [`op_tstmp_e1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`op_tstmp_e1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OpTstmpE1Spec;
impl crate::RegisterSpec for OpTstmpE1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`op_tstmp_e1::R`](R) reader structure"]
impl crate::Readable for OpTstmpE1Spec {}
#[doc = "`write(|w| ..)` method takes [`op_tstmp_e1::W`](W) writer structure"]
impl crate::Writable for OpTstmpE1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OP%s_TSTMP_E1 to value 0"]
impl crate::Resettable for OpTstmpE1Spec {}
