#[doc = "Register `AMP_CTRL1` reader"]
pub type R = crate::R<AmpCtrl1Spec>;
#[doc = "Register `AMP_CTRL1` writer"]
pub type W = crate::W<AmpCtrl1Spec>;
#[doc = "Field `SAR_AMP_WAIT1` reader - N/A"]
pub type SarAmpWait1R = crate::FieldReader<u16>;
#[doc = "Field `SAR_AMP_WAIT1` writer - N/A"]
pub type SarAmpWait1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SAR_AMP_WAIT2` reader - N/A"]
pub type SarAmpWait2R = crate::FieldReader<u16>;
#[doc = "Field `SAR_AMP_WAIT2` writer - N/A"]
pub type SarAmpWait2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn sar_amp_wait1(&self) -> SarAmpWait1R {
        SarAmpWait1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - N/A"]
    #[inline(always)]
    pub fn sar_amp_wait2(&self) -> SarAmpWait2R {
        SarAmpWait2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn sar_amp_wait1(&mut self) -> SarAmpWait1W<'_, AmpCtrl1Spec> {
        SarAmpWait1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - N/A"]
    #[inline(always)]
    pub fn sar_amp_wait2(&mut self) -> SarAmpWait2W<'_, AmpCtrl1Spec> {
        SarAmpWait2W::new(self, 16)
    }
}
#[doc = "N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`amp_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amp_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmpCtrl1Spec;
impl crate::RegisterSpec for AmpCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`amp_ctrl1::R`](R) reader structure"]
impl crate::Readable for AmpCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`amp_ctrl1::W`](W) writer structure"]
impl crate::Writable for AmpCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AMP_CTRL1 to value 0x000a_000a"]
impl crate::Resettable for AmpCtrl1Spec {
    const RESET_VALUE: u32 = 0x000a_000a;
}
