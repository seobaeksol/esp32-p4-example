#[doc = "Register `CSI_EN` reader"]
pub type R = crate::R<CsiEnSpec>;
#[doc = "Register `CSI_EN` writer"]
pub type W = crate::W<CsiEnSpec>;
#[doc = "Field `CSI_BRIG_EN` reader - 0: disable csi bridge. 1: enable csi bridge."]
pub type CsiBrigEnR = crate::BitReader;
#[doc = "Field `CSI_BRIG_EN` writer - 0: disable csi bridge. 1: enable csi bridge."]
pub type CsiBrigEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSI_BRIG_RST` reader - 0: release csi bridge reset. 1: enable csi bridge reset."]
pub type CsiBrigRstR = crate::BitReader;
#[doc = "Field `CSI_BRIG_RST` writer - 0: release csi bridge reset. 1: enable csi bridge reset."]
pub type CsiBrigRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: disable csi bridge. 1: enable csi bridge."]
    #[inline(always)]
    pub fn csi_brig_en(&self) -> CsiBrigEnR {
        CsiBrigEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: release csi bridge reset. 1: enable csi bridge reset."]
    #[inline(always)]
    pub fn csi_brig_rst(&self) -> CsiBrigRstR {
        CsiBrigRstR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: disable csi bridge. 1: enable csi bridge."]
    #[inline(always)]
    pub fn csi_brig_en(&mut self) -> CsiBrigEnW<'_, CsiEnSpec> {
        CsiBrigEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 0: release csi bridge reset. 1: enable csi bridge reset."]
    #[inline(always)]
    pub fn csi_brig_rst(&mut self) -> CsiBrigRstW<'_, CsiEnSpec> {
        CsiBrigRstW::new(self, 1)
    }
}
#[doc = "csi bridge enable.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsiEnSpec;
impl crate::RegisterSpec for CsiEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi_en::R`](R) reader structure"]
impl crate::Readable for CsiEnSpec {}
#[doc = "`write(|w| ..)` method takes [`csi_en::W`](W) writer structure"]
impl crate::Writable for CsiEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSI_EN to value 0"]
impl crate::Resettable for CsiEnSpec {}
