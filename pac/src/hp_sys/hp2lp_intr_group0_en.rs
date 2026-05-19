#[doc = "Register `HP2LP_INTR_GROUP0_EN` reader"]
pub type R = crate::R<Hp2lpIntrGroup0EnSpec>;
#[doc = "Register `HP2LP_INTR_GROUP0_EN` writer"]
pub type W = crate::W<Hp2lpIntrGroup0EnSpec>;
#[doc = "Field `H2LP_INTR_GROUP0_EN` reader - Set each bit to enable corresponding peripheral interrupt to LP CPU."]
pub type H2lpIntrGroup0EnR = crate::FieldReader<u32>;
#[doc = "Field `H2LP_INTR_GROUP0_EN` writer - Set each bit to enable corresponding peripheral interrupt to LP CPU."]
pub type H2lpIntrGroup0EnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Set each bit to enable corresponding peripheral interrupt to LP CPU."]
    #[inline(always)]
    pub fn h2lp_intr_group0_en(&self) -> H2lpIntrGroup0EnR {
        H2lpIntrGroup0EnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Set each bit to enable corresponding peripheral interrupt to LP CPU."]
    #[inline(always)]
    pub fn h2lp_intr_group0_en(&mut self) -> H2lpIntrGroup0EnW<'_, Hp2lpIntrGroup0EnSpec> {
        H2lpIntrGroup0EnW::new(self, 0)
    }
}
#[doc = "HpP2LP Interrupt Enable Register Group0\n\nYou can [`read`](crate::Reg::read) this register and get [`hp2lp_intr_group0_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp2lp_intr_group0_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hp2lpIntrGroup0EnSpec;
impl crate::RegisterSpec for Hp2lpIntrGroup0EnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp2lp_intr_group0_en::R`](R) reader structure"]
impl crate::Readable for Hp2lpIntrGroup0EnSpec {}
#[doc = "`write(|w| ..)` method takes [`hp2lp_intr_group0_en::W`](W) writer structure"]
impl crate::Writable for Hp2lpIntrGroup0EnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP2LP_INTR_GROUP0_EN to value 0xffff_ffff"]
impl crate::Resettable for Hp2lpIntrGroup0EnSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
