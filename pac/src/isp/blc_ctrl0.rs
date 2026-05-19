#[doc = "Register `BLC_CTRL0` reader"]
pub type R = crate::R<BlcCtrl0Spec>;
#[doc = "Register `BLC_CTRL0` writer"]
pub type W = crate::W<BlcCtrl0Spec>;
#[doc = "Field `BLC_R3_STRETCH` reader - this bit configures the stretch feature of bottom right channel. 0: stretch disable, 1: stretch enable"]
pub type BlcR3StretchR = crate::BitReader;
#[doc = "Field `BLC_R3_STRETCH` writer - this bit configures the stretch feature of bottom right channel. 0: stretch disable, 1: stretch enable"]
pub type BlcR3StretchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLC_R2_STRETCH` reader - this bit configures the stretch feature of bottom left channel. 0: stretch disable, 1: stretch enable"]
pub type BlcR2StretchR = crate::BitReader;
#[doc = "Field `BLC_R2_STRETCH` writer - this bit configures the stretch feature of bottom left channel. 0: stretch disable, 1: stretch enable"]
pub type BlcR2StretchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLC_R1_STRETCH` reader - this bit configures the stretch feature of top right channel. 0: stretch disable, 1: stretch enable"]
pub type BlcR1StretchR = crate::BitReader;
#[doc = "Field `BLC_R1_STRETCH` writer - this bit configures the stretch feature of top right channel. 0: stretch disable, 1: stretch enable"]
pub type BlcR1StretchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLC_R0_STRETCH` reader - this bit configures the stretch feature of top left channel. 0: stretch disable, 1: stretch enable"]
pub type BlcR0StretchR = crate::BitReader;
#[doc = "Field `BLC_R0_STRETCH` writer - this bit configures the stretch feature of top left channel. 0: stretch disable, 1: stretch enable"]
pub type BlcR0StretchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - this bit configures the stretch feature of bottom right channel. 0: stretch disable, 1: stretch enable"]
    #[inline(always)]
    pub fn blc_r3_stretch(&self) -> BlcR3StretchR {
        BlcR3StretchR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - this bit configures the stretch feature of bottom left channel. 0: stretch disable, 1: stretch enable"]
    #[inline(always)]
    pub fn blc_r2_stretch(&self) -> BlcR2StretchR {
        BlcR2StretchR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - this bit configures the stretch feature of top right channel. 0: stretch disable, 1: stretch enable"]
    #[inline(always)]
    pub fn blc_r1_stretch(&self) -> BlcR1StretchR {
        BlcR1StretchR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - this bit configures the stretch feature of top left channel. 0: stretch disable, 1: stretch enable"]
    #[inline(always)]
    pub fn blc_r0_stretch(&self) -> BlcR0StretchR {
        BlcR0StretchR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - this bit configures the stretch feature of bottom right channel. 0: stretch disable, 1: stretch enable"]
    #[inline(always)]
    pub fn blc_r3_stretch(&mut self) -> BlcR3StretchW<'_, BlcCtrl0Spec> {
        BlcR3StretchW::new(self, 0)
    }
    #[doc = "Bit 1 - this bit configures the stretch feature of bottom left channel. 0: stretch disable, 1: stretch enable"]
    #[inline(always)]
    pub fn blc_r2_stretch(&mut self) -> BlcR2StretchW<'_, BlcCtrl0Spec> {
        BlcR2StretchW::new(self, 1)
    }
    #[doc = "Bit 2 - this bit configures the stretch feature of top right channel. 0: stretch disable, 1: stretch enable"]
    #[inline(always)]
    pub fn blc_r1_stretch(&mut self) -> BlcR1StretchW<'_, BlcCtrl0Spec> {
        BlcR1StretchW::new(self, 2)
    }
    #[doc = "Bit 3 - this bit configures the stretch feature of top left channel. 0: stretch disable, 1: stretch enable"]
    #[inline(always)]
    pub fn blc_r0_stretch(&mut self) -> BlcR0StretchW<'_, BlcCtrl0Spec> {
        BlcR0StretchW::new(self, 3)
    }
}
#[doc = "blc stretch control register\n\nYou can [`read`](crate::Reg::read) this register and get [`blc_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blc_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlcCtrl0Spec;
impl crate::RegisterSpec for BlcCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blc_ctrl0::R`](R) reader structure"]
impl crate::Readable for BlcCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`blc_ctrl0::W`](W) writer structure"]
impl crate::Writable for BlcCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLC_CTRL0 to value 0"]
impl crate::Resettable for BlcCtrl0Spec {}
