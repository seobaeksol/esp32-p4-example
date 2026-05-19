#[doc = "Register `ANA_XPD_PAD_GROUP` reader"]
pub type R = crate::R<AnaXpdPadGroupSpec>;
#[doc = "Register `ANA_XPD_PAD_GROUP` writer"]
pub type W = crate::W<AnaXpdPadGroupSpec>;
#[doc = "Field `ANA_REG_XPD_PAD_GROUP` reader - Set 1 to power up pad group"]
pub type AnaRegXpdPadGroupR = crate::FieldReader;
#[doc = "Field `ANA_REG_XPD_PAD_GROUP` writer - Set 1 to power up pad group"]
pub type AnaRegXpdPadGroupW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Set 1 to power up pad group"]
    #[inline(always)]
    pub fn ana_reg_xpd_pad_group(&self) -> AnaRegXpdPadGroupR {
        AnaRegXpdPadGroupR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Set 1 to power up pad group"]
    #[inline(always)]
    pub fn ana_reg_xpd_pad_group(&mut self) -> AnaRegXpdPadGroupW<'_, AnaXpdPadGroupSpec> {
        AnaRegXpdPadGroupW::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_xpd_pad_group::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_xpd_pad_group::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaXpdPadGroupSpec;
impl crate::RegisterSpec for AnaXpdPadGroupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_xpd_pad_group::R`](R) reader structure"]
impl crate::Readable for AnaXpdPadGroupSpec {}
#[doc = "`write(|w| ..)` method takes [`ana_xpd_pad_group::W`](W) writer structure"]
impl crate::Writable for AnaXpdPadGroupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANA_XPD_PAD_GROUP to value 0xff"]
impl crate::Resettable for AnaXpdPadGroupSpec {
    const RESET_VALUE: u32 = 0xff;
}
