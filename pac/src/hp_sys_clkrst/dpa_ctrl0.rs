#[doc = "Register `DPA_CTRL0` reader"]
pub type R = crate::R<DpaCtrl0Spec>;
#[doc = "Register `DPA_CTRL0` writer"]
pub type W = crate::W<DpaCtrl0Spec>;
#[doc = "Field `SEC_DPA_LEVEL` reader - Reserved"]
pub type SecDpaLevelR = crate::FieldReader;
#[doc = "Field `SEC_DPA_LEVEL` writer - Reserved"]
pub type SecDpaLevelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SEC_DPA_CFG_SEL` reader - Reserved"]
pub type SecDpaCfgSelR = crate::BitReader;
#[doc = "Field `SEC_DPA_CFG_SEL` writer - Reserved"]
pub type SecDpaCfgSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn sec_dpa_level(&self) -> SecDpaLevelR {
        SecDpaLevelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn sec_dpa_cfg_sel(&self) -> SecDpaCfgSelR {
        SecDpaCfgSelR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn sec_dpa_level(&mut self) -> SecDpaLevelW<'_, DpaCtrl0Spec> {
        SecDpaLevelW::new(self, 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn sec_dpa_cfg_sel(&mut self) -> SecDpaCfgSelW<'_, DpaCtrl0Spec> {
        SecDpaCfgSelW::new(self, 2)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`dpa_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpa_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpaCtrl0Spec;
impl crate::RegisterSpec for DpaCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpa_ctrl0::R`](R) reader structure"]
impl crate::Readable for DpaCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`dpa_ctrl0::W`](W) writer structure"]
impl crate::Writable for DpaCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPA_CTRL0 to value 0"]
impl crate::Resettable for DpaCtrl0Spec {}
